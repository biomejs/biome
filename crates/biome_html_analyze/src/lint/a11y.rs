//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_header_scope;
declare_lint_group! { pub A11y { name : "a11y" , rules : [self :: no_header_scope :: NoHeaderScope ,] } }
