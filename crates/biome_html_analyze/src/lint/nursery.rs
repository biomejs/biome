//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod use_vue_valid_v_bind;
pub mod use_vue_valid_v_else;
pub mod use_vue_valid_v_else_if;
pub mod use_vue_valid_v_html;
pub mod use_vue_valid_v_if;
pub mod use_vue_valid_v_on;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: use_vue_valid_v_bind :: UseVueValidVBind , self :: use_vue_valid_v_else :: UseVueValidVElse , self :: use_vue_valid_v_else_if :: UseVueValidVElseIf , self :: use_vue_valid_v_html :: UseVueValidVHtml , self :: use_vue_valid_v_if :: UseVueValidVIf , self :: use_vue_valid_v_on :: UseVueValidVOn ,] } }
