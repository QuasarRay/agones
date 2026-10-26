//! Source build contracts and literal CPG module registration.
//! The retained cpg-rs library remains the single Grafeo graph/persistence owner.
pub mod build_contracts;
pub mod declarations;
pub mod lifecycle;
pub mod source_io;

#[path = "../../../cpg-rs/project/FileUtils.rs"]
#[allow(non_snake_case)]
pub mod FileUtils;
#[path = "../../../cpg-rs/project/Projects.rs"]
#[allow(non_snake_case)]
pub mod Projects;
#[path = "../../../cpg-rs/project/Utils.rs"]
#[allow(non_snake_case)]
pub mod Utils;
#[path = "../../../cpg-rs/project/Versions.rs"]
#[allow(non_snake_case)]
pub mod Versions;
#[path = "../../../cpg-rs/project/project/Versions.rs"]
#[allow(non_snake_case)]
pub mod ProjectVersions;

pub use cpg_rs::{Error, Result};
pub use lifecycle::NativeBuildContext;
