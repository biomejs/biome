//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_type_only_import_attributes;

declare_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_type_only_import_attributes :: NoTypeOnlyImportAttributes ,
        ]
     }
}
