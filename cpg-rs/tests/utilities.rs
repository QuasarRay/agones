use cpg_rs::{execution::{CanRunInExecutionContext, NativeCpgRuntimeContext, ParallelIteratorExecutor}, utilities::*};
use sha2::{Digest, Sha256};
use std::{path::Path, sync::{Arc, atomic::{AtomicUsize, Ordering}}};

#[test]
fn native_utf8_replacement_matches_the_pinned_jdk_for_all_byte_pairs_and_boundary_sequences() {
    let inputs = include_bytes!("../../verification/agones/first-generation-utf8-inputs.bin");
    let reference: serde_json::Value = serde_json::from_str(include_str!("../../verification/agones/first-generation-utf8-reference.json")).unwrap();
    assert_eq!(cpg_rs::content_digest(inputs), reference["input_sha256"]);
    let mut index = 0;
    let mut count = 0u64;
    let mut digest = Sha256::new();
    while index < inputs.len() {
        let len = inputs[index] as usize;
        index += 1;
        let decoded: Vec<_> = decode_utf8_replacing(&inputs[index..index + len]).encode_utf16().collect();
        digest.update((decoded.len() as u32).to_be_bytes());
        for unit in decoded { digest.update(unit.to_be_bytes()); }
        index += len;
        count += 1;
    }
    let hash: String = digest.finalize().iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(count, reference["cases"].as_u64().unwrap());
    assert_eq!(hash, reference["utf16_output_sha256"]);
    assert_eq!(decode_utf8_replacing(&[0xed, 0xa0, 0x80]), "\u{fffd}");
}

#[test]
fn file_reading_retains_java_bom_cr_lf_and_empty_line_semantics() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("input");
    for (bytes, lines, entire) in [
        (&b""[..], vec![], ""),
        (&b"\xef\xbb\xbf"[..], vec![], ""),
        (&b"\n\r\n\r"[..], vec!["", "", ""], "\n\r\n\r"),
        (&b"\xef\xbb\xbfA\r\nB\rC\n\n"[..], vec!["A", "B", "C", ""], "A\r\nB\rC\n\n"),
        (&b"a\xff\xed\xa0\x80\n"[..], vec!["a\u{fffd}\u{fffd}"], "a\u{fffd}\u{fffd}\n"),
    ] {
        std::fs::write(&path, bytes).unwrap();
        assert_eq!(read_lines_in_file(&path).unwrap(), lines);
        assert_eq!(read_entire_file(&path).unwrap(), entire);
    }
    let text = "\u{feff}\u{feff}x\u{2028}y";
    std::fs::write(&path, text).unwrap();
    assert_eq!(read_lines_in_file(&path).unwrap(), ["\u{feff}x\u{2028}y"]);
    assert!(read_entire_file(&directory.path().join("absent")).is_err());
}

#[test]
fn interning_reuses_identity_only_below_the_strict_utf16_length_limit() {
    let mut interner = StrongStringInterner::new(3, 2).unwrap();
    let first: Arc<str> = Arc::from("😀"); // 2 UTF-16 code units, 4 UTF-8 bytes.
    let second: Arc<str> = Arc::from("😀");
    assert!(!Arc::ptr_eq(&first, &second));
    assert!(Arc::ptr_eq(&interner.intern(first.clone()), &interner.intern(second)));
    let long_first: Arc<str> = Arc::from("😀x");
    let long_second: Arc<str> = Arc::from("😀x");
    assert!(!Arc::ptr_eq(&interner.intern(long_first), &interner.intern(long_second)));
    assert_eq!(interner.cached_strings(), 1);
    assert!(interner.intern_nullable(None).is_err());
    let mut noop = NoopStringInterner;
    assert!(Arc::ptr_eq(&first, &noop.intern(first.clone())));
    assert_eq!(noop.intern_nullable(None).unwrap(), None);
    let mut disabled = StrongStringInterner::new(-1, 0).unwrap();
    disabled.intern(first);
    assert_eq!(disabled.cached_strings(), 0);
    assert!(StrongStringInterner::new(3, -1).is_err());
}

