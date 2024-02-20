//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_access_key;
pub mod no_autofocus;
pub mod no_blank_target;
pub mod no_distracting_elements;
pub mod no_header_scope;
pub mod no_redundant_alt;
pub mod no_svg_without_title;
pub mod use_alt_text;
pub mod use_anchor_content;
pub mod use_heading_content;
pub mod use_html_lang;
pub mod use_iframe_title;
pub mod use_key_with_click_events;
pub mod use_key_with_mouse_events;
pub mod use_media_caption;
pub mod use_valid_anchor;

declare_group! {
    pub A11y {
        name : "a11y" ,
        rules : [
            self :: no_access_key :: NoAccessKey ,
            self :: no_autofocus :: NoAutofocus ,
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
