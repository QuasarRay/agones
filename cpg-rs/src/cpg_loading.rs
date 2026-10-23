//! Source-compatible format probes shared by the CPG loading/conversion lifecycle.
use std::{io::Read,path::Path};

pub fn probe_first_bytes(path:&Path,probe_for:&str)->bool {
    let read=(||->std::io::Result<bool> {
        let mut file=std::fs::File::open(path)?;
        // The original allocates probeFor.size UTF-16 units, then decodes the bytes.
        let mut buffer=vec![0;probe_for.encode_utf16().count()];
        let _read=file.read(&mut buffer)?;
        Ok(crate::utilities::decode_utf8_replacing(&buffer)==probe_for)
    })();
    read.unwrap_or(false)
}
pub fn is_proto_format(path:impl AsRef<Path>)->bool {probe_first_bytes(path.as_ref(),"PK")}
pub fn is_overflow_db_format(path:impl AsRef<Path>)->bool {probe_first_bytes(path.as_ref(),"H:2")}
pub fn is_flatgraph_format(path:impl AsRef<Path>)->bool {probe_first_bytes(path.as_ref(),"FLT GRPH")}
#[deprecated(note="use is_proto_format")]
pub fn is_legacy_cpg(path:impl AsRef<Path>)->bool {is_proto_format(path)}
