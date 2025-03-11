//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_dangerously_set_inner_html;
pub mod no_dangerously_set_inner_html_with_children;
pub mod no_global_eval;
declare_lint_group! { pub Security { name : "security" , rules : [self :: no_dangerously_set_inner_html :: NoDangerouslySetInnerHtml , self :: no_dangerously_set_inner_html_with_children :: NoDangerouslySetInnerHtmlWithChildren , self :: no_global_eval :: NoGlobalEval ,] } }
