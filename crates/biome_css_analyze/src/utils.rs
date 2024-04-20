use crate::keywords::{
    BASIC_KEYWORDS, FONT_FAMILY_KEYWORDS, FONT_SIZE_KEYWORDS, FONT_STRETCH_KEYWORDS,
    FONT_STYLE_KEYWORDS, FONT_VARIANTS_KEYWORDS, FONT_WEIGHT_ABSOLUTE_KEYWORDS,
    FONT_WEIGHT_NUMERIC_KEYWORDS, LINE_HEIGHT_KEYWORDS,
};
use biome_css_syntax::{AnyCssGenericComponentValue, AnyCssValue, CssGenericComponentValueList};
use biome_rowan::{AstNode, SyntaxNodeCast};
use regex::Regex;

pub fn is_font_family_keyword(value: &str) -> bool {
    BASIC_KEYWORDS.contains(&value) || FONT_FAMILY_KEYWORDS.contains(&value)
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

// Get the font-families within a `font` shorthand property value.
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

pub fn strip_vendor_prefix(property: &str) -> String {
    let re = Regex::new(r"^-\w+-").unwrap();
    re.replace(property, "").to_string()
}
