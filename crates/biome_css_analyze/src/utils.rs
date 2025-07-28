use crate::keywords::{
    A_NPLUS_BNOTATION_PSEUDO_CLASSES, A_NPLUS_BOF_SNOTATION_PSEUDO_CLASSES,
    AT_RULE_PAGE_PSEUDO_CLASSES, BASIC_KEYWORDS, FONT_FAMILY_KEYWORDS, FONT_SIZE_KEYWORDS,
    FONT_STRETCH_KEYWORDS, FONT_STYLE_KEYWORDS, FONT_VARIANTS_KEYWORDS,
    FONT_WEIGHT_ABSOLUTE_KEYWORDS, FONT_WEIGHT_NUMERIC_KEYWORDS, FUNCTION_KEYWORDS, HTML_TAGS,
    KNOWN_CHROME_PROPERTIES, KNOWN_EDGE_PROPERTIES, KNOWN_EXPLORER_PROPERTIES,
    KNOWN_FIREFOX_PROPERTIES, KNOWN_PROPERTIES, KNOWN_SAFARI_PROPERTIES,
    KNOWN_SAMSUNG_INTERNET_PROPERTIES, KNOWN_US_BROWSER_PROPERTIES,
    LEVEL_ONE_AND_TWO_PSEUDO_ELEMENTS, LINE_HEIGHT_KEYWORDS, LINGUISTIC_PSEUDO_CLASSES,
    LOGICAL_COMBINATIONS_PSEUDO_CLASSES, LONGHAND_SUB_PROPERTIES_OF_SHORTHAND_PROPERTIES,
    MATH_ML_TAGS, MEDIA_FEATURE_NAMES, OTHER_PSEUDO_CLASSES, OTHER_PSEUDO_ELEMENTS,
    RESET_TO_INITIAL_PROPERTIES_BY_BORDER, RESET_TO_INITIAL_PROPERTIES_BY_FONT,
    RESOURCE_STATE_PSEUDO_CLASSES, SHADOW_TREE_PSEUDO_ELEMENTS, SHORTHAND_PROPERTIES, SVG_TAGS,
    SYSTEM_FAMILY_NAME_KEYWORDS, VENDOR_PREFIXES, VENDOR_SPECIFIC_PSEUDO_ELEMENTS,
};
use biome_css_syntax::{
    AnyCssGenericComponentValue, AnyCssValue, CssGenericComponentValueList, CssSyntaxToken,
};
use biome_rowan::AstNodeList;
use biome_rowan::{AstNode, SyntaxNodeCast};
use biome_string_case::{StrLikeExtension, StrOnlyExtension};

pub fn is_font_family_keyword(value: &str) -> bool {
    BASIC_KEYWORDS.binary_search(&value).is_ok()
        || FONT_FAMILY_KEYWORDS.binary_search(&value).is_ok()
}

pub fn is_system_family_name_keyword(value: &str) -> bool {
    BASIC_KEYWORDS.binary_search(&value).is_ok()
        || SYSTEM_FAMILY_NAME_KEYWORDS.binary_search(&value).is_ok()
}

// check if the value is a shorthand keyword used in `font` property
pub fn is_font_shorthand_keyword(value: &str) -> bool {
    BASIC_KEYWORDS.binary_search(&value).is_ok()
        || FONT_STYLE_KEYWORDS.binary_search(&value).is_ok()
        || FONT_VARIANTS_KEYWORDS.binary_search(&value).is_ok()
        || FONT_WEIGHT_ABSOLUTE_KEYWORDS.binary_search(&value).is_ok()
        || FONT_WEIGHT_NUMERIC_KEYWORDS.binary_search(&value).is_ok()
        || FONT_STRETCH_KEYWORDS.binary_search(&value).is_ok()
        || FONT_SIZE_KEYWORDS.binary_search(&value).is_ok()
        || LINE_HEIGHT_KEYWORDS.binary_search(&value).is_ok()
        || FONT_FAMILY_KEYWORDS.binary_search(&value).is_ok()
}

pub fn is_css_variable(value: &str) -> bool {
    value.to_ascii_lowercase_cow().starts_with("var(")
}

