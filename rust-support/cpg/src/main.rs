//! Native build-task entrypoint; execution is deferred by the global gate.
use std::path::PathBuf;
use cpg_native_support::{NativeBuildContext, Error, Result};
use cpg_native_support::build_contracts::{CanLoadBuildConfiguration, CanRunProtobufBuildTask, SchemaBuildPaths};

fn run() -> Result<()> {
    let mut arguments = std::env::args_os().skip(1);
    let operation = arguments.next().ok_or_else(|| Error::IllegalArgument(
        "usage: cpg-build <load|generate-protobuf> <pinned-source-root>".into()
    ))?;
    let root = arguments.next().map(PathBuf::from).ok_or_else(|| Error::IllegalArgument(
        "missing pinned CPG source root".into()
    ))?;
    if arguments.next().is_some() {
        return Err(Error::IllegalArgument("unexpected build-task argument".into()));
    }
    let context = NativeBuildContext::new(&root);
    match operation.to_str() {
        Some("load") => {
            let configuration = context.load_build_configuration()?;
            for name in configuration.project_names { println!("{name}"); }
        }
        Some("generate-protobuf") => {
            let output = context.run_protobuf_build_task(&SchemaBuildPaths::at(&root))?;
            println!("{}", output.display());
        }
        _ => return Err(Error::IllegalArgument("unknown native build operation".into())),
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
