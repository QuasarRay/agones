//! Per-response trailer data for the Agones source HTTP profile.
use bytes::Bytes;
use std::sync::{Arc, Mutex};

/// An owned, single-publication HTTP/1 trailer block.
///
/// The source response writer attaches this value to its final response head,
/// publishes its encoded trailer fields, and then yields a trailers body frame.
/// The connection consumes these bytes only at that frame. This preserves the
/// source header spelling, ordering and value normalization without applying
/// Hyper's distinct TE/declaration policy. Other responses keep that policy.
#[derive(Clone, Debug, Default)]
pub struct SourceTrailers(Arc<Mutex<Block>>);

#[derive(Debug, Default)]
enum Block {
    #[default]
    Pending,
    Ready(Bytes),
    Taken,
}

impl SourceTrailers {
    /// Creates the trailer publication owned by one response writer.
    pub fn new() -> Self { Self::default() }

    /// Publishes fields already encoded with the source's Header.Write rules.
    ///
    /// `fields` contains header lines only, without the terminating chunk or
    /// final blank line. Publish once, before yielding the trailers body frame.
    /// A second publication is an error, including after transport consumption.
    pub fn finish_http1(&self, fields: Bytes) -> Result<(), std::io::Error> {
        let mut block = self.0.lock().unwrap_or_else(|error| error.into_inner());
        if !matches!(*block, Block::Pending) {
            return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "source HTTP trailers already published"));
        }
        *block = Block::Ready(fields);
        Ok(())
    }

    pub(crate) fn take_http1(&self) -> Option<Bytes> {
        let mut block = self.0.lock().unwrap_or_else(|error| error.into_inner());
        match std::mem::replace(&mut *block, Block::Taken) {
            Block::Ready(fields) => Some(fields),
            Block::Pending | Block::Taken => None,
        }
    }
}
