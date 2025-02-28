//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod use_generic_font_names;
declare_lint_group! { pub A11y { name : "a11y" , rules : [self :: use_generic_font_names :: UseGenericFontNames ,] } }
