use cpg_rs::{construction::CanGenerateSchemaConstructors,codegen::CpgToolContext};
fn main()->cpg_rs::Result<()> {
    let args:Vec<_>=std::env::args().collect();
    if args.len()!=3 {return Err(cpg_rs::Error::Invalid("usage: cpg-schema-constructors <source-construction-trace.json> <owned-output-directory>".into()));}
    let report=CpgToolContext.generate_schema_constructors(std::path::Path::new(&args[1]),std::path::Path::new(&args[2]))?;
    println!("{}",serde_json::json!({"mutations":report.emitted_mutations,"domainEvents":report.domain_events,
        "protobufEvents":report.protobuf_events,"outputSha256":report.output_sha256}));
    Ok(())
}
