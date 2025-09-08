//! Module responsible for providing utility methods regarding fonts
//!

use crate::keywords::{
    BASIC_KEYWORDS, FONT_FAMILY_KEYWORDS, FONT_SIZE_KEYWORDS, FONT_STRETCH_KEYWORDS,
    FONT_STYLE_KEYWORDS, FONT_VARIANTS_KEYWORDS, FONT_WEIGHT_ABSOLUTE_KEYWORDS,
    FONT_WEIGHT_NUMERIC_KEYWORDS, FUNCTION_KEYWORDS, LINE_HEIGHT_KEYWORDS,
    SYSTEM_FAMILY_NAME_KEYWORDS,
};
use crate::utils::is_css_variable;
use biome_css_syntax::{
    AnyCssGenericComponentValue, AnyCssValue, CssGenericComponentValueList, CssIdentifier,
    CssString,
};
use biome_rowan::{AstNode, AstNodeList, SyntaxNodeCast, TextRange, TokenText, declare_node_union};
use biome_string_case::StrLikeExtension;
use std::hash::Hash;

/// Particular type that holds the value of a particular font.
/// This type implements a particular algorithm of [PartialEq] and [Hash], where the
/// values checked are the **trimmed text of their value**, which means that
/// nodes (raw values and ranges) aren't taken into consideration.
#[derive(Debug, Clone, Eq)]
pub enum CssFontValue {
    /// Groups those font names that are represented by a multiple [CssIdentifier].
    ///
    /// ## Examples
    ///
    /// ```css
    /// code {
    ///     font-family: Liberation Mono, SF Mono
    /// }
    /// ```
    MultipleValue(Vec<AnyCssFontValue>),
    /// Groups those font names that are represented by a single [CssIdentifier] or a single [CssString].
    ///
    /// ## Examples
    ///
    /// ```css
    /// code {
    ///     font-family: "Arial", Arial, "Liberation Mono"
    /// }
    /// ```
    SingleValue(AnyCssFontValue),
}

impl Hash for CssFontValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::SingleValue(node) => {
                if let Some(text) = node.inner_string_text() {
                    text.text().trim().hash(state);
                } else {
                    state.write_u8(0);
                }
            }
            Self::MultipleValue(nodes) => {
                for node in nodes {
                    if let Some(text) = node.inner_string_text() {
                        text.text().trim().hash(state);
                    } else {
                        state.write_u8(0);
                    }
                }
            }
        }
    }
}

impl PartialEq for CssFontValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SingleValue(this), Self::SingleValue(other)) => {
                if let (Some(this), Some(other)) =
                    (this.inner_string_text(), other.inner_string_text())
                {
                    this.text().trim() == other.text().trim()
                } else {
                    false
                }
            }
            (Self::MultipleValue(this_values), Self::MultipleValue(other_values)) => {
                this_values.len() == other_values.len()
                    && this_values
                        .iter()
                        .zip(other_values.iter())
                        .all(|(this, other)| {
                            if let (Some(this), Some(other)) =
                                (this.inner_string_text(), other.inner_string_text())
                            {
                                this.text().trim() == other.text().trim()
                            } else {
                                false
                            }
                        })
            }
            _ => false,
        }
    }
}

impl CssFontValue {
    pub fn is_identifier(&self) -> bool {
        match self {
            Self::MultipleValue(nodes) => nodes
                .iter()
                .all(|node| matches!(node, AnyCssFontValue::CssIdentifier(_))),
            Self::SingleValue(AnyCssFontValue::CssIdentifier(_)) => true,
            Self::SingleValue(AnyCssFontValue::CssString(_)) => false,
        }
    }

    pub fn range(&self) -> TextRange {
        match &self {
            // SAFETY: we assume the caller provides a non-empty list
            Self::MultipleValue(nodes) => TextRange::new(
                nodes
                    .first()
                    .expect("The list nodes cannot be empty")
                    .range()
                    .start(),
                nodes
                    .last()
                    .expect("The list nodes cannot be empty")
                    .range()
                    .end(),
            ),
            Self::SingleValue(node) => node.range(),
        }
    }

