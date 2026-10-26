//! Native build-task entrypoint; execution is deferred by the global gate.
use std::path::PathBuf;
use cpg_native_support::{NativeBuildContext, Error, Result};
use cpg_native_support::build_contracts::{CanRunProtobufBuildTask, SchemaBuildPaths};
use cpg_native_support::stages::{BuildRequest, CanRunNativeBuildStage, NativeBuildOperation, NativeFacility};

fn run() -> Result<()> {
    let mut arguments = std::env::args_os().skip(1);
    let operation = arguments.next().ok_or_else(|| Error::IllegalArgument(
        "usage: cpg-build <load|generate-protobuf|task|facility|source-task> <pinned-source-root> [operation request.json]".into()
    ))?;
    let root = arguments.next().map(PathBuf::from).ok_or_else(|| Error::IllegalArgument(
        "missing pinned CPG source root".into()
    ))?;
    let context = NativeBuildContext::new(&root)?;
    match operation.to_str() {
        Some("load") => {
            if arguments.next().is_some() { return Err(Error::IllegalArgument("unexpected load argument".into())); }
            let (configuration, ()) = context.load_with_previous((), Ok)?;
            for name in configuration.project_names { println!("{name}"); }
        }
        Some("generate-protobuf") => {
            if arguments.next().is_some() { return Err(Error::IllegalArgument("unexpected protobuf argument".into())); }
            context.load_with_previous((), Ok)?;
            let output = context.run_protobuf_build_task(&SchemaBuildPaths::at(context.build_io.project_root()))?;
            println!("{}", output.display());
        }
        Some("task" | "facility" | "source-task") => {
            let name = arguments.next().ok_or_else(|| Error::IllegalArgument("missing native build operation".into()))?;
            let name = name.to_str().ok_or_else(|| Error::IllegalArgument("build operation is not UTF-8".into()))?;
            let request_path = arguments.next().ok_or_else(|| Error::IllegalArgument("missing build request JSON".into()))?;
            if arguments.next().is_some() { return Err(Error::IllegalArgument("unexpected task argument".into())); }
            let request: BuildRequest = serde_json::from_slice(&std::fs::read(request_path)?)?;
            let artifact = match operation.to_str() {
                Some("facility") => context.run_native_build_stage(NativeFacility::parse(name)?.operation(), &request)?,
                Some("source-task") => context.run_source_task_with_previous(name, &request, (), Ok)?.0,
                _ => context.run_native_build_stage(NativeBuildOperation::parse(name)?, &request)?,
            };
            println!("{}", serde_json::to_string(&artifact)?);
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
