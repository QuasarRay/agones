use cpg_rs::graph::storage::CanRewriteGraphStorage;
fn main()->Result<(),Box<dyn std::error::Error>> {
    let mut args=std::env::args_os().skip(1);
    let operation=args.next().ok_or("usage: cpg-storage <import|export|rewrite> <input> <output> [schema-bootstrap.json]")?;
    let input=args.next().ok_or("missing input storage path")?;let output=args.next().ok_or("missing output storage path")?;
    let bootstrap=args.next();if args.next().is_some() {return Err("unexpected additional argument".into());}
    let (input,output,bootstrap)=(std::path::Path::new(&input),std::path::Path::new(&output),bootstrap.as_deref().map(std::path::Path::new));
    match operation.to_str() {
        Some("import")=>{let count=cpg_rs::graph::CpgContext.import_flatgraph_storage(input,output,bootstrap)?;println!("imported {count} CPG nodes into Grafeo");},
        Some("export"|"rewrite")=>{
            let context=cpg_rs::graph::CpgContext;
            let counts=if operation=="export" {context.export_flatgraph_storage(input,output,bootstrap)?} else {context.rewrite_graph_storage(input,output,bootstrap)?};
            println!("wrote {} node kinds, {} edge columns, {} property columns",counts.node_kinds,counts.edge_kinds,counts.property_kinds);
        },
        _=>return Err("usage: cpg-storage <import|export|rewrite> <input> <output> [schema-bootstrap.json]".into()),
    }
    Ok(())
}
