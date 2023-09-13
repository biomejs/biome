//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod organize_imports;

declare_group! {
    pub (crate) Correctness {
        name : "correctness" ,
        rules : [
            self :: organize_imports :: OrganizeImports ,
        ]
     }
}
