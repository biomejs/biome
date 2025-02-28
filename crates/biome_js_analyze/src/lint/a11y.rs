//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_access_key;
pub mod no_aria_hidden_on_focusable;
pub mod no_aria_unsupported_elements;
pub mod no_autofocus;
pub mod no_blank_target;
pub mod no_distracting_elements;
pub mod no_header_scope;
pub mod no_interactive_element_to_noninteractive_role;
pub mod no_label_without_control;
pub mod no_noninteractive_element_to_interactive_role;
pub mod no_noninteractive_tabindex;
pub mod no_positive_tabindex;
pub mod no_redundant_alt;
pub mod no_redundant_roles;
pub mod no_svg_without_title;
pub mod use_alt_text;
pub mod use_anchor_content;
pub mod use_aria_activedescendant_with_tabindex;
pub mod use_aria_props_for_role;
pub mod use_button_type;
pub mod use_focusable_interactive;
pub mod use_heading_content;
pub mod use_html_lang;
pub mod use_iframe_title;
pub mod use_key_with_click_events;
pub mod use_key_with_mouse_events;
pub mod use_media_caption;
pub mod use_semantic_elements;
pub mod use_valid_anchor;
pub mod use_valid_aria_props;
pub mod use_valid_aria_role;
pub mod use_valid_aria_values;
pub mod use_valid_lang;
declare_lint_group! { pub A11y { name : "a11y" , rules : [self :: no_access_key :: NoAccessKey , self :: no_aria_hidden_on_focusable :: NoAriaHiddenOnFocusable , self :: no_aria_unsupported_elements :: NoAriaUnsupportedElements , self :: no_autofocus :: NoAutofocus , self :: no_blank_target :: NoBlankTarget , self :: no_distracting_elements :: NoDistractingElements , self :: no_header_scope :: NoHeaderScope , self :: no_interactive_element_to_noninteractive_role :: NoInteractiveElementToNoninteractiveRole , self :: no_label_without_control :: NoLabelWithoutControl , self :: no_noninteractive_element_to_interactive_role :: NoNoninteractiveElementToInteractiveRole , self :: no_noninteractive_tabindex :: NoNoninteractiveTabindex , self :: no_positive_tabindex :: NoPositiveTabindex , self :: no_redundant_alt :: NoRedundantAlt , self :: no_redundant_roles :: NoRedundantRoles , self :: no_svg_without_title :: NoSvgWithoutTitle , self :: use_alt_text :: UseAltText , self :: use_anchor_content :: UseAnchorContent , self :: use_aria_activedescendant_with_tabindex :: UseAriaActivedescendantWithTabindex , self :: use_aria_props_for_role :: UseAriaPropsForRole , self :: use_button_type :: UseButtonType , self :: use_focusable_interactive :: UseFocusableInteractive , self :: use_heading_content :: UseHeadingContent , self :: use_html_lang :: UseHtmlLang , self :: use_iframe_title :: UseIframeTitle , self :: use_key_with_click_events :: UseKeyWithClickEvents , self :: use_key_with_mouse_events :: UseKeyWithMouseEvents , self :: use_media_caption :: UseMediaCaption , self :: use_semantic_elements :: UseSemanticElements , self :: use_valid_anchor :: UseValidAnchor , self :: use_valid_aria_props :: UseValidAriaProps , self :: use_valid_aria_role :: UseValidAriaRole , self :: use_valid_aria_values :: UseValidAriaValues , self :: use_valid_lang :: UseValidLang ,] } }
