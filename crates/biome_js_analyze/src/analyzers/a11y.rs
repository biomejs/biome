//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_access_key;
pub(crate) mod no_auto_focus;
pub(crate) mod no_blank_target;
pub(crate) mod no_distracting_elements;
pub(crate) mod no_header_scope;
pub(crate) mod no_redundant_alt;
pub(crate) mod no_svg_without_title;
pub(crate) mod use_alt_text;
pub(crate) mod use_anchor_content;
pub(crate) mod use_heading_content;
pub(crate) mod use_html_lang;
pub(crate) mod use_iframe_title;
pub(crate) mod use_key_with_click_events;
pub(crate) mod use_key_with_mouse_events;
pub(crate) mod use_media_caption;
pub(crate) mod use_valid_anchor;

declare_group! {
    pub (crate) A11y {
        name : "a11y" ,
        rules : [
            self :: no_access_key :: NoAccessKey ,
            self :: no_auto_focus :: NoAutoFocus ,
            self :: no_blank_target :: NoBlankTarget ,
            self :: no_distracting_elements :: NoDistractingElements ,
            self :: no_header_scope :: NoHeaderScope ,
            self :: no_redundant_alt :: NoRedundantAlt ,
            self :: no_svg_without_title :: NoSvgWithoutTitle ,
            self :: use_alt_text :: UseAltText ,
            self :: use_anchor_content :: UseAnchorContent ,
            self :: use_heading_content :: UseHeadingContent ,
            self :: use_html_lang :: UseHtmlLang ,
            self :: use_iframe_title :: UseIframeTitle ,
            self :: use_key_with_click_events :: UseKeyWithClickEvents ,
            self :: use_key_with_mouse_events :: UseKeyWithMouseEvents ,
            self :: use_media_caption :: UseMediaCaption ,
            self :: use_valid_anchor :: UseValidAnchor ,
        ]
     }
}