#[test]
fn temporary_copy_keeps_source_bytes_and_deletes_on_explicit_close_or_drop() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source");
    let data = b"\0\xffnative CPG bytes";
    std::fs::write(&source, data).unwrap();
    let mut copy = TempFileCopy::with_options(&source, "cpg", ".copy", Some(directory.path())).unwrap();
    let path = copy.path().to_owned();
    assert_eq!(std::fs::read(&path).unwrap(), data);
    assert!(path.file_name().unwrap().to_str().unwrap().ends_with(".copy"));
    assert!(copy.close().unwrap());
    assert!(!copy.close().unwrap());
    assert_eq!(std::fs::read(&source).unwrap(), data);
    let other = TempFileCopy::new(&source).unwrap();
    let path = other.path().to_owned();
    drop(other);
    assert!(!path.exists());
    assert!(TempFileCopy::with_options(&source, "x", ".tmp", None).is_err());
    assert!(TempFileCopy::new(&directory.path().join("missing")).is_err());
}

#[test]
fn project_root_search_checks_zero_through_four_ancestors_and_retains_literal_prefixes() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("marker"), "").unwrap();
    let resolver = ProjectRoot::new("marker".into());
    for depth in 0..=5 {
        let path = directory.path().join(std::iter::repeat_n("sub", depth).collect::<std::path::PathBuf>());
        std::fs::create_dir_all(&path).unwrap();
        if depth <= SEARCH_DEPTH { assert_eq!(resolver.find_relative_from(&path).unwrap(), format!("./{}", "../".repeat(depth))); }
        else { assert_eq!(resolver.find_relative_from(&path), Err(SearchDepthExceededError)); }
    }
    assert!(ProjectRoot::new("does-not-exist".into()).find_relative_from(directory.path()).is_err());
    assert_eq!(ProjectRoot::new("AGENTS.MD".into()).relativise("/absolute".as_ref()).unwrap(), ".//absolute");
    assert_eq!(ProjectRoot::new("AGENTS.MD".into()).find().unwrap(), Path::new("./"));
}

#[test]
fn only_checked_preserves_reusable_collection_and_one_shot_cursor_behavior() {
    let mut warnings = Vec::new();
    let mut once = OnceIterator::new([4].into_iter());
    assert_eq!(only_checked_with_warning(&mut once, |s| warnings.push(s.to_owned())).unwrap(), 4);
    assert!(warnings.is_empty());
    assert!(only_checked(&mut once).is_err());
    let mut reusable = ReusableIterable([4].into_iter());
    assert_eq!(only_checked_with_warning(&mut reusable, |s| warnings.push(s.to_owned())).unwrap(), 4);
    assert_eq!(warnings.len(), 1); // The source re-creates the collection iterator before probing again.
    let mut once = OnceIterator::new([4, 5].into_iter());
    assert_eq!(only_checked_with_warning(&mut once, |s| warnings.push(s.to_owned())).unwrap(), 4);
    assert_eq!(once.0.next(), Some(5));
    assert_eq!(warnings.len(), 2);
}

#[test]
fn execution_inherits_the_callers_pool_and_returns_completed_results_in_input_order() {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(2).thread_name(|i| format!("cpg-owned-{i}")).build().unwrap();
    let context = NativeCpgRuntimeContext;
    let completed = AtomicUsize::new(0);
    pool.install(|| {
        let result = ParallelIteratorExecutor::new(0..64).map(&context, |value| {
            assert_eq!(rayon::current_num_threads(), 2);
            assert!(std::thread::current().name().unwrap().starts_with("cpg-owned-"));
            completed.fetch_add(1, Ordering::SeqCst);
            value * value
        });
        assert_eq!(completed.load(Ordering::SeqCst), 64);
        assert_eq!(result.collect::<Vec<_>>(), (0..64).map(|value| value * value).collect::<Vec<_>>());
        assert_eq!(context.with_execution_context(|| rayon::current_num_threads()), 2);
    });
    let empty = ParallelIteratorExecutor::new(std::iter::empty::<u8>()).map(&context, |_| panic!("empty map callback"));
    assert_eq!(empty.count(), 0);
    let failure = std::panic::catch_unwind(|| {
        ParallelIteratorExecutor::new([1, 2]).map(&context, |value| { if value == 2 { panic!("native map failure") } value }).count()
    });
    assert!(failure.is_err());
}
