use crate::keywords::{
    BASIC_KEYWORDS, FONT_FAMILY_KEYWORDS, FONT_SIZE_KEYWORDS, FONT_STRETCH_KEYWORDS,
    FONT_STYLE_KEYWORDS, FONT_VARIANTS_KEYWORDS, FONT_WEIGHT_ABSOLUTE_KEYWORDS,
    FONT_WIGHT_NUMERIC_KEYWORDS, LINE_HEIGHT_KEYWORDS,
};

pub fn is_font_family_keyword(value: &str) -> bool {
    BASIC_KEYWORDS.contains(&value) || FONT_FAMILY_KEYWORDS.contains(&value)
}

// check if the value is a shorthand keyword used in `font` property
pub fn is_font_shorthand_keyword(value: &str) -> bool {
    BASIC_KEYWORDS.contains(&value)
        || FONT_STYLE_KEYWORDS.contains(&value)
        || FONT_VARIANTS_KEYWORDS.contains(&value)
        || FONT_WEIGHT_ABSOLUTE_KEYWORDS.contains(&value)
        || FONT_WIGHT_NUMERIC_KEYWORDS.contains(&value)
        || FONT_STRETCH_KEYWORDS.contains(&value)
        || FONT_SIZE_KEYWORDS.contains(&value)
        || LINE_HEIGHT_KEYWORDS.contains(&value)
        || FONT_FAMILY_KEYWORDS.contains(&value)
}

pub fn is_css_variable(value: &str) -> bool {
    value.to_lowercase().starts_with("var(")
}
