//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_important_styles;
pub mod no_unknown_at_rule;
pub mod no_useless_escape_in_string;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_important_styles :: NoImportantStyles , self :: no_unknown_at_rule :: NoUnknownAtRule , self :: no_useless_escape_in_string :: NoUselessEscapeInString ,] } }
