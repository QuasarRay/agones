use cpg_rs::{schema_tools::*,Error,Result};
use std::path::Path;
fn main()->Result<()> {
    let args:Vec<_>=std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("proto")=>{
            let output=args.get(2).ok_or_else(||Error::Invalid("please pass outputDir as first parameter".into()))?;
            println!("{}",NativeCpgSchemaContext.generate_cpg_proto(Path::new(output))?.display());
        }
        Some("json")=>{
            let output=args.get(2).map_or("/tmp/schema.json",String::as_str);
            NativeCpgSchemaContext.export_cpg_schema_json(Path::new(output))?;
            println!("Schema written to: {output}");
        }
        _=>return Err(Error::Invalid("usage: cpg-schema proto <outputDir> | json [output.json]".into())),
    }
    Ok(())
}
