//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_invalid_new_builtin;
pub(crate) mod no_this_in_static;
pub(crate) mod no_unused_imports;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_invalid_new_builtin :: NoInvalidNewBuiltin ,
            self :: no_this_in_static :: NoThisInStatic ,
            self :: no_unused_imports :: NoUnusedImports ,
        ]
     }
}
