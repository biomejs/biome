//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_color_invalid_hex;
pub mod no_duplicate_font_names;
pub mod no_unknown_unit;

declare_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_color_invalid_hex :: NoColorInvalidHex ,
            self :: no_duplicate_font_names :: NoDuplicateFontNames ,
            self :: no_unknown_unit :: NoUnknownUnit ,
        ]
     }
}
