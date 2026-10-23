use cpg_rs::{archive::{SerializedCpg,ZipArchive},proto::{CpgStruct,CpgOverlay},proto_loading::compare_overlay_path};
use protobuf::Message;
use std::{collections::BTreeMap,path::{Path,PathBuf}};

fn evidence(name:&str)->PathBuf {Path::new(env!("CARGO_MANIFEST_DIR")).join("../verification/agones").join(name)}
fn reference()->serde_json::Value {serde_json::from_slice(&std::fs::read(evidence("first-generation-proto-reference.json")).expect("original CPG archive observation must exist")).unwrap()}

#[test]
fn archive_walk_and_normalized_entry_data_match_original_java_zip_filesystem() {
    let expected=reference();
    let mut archive=ZipArchive::open(&evidence("first-generation-zip-reference-input.zip")).unwrap();
    let names=archive.entries().unwrap();
    assert_eq!(serde_json::to_value(&names).unwrap(),expected["archiveEntries"]);
    for name in &names {
        let bytes=archive.read(name).unwrap();let hex:String=bytes.iter().map(|byte|format!("{byte:02x}")).collect();
        assert_eq!(hex,expected["archiveContents"][name]);
    }
    assert_eq!(archive.read("/nested/../top.txt").unwrap(),b"overwritten");
    assert!(archive.read("/nested").is_err());
    archive.close().unwrap();archive.close().unwrap();assert!(archive.entries().is_err());
}

#[test]
fn native_serialization_matches_original_overlay_payloads_and_overwrite_counter_behavior() {
    let expected=reference();let directory=tempfile::tempdir().unwrap();let path=directory.path().join("cpg.zip");
    let input=CpgStruct::parse_from_bytes(&std::fs::read(evidence("first-generation-proto-reference-duplicate-types.pb")).unwrap()).unwrap();
    let mut a=CpgOverlay::new();a.node.push(input.node.iter().find(|node|node.key==3).unwrap().clone());
    let mut b=CpgOverlay::new();b.node.push(input.node.iter().find(|node|node.key==4).unwrap().clone());
    let mut dummy=SerializedCpg::empty();dummy.add_overlay(&a,"ignored").unwrap();
    assert_eq!(dummy.is_empty(),expected["dummyEmpty"].as_bool().unwrap());dummy.close().unwrap();
    let mut writer=SerializedCpg::open(&path).unwrap();
    assert_eq!(path.exists(),expected["createdAtOpen"].as_bool().unwrap());
    writer.add_overlay(&a,"one").unwrap();writer.add_overlays([a,b.clone()],"batch").unwrap();writer.close().unwrap();
    assert_eq!(writer.is_empty(),expected["emptyAfterClose"].as_bool().unwrap());
    assert!(writer.add_overlay(&b,"closed").is_err());
    let mut reopened=SerializedCpg::open(&path).unwrap();reopened.add_overlay(&b,"one").unwrap();reopened.close().unwrap();
    fn payloads(path:&Path)->BTreeMap<String,Vec<u8>> {
        let mut zip=ZipArchive::open(path).unwrap();zip.entries().unwrap().into_iter().map(|name|{let data=zip.read(&name).unwrap();(name,data)}).collect()
    }
    assert_eq!(payloads(&path),payloads(&evidence("first-generation-serialized-reference.zip")));
}

#[test]
fn failed_overlay_open_consumes_counter_and_existing_entries_are_retained() {
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("existing.zip");
    std::fs::copy(evidence("first-generation-zip-reference-input.zip"),&path).unwrap();
    let mut writer=SerializedCpg::open(&path).unwrap();let overlay=CpgOverlay::new();
    assert!(writer.add_overlay(&overlay,"missing/child").is_err());
    writer.add_overlay(&overlay,"valid").unwrap();writer.close().unwrap();
    let mut archive=ZipArchive::open(&path).unwrap();assert!(archive.entries().unwrap().contains(&"/1_valid".into()));
    assert_eq!(archive.read("/top.txt").unwrap(),b"overwritten");
    assert_eq!(archive.read("/nested/deeper/item").unwrap(),b"deep");
}

