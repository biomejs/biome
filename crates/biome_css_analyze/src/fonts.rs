#![expect(
    clippy::disallowed_methods,
    reason = "This module compares CSS values that can span multiple tokens."
)]

//! Module responsible for providing utility methods regarding fonts
//!

use crate::utils::is_css_variable;
use biome_css_syntax::keywords::{
    BASIC_KEYWORDS, FONT_FAMILY_KEYWORDS, FONT_SIZE_KEYWORDS, FONT_STRETCH_KEYWORDS,
    FONT_STYLE_KEYWORDS, FONT_VARIANTS_KEYWORDS, FONT_WEIGHT_ABSOLUTE_KEYWORDS,
    FONT_WEIGHT_NUMERIC_KEYWORDS, FUNCTION_KEYWORDS, LINE_HEIGHT_KEYWORDS,
    SYSTEM_FAMILY_NAME_KEYWORDS,
};
use biome_css_syntax::{
    AnyCssFunction, AnyCssGenericComponentValue, AnyCssGenericPropertyValueOrExpression,
    AnyCssValue, AnyScssExpression, AnyScssExpressionItem, CssIdentifier, CssString,
    ScssBinaryExpression, ScssExpression, ScssListExpression,
};
use biome_rowan::{AstNode, TextRange, TokenText, declare_node_union};
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

#[derive(Clone)]
pub enum CssFontComponent {
    Value(AnyCssValue),
    Comma,
    Slash,
    OtherDelimiter,
}

impl CssFontComponent {
    pub fn as_value(&self) -> Option<&AnyCssValue> {
        match self {
            Self::Value(value) => Some(value),
            _ => None,
        }
    }

    fn is_delimiter(&self) -> bool {
        !matches!(self, Self::Value(_))
    }
}

pub fn font_components(
    value: AnyCssGenericPropertyValueOrExpression,
) -> Option<Vec<CssFontComponent>> {
    match value {
        AnyCssGenericPropertyValueOrExpression::CssGenericComponentValueList(list) => list
            .into_iter()
            .map(|component| match component {
                AnyCssGenericComponentValue::AnyCssValue(value) => {
                    Some(CssFontComponent::Value(value))
                }
                AnyCssGenericComponentValue::CssGenericDelimiter(delimiter) => {
                    let token = delimiter.value().ok()?;
                    Some(match token.text_trimmed() {
                        "," => CssFontComponent::Comma,
                        "/" => CssFontComponent::Slash,
                        _ => CssFontComponent::OtherDelimiter,
                    })
                }
            })
            .collect(),
        AnyCssGenericPropertyValueOrExpression::ScssExpression(expression) => {
            let mut components = Vec::new();
            append_scss_expression_components(&expression, &mut components)?;
            Some(components)
        }
        AnyCssGenericPropertyValueOrExpression::CssCustomPropertyValue(_)
        | AnyCssGenericPropertyValueOrExpression::CssLegacyFilterValue(_) => None,
    }
}

fn append_scss_expression_components(
    expression: &ScssExpression,
    components: &mut Vec<CssFontComponent>,
) -> Option<()> {
    for item in expression.items() {
        match item {
            AnyScssExpressionItem::AnyCssValue(value) => {
                if is_dynamic_scss_value(&value) {
                    return None;
                }
                components.push(CssFontComponent::Value(value));
            }
            AnyScssExpressionItem::CssGenericDelimiter(delimiter) => {
                let token = delimiter.value().ok()?;
                components.push(match token.text_trimmed() {
                    "," => CssFontComponent::Comma,
                    "/" => CssFontComponent::Slash,
                    _ => CssFontComponent::OtherDelimiter,
                });
            }
            AnyScssExpressionItem::ScssBinaryExpression(expression) => {
                append_scss_binary_expression_components(&expression, components)?;
            }
            AnyScssExpressionItem::ScssListExpression(list) => {
                append_scss_list_components(&list, components)?;
            }
            _ => return None,
        }
    }
    Some(())
}

fn append_scss_list_components(
    list: &ScssListExpression,
    components: &mut Vec<CssFontComponent>,
) -> Option<()> {
    for (index, element) in list.elements().into_iter().enumerate() {
        if index > 0 {
            components.push(CssFontComponent::Comma);
        }
        append_any_scss_expression_components(&element.ok()?.value().ok()?, components)?;
    }
    Some(())
}

