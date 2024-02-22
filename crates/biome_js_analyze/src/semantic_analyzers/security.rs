//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_dangerously_set_inner_html;
pub mod no_dangerously_set_inner_html_with_children;

declare_group! {
    pub Security {
        name : "security" ,
        rules : [
            self :: no_dangerously_set_inner_html :: NoDangerouslySetInnerHtml ,
            self :: no_dangerously_set_inner_html_with_children :: NoDangerouslySetInnerHtmlWithChildren ,
        ]
     }
}
