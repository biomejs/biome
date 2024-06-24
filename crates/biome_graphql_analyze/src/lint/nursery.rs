//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;

pub mod use_dummy_rule;

declare_lint_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: use_dummy_rule :: UseDummyRule ,
        ]
     }
}
