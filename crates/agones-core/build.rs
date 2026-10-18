use std::{env, fs, path::Path};

use genco::{lang::rust, prelude::*, tokens::quoted};

#[path = "src/go_utf8.rs"]
mod go_utf8;
#[path = "src/version_annotation.rs"]
mod version_annotation;

fn write_changed(path: &Path, bytes: &[u8]) {
    if fs::read(path).ok().as_deref() != Some(bytes) {
        fs::write(path, bytes).expect("write build-version data in OUT_DIR");
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed=AGONES_BUILD_VERSION");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/version_annotation.rs");
    println!("cargo:rerun-if-changed=src/go_utf8.rs");
    // Equivalent to the source build's -X agones.dev/agones/pkg.Version=... .
    // Preserve explicit empty values and raw OS bytes on the pinned Linux host.
    let configured = env::var_os("AGONES_BUILD_VERSION");
    let out = env::var_os("OUT_DIR").expect("OUT_DIR");
    let out = Path::new(&out);
    // Cargo's environment fingerprint can conflate absent/non-UTF-8 values.
    // Recheck those profiles every invocation, including outside this workspace.
    // The workspace's ordinary UTF-8 "dev" default remains normally cacheable.
    if configured
        .as_ref()
        .and_then(|value| value.to_str())
        .is_none()
    {
        println!(
            "cargo:rerun-if-changed={}",
            out.join("recheck-raw-version-input").display()
        );
    }
    let bytes = configured
        .map(|value| value.into_encoded_bytes())
        .unwrap_or_else(|| b"dev".to_vec());
    write_changed(&out.join("agones-build-version.bin"), &bytes);
    let annotation = version_annotation::version_annotation_text(&bytes);
    let tokens: rust::Tokens = quote! {
        pub const AGONES_BUILD_VERSION: &str = $(quoted(annotation.as_str()));
    };
    let generated = tokens
        .to_file_string()
        .expect("format build-version constant");
    write_changed(&out.join("agones-build-version.rs"), generated.as_bytes());
}
