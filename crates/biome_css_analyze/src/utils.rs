use crate::keywords::{
    BASIC_KEYWORDS, FONT_FAMILY_KEYWORDS, FONT_SIZE_KEYWORDS, FONT_STRETCH_KEYWORDS,
    FONT_STYLE_KEYWORDS, FONT_VARIANTS_KEYWORDS, FONT_WEIGHT_ABSOLUTE_KEYWORDS,
    FONT_WEIGHT_NUMERIC_KEYWORDS, FUNCTION_KEYWORDS, KNOWN_CHROME_PROPERTIES,
    KNOWN_EDGE_PROPERTIES, KNOWN_EXPLORER_PROPERTIES, KNOWN_FIREFOX_PROPERTIES, KNOWN_PROPERTIES,
    KNOWN_SAFARI_PROPERTIES, KNOWN_SUMSUNG_INTERNET_PROPERTIES, KNOWN_US_BROWSER_PROPERTIES,
    LEVEL_ONE_AND_TWO_PSEUDO_ELEMENTS, LINE_HEIGHT_KEYWORDS, OTHER_PSEUDO_ELEMENTS,
    SHADOW_TREE_PSEUDO_ELEMENTS, SYSTEM_FAMILY_NAME_KEYWORDS, VENDER_PREFIXES,
    VENDOR_SPECIFIC_PSEUDO_ELEMENTS,
};
use biome_css_syntax::{AnyCssGenericComponentValue, AnyCssValue, CssGenericComponentValueList};
use biome_rowan::{AstNode, SyntaxNodeCast};

pub fn is_font_family_keyword(value: &str) -> bool {
    BASIC_KEYWORDS.contains(&value) || FONT_FAMILY_KEYWORDS.contains(&value)
}

pub fn is_system_family_name_keyword(value: &str) -> bool {
    BASIC_KEYWORDS.contains(&value) || SYSTEM_FAMILY_NAME_KEYWORDS.contains(&value)
}

// check if the value is a shorthand keyword used in `font` property
pub fn is_font_shorthand_keyword(value: &str) -> bool {
    BASIC_KEYWORDS.contains(&value)
        || FONT_STYLE_KEYWORDS.contains(&value)
        || FONT_VARIANTS_KEYWORDS.contains(&value)
        || FONT_WEIGHT_ABSOLUTE_KEYWORDS.contains(&value)
        || FONT_WEIGHT_NUMERIC_KEYWORDS.contains(&value)
        || FONT_STRETCH_KEYWORDS.contains(&value)
        || FONT_SIZE_KEYWORDS.contains(&value)
        || LINE_HEIGHT_KEYWORDS.contains(&value)
        || FONT_FAMILY_KEYWORDS.contains(&value)
}

pub fn is_css_variable(value: &str) -> bool {
    value.to_lowercase().starts_with("var(")
}

/// Get the font-families within a `font` shorthand property value.
pub fn find_font_family(value: CssGenericComponentValueList) -> Vec<AnyCssValue> {
    let mut font_families: Vec<AnyCssValue> = Vec::new();
    for v in value {
        let lower_case_value = v.text().to_lowercase();

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
            AnyCssGenericComponentValue::CssGenericDelimiter(_) => continue,
            AnyCssGenericComponentValue::AnyCssValue(css_value) => match css_value {
                AnyCssValue::CssIdentifier(_) | AnyCssValue::CssString(_) => {
                    font_families.push(css_value)
                }
                _ => continue,
            },
        }
    }
    font_families
}

/// Check if the value is a known CSS value function.
pub fn is_function_keyword(value: &str) -> bool {
    FUNCTION_KEYWORDS
        .binary_search(&value.to_lowercase().as_str())
        .is_ok()
}

/// Check if the value is a double-dashed custom function.
pub fn is_custom_function(value: &str) -> bool {
    value.starts_with("--")
}

// Returns the vendor prefix extracted from an input string.
pub fn vender_prefix(prop: &str) -> String {
    for prefix in VENDER_PREFIXES.iter() {
        if prop.starts_with(prefix) {
            return (*prefix).to_string();
        }
    }
    String::new()
}

pub fn is_pseudo_elements(prop: &str) -> bool {
    LEVEL_ONE_AND_TWO_PSEUDO_ELEMENTS.contains(&prop)
        || VENDOR_SPECIFIC_PSEUDO_ELEMENTS.contains(&prop)
        || SHADOW_TREE_PSEUDO_ELEMENTS.contains(&prop)
        || OTHER_PSEUDO_ELEMENTS.contains(&prop)
}

pub fn is_known_properties(prop: &str) -> bool {
    KNOWN_PROPERTIES.binary_search(&prop).is_ok()
        || KNOWN_CHROME_PROPERTIES.binary_search(&prop).is_ok()
        || KNOWN_EDGE_PROPERTIES.binary_search(&prop).is_ok()
        || KNOWN_EXPLORER_PROPERTIES.binary_search(&prop).is_ok()
        || KNOWN_FIREFOX_PROPERTIES.binary_search(&prop).is_ok()
        || KNOWN_SAFARI_PROPERTIES.binary_search(&prop).is_ok()
        || KNOWN_SUMSUNG_INTERNET_PROPERTIES
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
