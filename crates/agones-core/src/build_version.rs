//! Source build identity and its metadata observation, composed through CGP.
//!
//! The source defaults pkg.Version to "dev" and may replace it at build time.
//! Cargo's package release version is a separate identity. AGONES_BUILD_VERSION
//! supplies the source build value; it is frozen into the normal Cargo build.
use cgp::prelude::*;
use derive_deftly::Deftly;

#[path = "version_annotation.rs"]
mod annotation_text;

include!(concat!(env!("OUT_DIR"), "/agones-build-version.rs"));
pub const AGONES_BUILD_VERSION_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/agones-build-version.bin"));

/// One owner for the original Go string bytes, before JSON normalization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgonesBuildVersion(Vec<u8>);

impl AgonesBuildVersion {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self(bytes.into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Default for AgonesBuildVersion {
    fn default() -> Self {
        Self(AGONES_BUILD_VERSION_BYTES.to_vec())
    }
}

#[cgp_getter]
pub trait HasAgonesBuildVersion {
    fn agones_build_version(&self) -> &AgonesBuildVersion;
}

#[cgp_component(AgonesVersionAnnotationGetter)]
pub trait CanGetAgonesVersionAnnotation {
    fn agones_version_annotation(&self) -> String;
}

#[cgp_new_provider]
impl<Context: HasAgonesBuildVersion> AgonesVersionAnnotationGetter<Context>
    for EncodeAgonesVersionAnnotation
{
    fn agones_version_annotation(context: &Context) -> String {
        annotation_text::version_annotation_text(context.agones_build_version().as_bytes())
    }
}

#[derive(Clone, Debug, HasField, Deftly)]
#[derive_deftly(FieldwiseDefault)]
pub struct AgonesBuildVersionContext {
    pub agones_build_version: AgonesBuildVersion,
}

impl AgonesBuildVersionContext {
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            agones_build_version: AgonesBuildVersion::new(bytes),
        }
    }
}

delegate_and_check_components! {
    AgonesBuildVersionContext {
        AgonesBuildVersionGetterComponent: UseField<Symbol!("agones_build_version")>,
        AgonesVersionAnnotationGetterComponent: EncodeAgonesVersionAnnotation,
    }
}
