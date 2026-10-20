//! Reusable structural derives for readable CGP contexts and domain records.
//!
//! CGP owns capabilities, providers, and delegation. Derive-deftly is kept to
//! structural boilerplate only: callers still state every nonstandard default
//! next to the field whose policy it changes.
use derive_deftly::define_derive_deftly;

define_derive_deftly! {
    /// Shared fieldwise Default derive used by both agones-core and the
    /// Rusternetes adapter contexts. CGP owns capability wiring; this template
    /// owns only structural construction policy.
    export FieldwiseDefault for struct:
    impl ::core::default::Default for $ttype {
        fn default() -> Self {
            Self {
                ${for fields {
                    $fname: ${if fmeta(default) {
                        ${fmeta(default) as expr}
                    } else {
                        ::core::default::Default::default()
                    }},
                }}
            }
        }
    }
}
