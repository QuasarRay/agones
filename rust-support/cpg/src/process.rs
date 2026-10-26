//! Synchronous child ownership for source build effects.
//! Waiting, failed waits, read errors and unwinding all retain one child owner.
use std::{io::{self, Read}, process::{Child, Command, ExitStatus, Stdio}};
use crate::{Error, Result};

struct OwnedChild { child: Option<Child> }
impl OwnedChild {
    fn wait(&mut self) -> io::Result<ExitStatus> {
        loop {
            match self.child.as_mut().expect("owned child").wait() {
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Ok(status) => { self.child.take(); return Ok(status); }
                Err(error) => return Err(error),
            }
        }
    }
    fn cancel_and_join(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            loop {
                match child.wait() {
                    Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                    _ => break,
                }
            }
        }
    }
}
impl Drop for OwnedChild {
    fn drop(&mut self) { self.cancel_and_join(); }
}

pub fn status(command: &mut Command, source_program: &str) -> Result<ExitStatus> {
    let child = command.spawn().map_err(|error| Error::process_start(source_program, error))?;
    OwnedChild { child: Some(child) }.wait().map_err(Error::stream_io)
}

/// The source ProcessLogger discards every stdout/stderr line, closes child
/// stdin immediately, and joins both output workers before returning its exit
/// code. Draining the pipes preserves that ownership and child backpressure;
/// redirecting the child directly to a null device would bypass those effects.
pub fn discard_logged_output(command: &mut Command, source_program: &str) -> Result<ExitStatus> {
    command.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
    let child = command.spawn().map_err(|error| Error::process_start(source_program, error))?;
    let mut owner = OwnedChild { child: Some(child) };
    let output = owner.child.as_mut().and_then(|child| child.stdout.take())
        .ok_or_else(|| Error::Runtime("owned logger stdout was not created".into()))?;
    let errors = owner.child.as_mut().and_then(|child| child.stderr.take())
        .ok_or_else(|| Error::Runtime("owned logger stderr was not created".into()))?;
    std::thread::scope(move |scope| {
        // This local drops before scope joins during unwinding, so a failed
        // worker spawn cannot strand a live child behind the join boundary.
        let mut owner = owner;
        let output = scope.spawn(|| discard_lines(output));
        let errors = scope.spawn(|| discard_lines(errors));
        let status = owner.wait().map_err(Error::stream_io);
        if status.is_err() { owner.cancel_and_join(); }
        // Scala's SimpleProcess joins the workers but does not turn an output
        // worker's IOException into the child's exit status.
        for worker in [output, errors] {
            match worker.join() {
                Ok(Err(error)) => eprintln!("{error}"),
                Err(_) => eprintln!("native process logger worker panicked"),
                Ok(Ok(())) => (),
            }
        }
        status
    })
}

fn discard_lines(mut input: impl Read) -> Result<()> {
    let mut bytes = [0; 8192];
    loop {
        let count = match input.read(&mut bytes) {
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            result => result.map_err(Error::stream_io)?,
        };
        if count == 0 { return Ok(()); }
        // The callback's body is (). InputStreamReader's replacement decoding
        // and splitting into lines have no retained output for this callback.
        // A bounded byte drain avoids inventing observable log messages.
    }
}

pub fn stdout(command: &mut Command, source_program: &str) -> Result<(ExitStatus, Vec<u8>)> {
    command.stdout(Stdio::piped());
    let child = command.spawn().map_err(|error| Error::process_start(source_program, error))?;
    let mut owner = OwnedChild { child: Some(child) };
    let mut pipe = owner.child.as_mut().and_then(|child| child.stdout.take())
        .ok_or_else(|| Error::Runtime("owned stdout pipe was not created".into()))?;
    let mut bytes = Vec::new();
    let mut buffer = [0; 8192];
    loop {
        let count = match pipe.read(&mut buffer) {
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            result => result.map_err(Error::stream_io)?,
        };
        if count == 0 { break; }
        bytes.try_reserve(count).map_err(|_| Error::out_of_memory())?;
        bytes.extend_from_slice(&buffer[..count]);
    }
    let status = owner.wait().map_err(Error::stream_io)?;
    Ok((status, bytes))
}
