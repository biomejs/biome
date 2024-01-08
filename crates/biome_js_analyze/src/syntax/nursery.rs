//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_empty_type_parameters;
pub(crate) mod no_type_only_import_attributes;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_empty_type_parameters :: NoEmptyTypeParameters ,
            self :: no_type_only_import_attributes :: NoTypeOnlyImportAttributes ,
        ]
     }
}
