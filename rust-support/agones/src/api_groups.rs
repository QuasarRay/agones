//! The four source API packages have distinct GroupName declarations.
//! Registration aliases the packages without changing their source filenames.
#[path = "../../../agones-rs/pkg/apis/agones/register.rs"]
pub mod agones;
#[path = "../../../agones-rs/pkg/apis/allocation/register.rs"]
pub mod allocation;
#[path = "../../../agones-rs/pkg/apis/autoscaling/register.rs"]
pub mod autoscaling;
#[path = "../../../agones-rs/pkg/apis/multicluster/register.rs"]
pub mod multicluster;
