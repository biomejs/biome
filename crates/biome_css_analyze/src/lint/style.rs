//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_descending_specificity;
pub mod no_value_at_rule;
declare_lint_group! { pub Style { name : "style" , rules : [self :: no_descending_specificity :: NoDescendingSpecificity , self :: no_value_at_rule :: NoValueAtRule ,] } }
