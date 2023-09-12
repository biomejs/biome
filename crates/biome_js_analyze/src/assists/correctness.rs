//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod flip_bin_exp;
pub(crate) mod inline_variable;
pub(crate) mod organize_imports;

declare_group! {
    pub (crate) Correctness {
        name : "correctness" ,
        rules : [
            self :: flip_bin_exp :: FlipBinExp ,
            self :: inline_variable :: InlineVariable ,
            self :: organize_imports :: OrganizeImports ,
        ]
     }
}
