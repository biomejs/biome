//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_dangerously_set_inner_html;
pub(crate) mod no_dangerously_set_inner_html_with_children;

declare_group! {
    pub (crate) Security {
        name : "security" ,
        rules : [
            self :: no_dangerously_set_inner_html :: NoDangerouslySetInnerHtml ,
            self :: no_dangerously_set_inner_html_with_children :: NoDangerouslySetInnerHtmlWithChildren ,
        ]
     }
}
