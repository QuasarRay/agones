// Copyright Contributors to Agones a Series of LF Projects, LLC.
// SPDX-License-Identifier: Apache-2.0
//! The pinned source's tools build profile retains five external tools.
//!
//! These are build inputs. Their consumers keep the source command paths;
//! production emission and invocation are owned by the build/tool milestones.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceTool {
    pub import_path: &'static str,
    pub module: &'static str,
    pub consumers: &'static [&'static str],
}

pub const TOOLS: &[SourceTool] = &[
    SourceTool {
        import_path: "github.com/ahmetb/gen-crd-api-reference-docs",
        module: "github.com/ahmetb/gen-crd-api-reference-docs",
        consumers: &["site/gen-api-docs.sh"],
    },
    SourceTool {
        import_path: "github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-grpc-gateway",
        module: "github.com/grpc-ecosystem/grpc-gateway/v2",
        consumers: &["build-sdk-images/go/gen.sh"],
    },
    SourceTool {
        import_path: "github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2",
        module: "github.com/grpc-ecosystem/grpc-gateway/v2",
        consumers: &["build-sdk-images/go/gen.sh"],
    },
    SourceTool {
        import_path: "golang.org/x/tools/cmd/goimports",
        module: "golang.org/x/tools",
        consumers: &["build-image/Dockerfile"],
    },
    SourceTool {
        import_path: "google.golang.org/protobuf/cmd/protoc-gen-go",
        module: "google.golang.org/protobuf",
        consumers: &["build-sdk-images/go/Dockerfile"],
    },
];

pub fn tools_for(consumer: &str) -> impl Iterator<Item = &'static SourceTool> + '_ {
    TOOLS.iter().filter(move |tool| tool.consumers.contains(&consumer))
}
