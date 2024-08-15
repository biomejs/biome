//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;

pub mod no_duplicated_fields;
pub mod use_deprecated_reason;

declare_lint_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_duplicated_fields :: NoDuplicatedFields ,
            self :: use_deprecated_reason :: UseDeprecatedReason ,
        ]
     }
}
