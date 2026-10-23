use cpg_rs::codegen::{CanGenerateDomain, CpgToolContext};
fn main() -> cpg_rs::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err(cpg_rs::Error::Invalid("usage: cpg-domain <complete-schema.json> <owned-output-directory>".into()));
    }
    let report = CpgToolContext.generate_domain(std::path::Path::new(&args[1]), std::path::Path::new(&args[2]))?;
    println!("{}", serde_json::json!({"concreteTypes":report.concrete_types,"abstractTraits":report.abstract_traits,
        "edgeTypes":report.edge_types,"properties":report.properties,"outputSha256":report.output_sha256}));
    Ok(())
}