#[test]
fn overlay_path_comparison_matches_the_original_private_comparator() {
    for case in reference()["comparisons"].as_array().unwrap() {
        let result=compare_overlay_path(case["left"].as_str().unwrap(),case["right"].as_str().unwrap());
        if case.get("errorClass").is_some() {assert!(result.is_err(),"{case}");}
        else {
            let actual=match result.unwrap(){std::cmp::Ordering::Less=>-1,std::cmp::Ordering::Equal=>0,std::cmp::Ordering::Greater=>1};
            assert_eq!(actual,case["comparison"].as_i64().unwrap(),"{case}");
        }
    }
}

#[test]
fn archive_mount_rejects_dot_entries_and_dummy_writer_consumes_iterator_without_encoding() {
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("invalid.zip");
    let file=std::fs::File::create(&path).unwrap();let mut zip=zip::ZipWriter::new(file);
    zip.start_file("dir/../file",zip::write::SimpleFileOptions::default()).unwrap();zip.finish().unwrap();
    assert!(ZipArchive::open(&path).is_err());
    let count=std::cell::Cell::new(0);let iter=(0..3).map(|_|{count.set(count.get()+1);CpgOverlay::new()});
    SerializedCpg::empty().add_overlays(iter,"ignored").unwrap();assert_eq!(count.get(),3);
}

#[test]
fn integer_prefix_parser_matches_all_bmp_code_units_and_original_overflow_limits() {
    use sha2::{Digest,Sha256};
    let reference:serde_json::Value=serde_json::from_slice(&std::fs::read(evidence("first-generation-integer-prefix-reference.json")).unwrap()).unwrap();
    let mut digest=Sha256::new();
    for unit in 0..=u16::MAX {
        match cpg_rs::proto_loading::parse_overlay_integer_units(&[b'1' as u16,unit,b'2' as u16]) {
            Some(value)=>{digest.update([1]);digest.update(value.to_be_bytes());},None=>digest.update([0]),
        }
    }
    let hash:String=digest.finalize().iter().map(|byte|format!("{byte:02x}")).collect();
    assert_eq!(hash,reference["outputSha256"]);
    for (text,expected) in [("-2147483648",Some(i32::MIN)),("2147483647",Some(i32::MAX)),("2147483648",None),
        ("-2147483649",None),("+",None),("-",None),("",None),(" 1",None),("1 ",None),("١٢٣",Some(123))] {
        assert_eq!(cpg_rs::proto_loading::parse_overlay_integer_units(&text.encode_utf16().collect::<Vec<_>>()),expected,"{text}");
    }
}

#[test]
fn overlay_loading_sorts_source_counters_and_returns_owned_results_after_archive_close() {
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("overlays.zip");
    std::fs::copy(evidence("first-generation-serialized-reference.zip"),&path).unwrap();
    let overlays=cpg_rs::proto_loading::load_overlays(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(overlays.map(|overlay|overlay.node[0].key).collect::<Vec<_>>(),[4,3,4]);
    let input=evidence("first-generation-serialized-reference.zip");
    let decoded=cpg_rs::proto_loading::load_overlays_with(&input,|bytes|Ok(CpgOverlay::parse_from_bytes(bytes)?)).unwrap();
    assert_eq!(decoded.count(),3);
}

#[test]
fn serialized_archive_mount_ownership_rejects_a_second_writer_and_releases_on_close() {
    let directory=tempfile::tempdir().unwrap();let path=directory.path().join("cpg.zip");
    let mut first=SerializedCpg::open(&path).unwrap();
    let failure=match SerializedCpg::open(&path) {Err(error)=>error,Ok(_)=>panic!("original URI mount must reject duplicate writer")};
    assert_eq!(failure.source_exception_class(),Some("java.nio.file.FileSystemAlreadyExistsException"));
    let mut reader=ZipArchive::open(&path).unwrap();assert!(reader.entries().unwrap().is_empty());reader.close().unwrap();
    first.close().unwrap();
    SerializedCpg::open(&path).unwrap().close().unwrap();
}
