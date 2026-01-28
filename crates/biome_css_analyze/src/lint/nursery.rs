//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_empty_source;
pub mod no_excessive_lines_per_file;
pub mod no_hex_colors;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_empty_source :: NoEmptySource , self :: no_excessive_lines_per_file :: NoExcessiveLinesPerFile , self :: no_hex_colors :: NoHexColors ,] } }