/// Get the font-families within a `font` shorthand property value.
// TODO: This function should be replaced with `parse_shorthand_font_families`
pub fn find_font_family(value: CssGenericComponentValueList) -> Vec<AnyCssValue> {
    let mut font_families: Vec<AnyCssValue> = Vec::new();
    for v in value {
        let value = v.to_trimmed_text();
        let lower_case_value = value.text().to_ascii_lowercase_cow();

        // Ignore CSS variables
        if is_css_variable(&lower_case_value) {
            continue;
        }

        // Ignore keywords for other font parts
        if is_font_shorthand_keyword(&lower_case_value)
            && !is_font_family_keyword(&lower_case_value)
        {
            continue;
        }

        // Ignore font-sizes
        if matches!(
            v,
            AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::AnyCssDimension(_))
        ) {
            continue;
        }

        // Ignore anything come after a <font-size>/, because it's a line-height
        if let Some(prev_node) = v.syntax().prev_sibling() {
            if let Some(prev_prev_node) = prev_node.prev_sibling() {
                if let Some(slash) = prev_node.cast::<AnyCssGenericComponentValue>() {
                    if let Some(size) = prev_prev_node.cast::<AnyCssGenericComponentValue>() {
                        if matches!(
                            size,
                            AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::AnyCssDimension(
                                _
                            ))
                        ) && matches!(slash, AnyCssGenericComponentValue::CssGenericDelimiter(_))
                        {
                            continue;
                        }
                    }
                };
            }
        }

        // Ignore number values
        if matches!(
            v,
            AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::CssNumber(_))
        ) {
            continue;
        }

        match v {
            AnyCssGenericComponentValue::CssGenericDelimiter(_) => {}
            AnyCssGenericComponentValue::AnyCssValue(css_value) => match css_value {
                AnyCssValue::CssIdentifier(_) | AnyCssValue::CssString(_) => {
                    font_families.push(css_value)
                }
                _ => {}
            },
        }
    }
    font_families
}

#[derive(Debug, Clone)]
pub struct FontFamily {
    tokens: Vec<CssSyntaxToken>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontPropertyKind {
    FontFamily, // `font-family` property
    Shorthand,  // `font` shorthand property
}

/// Collects the list of `FontFamily` items from the given component value list
/// depending on the property kind (`font-family` or shorthand `font`).
pub fn collect_font_families(
    list: CssGenericComponentValueList,
    kind: FontPropertyKind,
) -> Vec<FontFamily> {
    match kind {
        FontPropertyKind::FontFamily => normalize_font_families(list),
        FontPropertyKind::Shorthand => normalize_shorthand_font_families(list),
    }
}

/// Parse font families from the CSS property value
/// Extract and normalize each font name to detect duplicate font names
/// in CSS font-family properties
///
/// Supported patterns:
/// 1. Quoted font names (CssString): "Arial", 'Helvetica', "Fira Sans"
///    → Remove quotes and treat as font family name
/// 2. Unquoted font names (CssIdentifier): Arial, Fira Sans, Times New Roman
///    → Multiple identifiers may be concatenated with spaces
///    → Comma delimiters separate individual font family names
fn normalize_font_families(list: CssGenericComponentValueList) -> Vec<FontFamily> {
    let mut current_font_texts: Vec<CssSyntaxToken> = Vec::new();
    let mut font_families: Vec<FontFamily> = Vec::new();

    let len = list.len();

    for (index, c) in list.into_iter().enumerate() {
        let is_last_value_node = index == len - 1;
        match c {
            AnyCssGenericComponentValue::AnyCssValue(css_value) => match css_value {
                AnyCssValue::CssIdentifier(val) => {
                    let text = val.value_token().ok().unwrap();

                    if is_last_value_node {
                        font_families.push(FontFamily {
                            tokens: vec![text.clone()],
                        });
                        continue;
                    }

                    current_font_texts.push(text.clone());
                }
                AnyCssValue::CssString(val) => {
                    let text = val.value_token().ok().unwrap();

                    font_families.push(FontFamily {
                        tokens: vec![text.clone()],
                    });
                }
                _ => {}
            },
            AnyCssGenericComponentValue::CssGenericDelimiter(_) => {
                if !current_font_texts.is_empty() {
                    font_families.push(FontFamily {
                        tokens: current_font_texts.clone(),
                    });

                    current_font_texts.clear();
                }
            }
        }
    }
    font_families
}

/// Parse font families from `font` shorthand property value
fn normalize_shorthand_font_families(list: CssGenericComponentValueList) -> Vec<FontFamily> {
    let mut current_font_texts: Vec<CssSyntaxToken> = Vec::new();
    let mut font_families: Vec<FontFamily> = Vec::new();

    let len = list.len();

    for (index, value) in list.into_iter().enumerate() {
        let is_last_value_node = index == len - 1;
        let raw_value = value.to_trimmed_text();
        let lower_case_value = raw_value.text().to_ascii_lowercase_cow();

        // Ignore CSS variables
        if is_css_variable(&lower_case_value) {
            continue;
        }

        // Ignore keywords for other font parts
        if is_font_shorthand_keyword(&lower_case_value)
            && !is_font_family_keyword(&lower_case_value)
        {
            continue;
        }

        // Ignore font-sizes
        if matches!(
            value,
            AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::AnyCssDimension(_))
        ) {
            continue;
        }

        // Ignore anything that comes after a <font-size>/ (line-height)
        if let Some(prev_node) = value.syntax().prev_sibling() {
            if let Some(prev_prev_node) = prev_node.prev_sibling() {
                if let Some(slash) = prev_node.cast::<AnyCssGenericComponentValue>() {
                    if let Some(size) = prev_prev_node.cast::<AnyCssGenericComponentValue>() {
                        if matches!(
                            size,
                            AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::AnyCssDimension(
                                _
                            ))
                        ) && matches!(slash, AnyCssGenericComponentValue::CssGenericDelimiter(_))
                        {
                            continue;
                        }
                    }
                }
            }
        }

