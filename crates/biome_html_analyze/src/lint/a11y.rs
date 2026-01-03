//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_access_key;
pub mod no_autofocus;
pub mod no_distracting_elements;
pub mod no_header_scope;
pub mod no_positive_tabindex;
pub mod no_svg_without_title;
pub mod use_alt_text;
pub mod use_aria_props_for_role;
pub mod use_button_type;
pub mod use_html_lang;
pub mod use_iframe_title;
pub mod use_valid_aria_role;
declare_lint_group! { pub A11y { name : "a11y" , rules : [self :: no_access_key :: NoAccessKey , self :: no_autofocus :: NoAutofocus , self :: no_distracting_elements :: NoDistractingElements , self :: no_header_scope :: NoHeaderScope , self :: no_positive_tabindex :: NoPositiveTabindex , self :: no_svg_without_title :: NoSvgWithoutTitle , self :: use_alt_text :: UseAltText , self :: use_aria_props_for_role :: UseAriaPropsForRole , self :: use_button_type :: UseButtonType , self :: use_html_lang :: UseHtmlLang , self :: use_iframe_title :: UseIframeTitle , self :: use_valid_aria_role :: UseValidAriaRole ,] } }