    /// Returns the value of the font with quotes
    pub fn to_string(&self) -> Option<String> {
        match self {
            Self::SingleValue(node) => Some(node.to_raw_text()?.to_string()),
            Self::MultipleValue(nodes) => {
                let string = nodes
                    .iter()
                    .filter_map(|node| {
                        let text = node.to_raw_text()?;
                        Some(text.to_string())
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                Some(string)
            }
        }
    }
}

declare_node_union! {
    pub AnyCssFontValue = CssString | CssIdentifier
}

impl AnyCssFontValue {
    /// Returns the value without quotes
    fn inner_string_text(&self) -> Option<TokenText> {
        match self {
            Self::CssString(node) => Some(node.inner_string_text().ok()?),
            Self::CssIdentifier(node) => Some(node.value_token().ok()?.token_text_trimmed()),
        }
    }

    /// Returns the value with quotes
    fn to_raw_text(&self) -> Option<TokenText> {
        match self {
            Self::CssString(node) => Some(node.value_token().ok()?.token_text_trimmed()),
            Self::CssIdentifier(node) => Some(node.value_token().ok()?.token_text_trimmed()),
        }
    }
}

/// Get the font-families within a `font` shorthand property value.
pub fn find_font_family(value: CssGenericComponentValueList) -> Vec<CssFontValue> {
    let mut font_families: Vec<CssFontValue> = Vec::new();
    // Vector needed to collect identifiers that are next to each other, eventually separated by colon
    let mut identifiers_collector: Vec<CssIdentifier> = vec![];
    let mut iter = value.iter().peekable();
    while let Some(v) = iter.next() {
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
        if let Some(prev_node) = v.syntax().prev_sibling()
            && let Some(prev_prev_node) = prev_node.prev_sibling()
            && let Some(slash) = prev_node.cast::<AnyCssGenericComponentValue>()
            && let Some(size) = prev_prev_node.cast::<AnyCssGenericComponentValue>()
            && matches!(
                size,
                AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::AnyCssDimension(_))
            )
            && matches!(slash, AnyCssGenericComponentValue::CssGenericDelimiter(_))
        {
            continue;
        };

        // Ignore number values
        if matches!(
            v,
            AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::CssNumber(_))
        ) {
            continue;
        }

        match v {
            AnyCssGenericComponentValue::CssGenericDelimiter(_) => {
                if !identifiers_collector.is_empty() {
                    font_families.push(CssFontValue::MultipleValue(
                        std::mem::take(&mut identifiers_collector)
                            .into_iter()
                            .map(AnyCssFontValue::from)
                            .collect(),
                    ));
                    identifiers_collector.clear();
                }
            }
            AnyCssGenericComponentValue::AnyCssValue(css_value) => match css_value {
                AnyCssValue::CssIdentifier(node) => {
                    // We query the next node first
                    if let Some(next_node) = iter.peek() {
                        if matches!(
                            next_node,
                            AnyCssGenericComponentValue::CssGenericDelimiter(_)
                        ) {
                            if identifiers_collector.is_empty() {
                                font_families.push(CssFontValue::SingleValue(node.into()));
                            } else {
                                identifiers_collector.push(node);
                            }
                        } else {
                            identifiers_collector.push(node);
                        }
                    }
                    // If we're here, the list finished and we're computing the last node
                    else {
                        // If there aren't pending nodes, we add the current node
                        if identifiers_collector.is_empty() {
                            font_families.push(CssFontValue::SingleValue(node.into()));
                        } else {
                            // We push this node to the list
                            identifiers_collector.push(node);

                            // This is a multiple value
                            font_families.push(CssFontValue::MultipleValue(
                                std::mem::take(&mut identifiers_collector)
                                    .into_iter()
                                    .map(AnyCssFontValue::from)
                                    .collect::<Vec<_>>(),
                            ));

                            identifiers_collector.clear();
                        }
                    }
                }
                AnyCssValue::CssString(node) => {
                    font_families.push(CssFontValue::SingleValue(node.into()));
                }
                _ => {}
            },
        }
    }
    font_families
}

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

/// Check if the value is a known CSS value function.
pub fn is_function_keyword(value: &str) -> bool {
    FUNCTION_KEYWORDS
        .binary_search(&value.to_ascii_lowercase_cow().as_ref())
        .is_ok()
}
