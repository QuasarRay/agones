//! Literal utility filenames, registered outside the mirror.
#[path = "../../../agones-rs/pkg/util/doc.rs"]
mod documentation;
#[path = "../../../agones-rs/pkg/util/errors/errors.rs"]
pub mod errors;
#[path = "../../../agones-rs/pkg/util/logfields/logfields.rs"]
pub mod logfields;
#[path = "../../../agones-rs/pkg/util/runtime/features.rs"]
pub mod features;
#[path = "../../../agones-rs/pkg/util/crd/crd.rs"]
pub mod crd;
#[path = "utility_https_module.rs"]
pub mod https;
#[path = "../../../agones-rs/pkg/util/httpserver/server.rs"]
pub mod httpserver;
#[path = "../../../agones-rs/pkg/util/fswatch/fswatch.rs"]
pub mod fswatch;

#[path = "utility_runtime_module.rs"]
pub mod runtime;
#[path = "../../../agones-rs/pkg/util/webhooks/webhooks.rs"]
pub mod webhooks;
#[path = "../../../agones-rs/pkg/util/apiserver/apiserver.rs"]
pub mod apiserver;
#[path = "../../../agones-rs/pkg/util/signals/signals.rs"]
pub mod signals;
#[path = "../../../agones-rs/pkg/util/workerqueue/workerqueue.rs"]
pub mod workerqueue;
