// Copyright Contributors to Agones a Series of LF Projects, LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Source build identity and its metadata observation, composed through CGP.
//!
//! The source defaults pkg.Version to "dev" and may replace it at build time.
//! Cargo's package release version is a separate identity. AGONES_BUILD_VERSION
//! supplies the source build value; it is frozen into the normal Cargo build.
use cgp::prelude::*;
use derive_deftly::Deftly;
use crate::derive_deftly_template_AgonesConfigurationProviders;

#[path = "../../crates/agones-core/src/version_annotation.rs"]
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

#[derive(Clone, Debug, Default, HasField, Deftly)]
#[derive_deftly(AgonesConfigurationProviders)]
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
