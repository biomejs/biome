//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_script_url;
pub mod no_sync_scripts;
pub mod no_vue_v_if_with_v_for;
pub mod use_vue_hyphenated_attributes;
pub mod use_vue_valid_v_bind;
pub mod use_vue_valid_v_else;
pub mod use_vue_valid_v_else_if;
pub mod use_vue_valid_v_html;
pub mod use_vue_valid_v_if;
pub mod use_vue_valid_v_on;
pub mod use_vue_valid_v_text;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_script_url :: NoScriptUrl , self :: no_sync_scripts :: NoSyncScripts , self :: no_vue_v_if_with_v_for :: NoVueVIfWithVFor , self :: use_vue_hyphenated_attributes :: UseVueHyphenatedAttributes , self :: use_vue_valid_v_bind :: UseVueValidVBind , self :: use_vue_valid_v_else :: UseVueValidVElse , self :: use_vue_valid_v_else_if :: UseVueValidVElseIf , self :: use_vue_valid_v_html :: UseVueValidVHtml , self :: use_vue_valid_v_if :: UseVueValidVIf , self :: use_vue_valid_v_on :: UseVueValidVOn , self :: use_vue_valid_v_text :: UseVueValidVText ,] } }