fn append_any_scss_expression_components(
    expression: &AnyScssExpression,
    components: &mut Vec<CssFontComponent>,
) -> Option<()> {
    match expression {
        AnyScssExpression::AnyCssValue(value) => {
            if is_dynamic_scss_value(value) {
                None
            } else {
                components.push(CssFontComponent::Value(value.clone()));
                Some(())
            }
        }
        AnyScssExpression::ScssBinaryExpression(expression) => {
            append_scss_binary_expression_components(expression, components)
        }
        AnyScssExpression::ScssExpression(expression) => {
            append_scss_expression_components(expression, components)
        }
        AnyScssExpression::ScssListExpression(list) => {
            append_scss_list_components(list, components)
        }
        _ => None,
    }
}

fn append_scss_binary_expression_components(
    expression: &ScssBinaryExpression,
    components: &mut Vec<CssFontComponent>,
) -> Option<()> {
    if expression.operator().ok()?.text_trimmed() != "/" {
        return None;
    }
    append_any_scss_expression_components(&expression.left().ok()?, components)?;
    components.push(CssFontComponent::Slash);
    append_any_scss_expression_components(&expression.right().ok()?, components)
}

fn is_dynamic_scss_value(value: &AnyCssValue) -> bool {
    fn is_dynamic(value: &AnyCssValue) -> bool {
        matches!(
            value,
            AnyCssValue::ScssInterpolatedIdentifier(_)
                | AnyCssValue::ScssInterpolatedString(_)
                | AnyCssValue::ScssInterpolatedValue(_)
                | AnyCssValue::ScssModuleMemberAccess(_)
                | AnyCssValue::ScssParentSelectorValue(_)
                | AnyCssValue::ScssVariable(_)
        ) || matches!(
            value,
            AnyCssValue::AnyCssFunction(AnyCssFunction::CssFunction(_))
        ) && !value.matches_function(FUNCTION_KEYWORDS)
    }

    is_dynamic(value)
        || value
            .syntax()
            .descendants()
            .filter_map(AnyCssValue::cast)
            .any(|value| is_dynamic(&value))
}

/// Get the font-families within a `font` shorthand property value.
pub fn find_font_family(value: &[CssFontComponent]) -> Vec<CssFontValue> {
    let mut font_families: Vec<CssFontValue> = Vec::new();
    // Vector needed to collect identifiers that are next to each other, eventually separated by colon
    let mut identifiers_collector: Vec<CssIdentifier> = vec![];
    for (index, component) in value.iter().enumerate() {
        let Some(css_value) = component.as_value() else {
            if !identifiers_collector.is_empty() {
                font_families.push(CssFontValue::MultipleValue(
                    std::mem::take(&mut identifiers_collector)
                        .into_iter()
                        .map(AnyCssFontValue::from)
                        .collect(),
                ));
            }
            continue;
        };

        let text = css_value.to_trimmed_text();
        let lower_case_value = text.text().to_ascii_lowercase_cow();

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
        if matches!(css_value, AnyCssValue::AnyCssDimension(_)) {
            continue;
        }

        // Ignore anything come after a <font-size>/, because it's a line-height
        if index >= 2
            && matches!(value[index - 1], CssFontComponent::Slash)
            && matches!(
                value[index - 2],
                CssFontComponent::Value(AnyCssValue::AnyCssDimension(_))
            )
        {
            continue;
        }

        // Ignore number values
        if matches!(css_value, AnyCssValue::CssNumber(_)) {
            continue;
        }

        match css_value {
            AnyCssValue::CssIdentifier(node) => {
                if value.get(index + 1).is_some_and(|next| next.is_delimiter()) {
                    if identifiers_collector.is_empty() {
                        font_families.push(CssFontValue::SingleValue(node.clone().into()));
                    } else {
                        identifiers_collector.push(node.clone());
                    }
                } else if index + 1 < value.len() {
                    identifiers_collector.push(node.clone());
                } else if identifiers_collector.is_empty() {
                    font_families.push(CssFontValue::SingleValue(node.clone().into()));
                } else {
                    identifiers_collector.push(node.clone());
                    font_families.push(CssFontValue::MultipleValue(
                        std::mem::take(&mut identifiers_collector)
                            .into_iter()
                            .map(AnyCssFontValue::from)
                            .collect(),
                    ));
                }
            }
            AnyCssValue::CssString(node) => {
                font_families.push(CssFontValue::SingleValue(node.clone().into()));
            }
            _ => {}
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