        // Ignore number values
        if matches!(
            value,
            AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::CssNumber(_))
        ) {
            continue;
        }

        match value {
            AnyCssGenericComponentValue::CssGenericDelimiter(_) => {
                if !current_font_texts.is_empty() {
                    font_families.push(FontFamily {
                        tokens: current_font_texts.clone(),
                    });

                    current_font_texts.clear();
                }
            }
            AnyCssGenericComponentValue::AnyCssValue(css_value) => match css_value {
                AnyCssValue::CssIdentifier(val) => {
                    let text = val.value_token().ok().unwrap();

                    if is_last_value_node {
                        font_families.push(FontFamily {
                            tokens: vec![text.clone()],
                        });
                        continue;
                    }

                    current_font_texts.push(text.clone());
                }
                AnyCssValue::CssString(val) => {
                    let raw_text = val.value_token().ok().unwrap();

                    font_families.push(FontFamily {
                        tokens: vec![raw_text.clone()],
                    });
                }
                _ => {}
            },
        }
    }
    font_families
}

/// Check if the value is a known CSS value function.
pub fn is_function_keyword(value: &str) -> bool {
    FUNCTION_KEYWORDS
        .binary_search(&value.to_ascii_lowercase_cow().as_ref())
        .is_ok()
}

/// Check if the value is a double-dashed custom function.
pub fn is_custom_function(value: &str) -> bool {
    value.starts_with("--")
}

// Returns the vendor prefix extracted from an input string.
pub fn vender_prefix(prop: &str) -> &'static str {
    for prefix in VENDOR_PREFIXES.iter() {
        if prop.starts_with(prefix) {
            return prefix;
        }
    }
    ""
}

pub fn is_pseudo_elements(prop: &str) -> bool {
    LEVEL_ONE_AND_TWO_PSEUDO_ELEMENTS
        .binary_search(&prop)
        .is_ok()
        || VENDOR_SPECIFIC_PSEUDO_ELEMENTS.binary_search(&prop).is_ok()
        || SHADOW_TREE_PSEUDO_ELEMENTS.binary_search(&prop).is_ok()
        || OTHER_PSEUDO_ELEMENTS.binary_search(&prop).is_ok()
}

