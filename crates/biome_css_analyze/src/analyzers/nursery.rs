//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod color_no_invalid_hex;

declare_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: color_no_invalid_hex :: ColorNoInvalidHex ,
        ]
     }
}
