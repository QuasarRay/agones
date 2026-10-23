use cpg_rs::{protobuf_bindings::CanGenerateCpgBindings,schema_tools::NativeCpgSchemaContext};
fn main()->Result<(),Box<dyn std::error::Error>> {
    let mut args=std::env::args_os().skip(1);
    let output=args.next().ok_or("usage: cpg-protobuf-bindings <owned-output-directory>")?;
    if args.next().is_some() {return Err("unexpected additional argument".into());}
    let report=NativeCpgSchemaContext.generate_cpg_bindings(std::path::Path::new(&output))?;
    println!("generated {} protocol files with {} source mappings",report.files.len(),report.mappings.len());
    Ok(())
}