/// Check if the input string is custom selector
/// See https://drafts.csswg.org/css-extensions/#custom-selectors for more details
pub fn is_custom_selector(prop: &str) -> bool {
    prop.starts_with("--")
}

pub fn is_page_pseudo_class(prop: &str) -> bool {
    AT_RULE_PAGE_PSEUDO_CLASSES.binary_search(&prop).is_ok()
}

pub fn is_known_pseudo_class(prop: &str) -> bool {
    LEVEL_ONE_AND_TWO_PSEUDO_ELEMENTS
        .binary_search(&prop)
        .is_ok()
        || A_NPLUS_BNOTATION_PSEUDO_CLASSES
            .binary_search(&prop)
            .is_ok()
        || A_NPLUS_BOF_SNOTATION_PSEUDO_CLASSES
            .binary_search(&prop)
            .is_ok()
        || LINGUISTIC_PSEUDO_CLASSES.binary_search(&prop).is_ok()
        || LOGICAL_COMBINATIONS_PSEUDO_CLASSES
            .binary_search(&prop)
            .is_ok()
        || RESOURCE_STATE_PSEUDO_CLASSES.binary_search(&prop).is_ok()
        || OTHER_PSEUDO_CLASSES.binary_search(&prop).is_ok()
}

pub fn is_known_properties(prop: &str) -> bool {
    KNOWN_PROPERTIES.binary_search(&prop).is_ok()
        || KNOWN_CHROME_PROPERTIES.binary_search(&prop).is_ok()
        || KNOWN_EDGE_PROPERTIES.binary_search(&prop).is_ok()
        || KNOWN_EXPLORER_PROPERTIES.binary_search(&prop).is_ok()
        || KNOWN_FIREFOX_PROPERTIES.binary_search(&prop).is_ok()
        || KNOWN_SAFARI_PROPERTIES.binary_search(&prop).is_ok()
        || KNOWN_SAMSUNG_INTERNET_PROPERTIES
            .binary_search(&prop)
            .is_ok()
        || KNOWN_US_BROWSER_PROPERTIES.binary_search(&prop).is_ok()
}

pub fn vendor_prefixed(props: &str) -> bool {
    props.starts_with("-webkit-")
        || props.starts_with("-moz-")
        || props.starts_with("-ms-")
        || props.starts_with("-o-")
}

/// Check if the input string is a media feature name.
pub fn is_media_feature_name(prop: &str) -> bool {
    let input = prop.to_ascii_lowercase_cow();
    let count = MEDIA_FEATURE_NAMES.binary_search(&input.as_ref());
    if count.is_ok() {
        return true;
    }
    let mut has_vendor_prefix = false;
    for prefix in VENDOR_PREFIXES.iter() {
        if input.starts_with(prefix) {
            has_vendor_prefix = true;
            break;
        }
    }
    if has_vendor_prefix {
        for feature_name in MEDIA_FEATURE_NAMES.iter() {
            if input.ends_with(feature_name) {
                return true;
            }
        }
    }
    false
}

pub fn get_longhand_sub_properties(shorthand_property: &str) -> &'static [&'static str] {
    if let Ok(index) = SHORTHAND_PROPERTIES.binary_search(&shorthand_property) {
        return LONGHAND_SUB_PROPERTIES_OF_SHORTHAND_PROPERTIES[index];
    }

    &[]
}

pub fn get_reset_to_initial_properties(shorthand_property: &str) -> &'static [&'static str] {
    match shorthand_property {
        "border" => &RESET_TO_INITIAL_PROPERTIES_BY_BORDER,
        "font" => &RESET_TO_INITIAL_PROPERTIES_BY_FONT,
        _ => &[],
    }
}

fn is_custom_element(prop: &str) -> bool {
    prop.contains('-') && prop.eq(prop.to_lowercase_cow().as_ref())
}

/// Check if the input string is a known type selector.
pub fn is_known_type_selector(prop: &str) -> bool {
    let input = prop.to_ascii_lowercase_cow();
    HTML_TAGS.binary_search(&input.as_ref()).is_ok()
        || SVG_TAGS.binary_search(&prop).is_ok()
        || MATH_ML_TAGS.binary_search(&input.as_ref()).is_ok()
        || is_custom_element(prop)
}
