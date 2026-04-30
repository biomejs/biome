use crate::utils::scss_expression::unwrap_single_expression_item;
use biome_css_syntax::{
    AnyScssExpression, CssLanguage, ScssListExpression, ScssMapExpressionPair,
    ScssParenthesizedExpression,
};
use biome_rowan::AstNode;

/// Returns `true` for the outer key expression in a SCSS map pair.
///
/// Example: in `(fn((a: b)): value)`, `fn((a: b))` returns `true`, but the
/// nested `(a: b)` returns `false`.
pub(crate) fn is_scss_map_key<N>(node: &N) -> bool
where
    N: AstNode<Language = CssLanguage>,
{
    matches!(
        scss_map_context(node),
        Some(context)
            if context.role == ScssMapRole::Key
                && context.position_kind == ScssMapPositionKind::Direct
    )
}

/// Returns `true` when `node` is anywhere on the key side of a SCSS map pair.
///
/// Example: in `(fn((a: b)): value)`, both `fn((a: b))` and the nested
/// `(a: b)` return `true`.
pub(crate) fn is_in_scss_map_key<N>(node: &N) -> bool
where
    N: AstNode<Language = CssLanguage>,
{
    matches!(
        scss_map_context(node),
        Some(context) if context.role == ScssMapRole::Key
    )
}

/// Returns `true` when `node` is the direct value expression of a SCSS map
/// pair.
///
/// Example: in `(key: (a, b))`, the outer `(a, b)` returns `true`, but `a`
/// does not. A single-item `ScssExpression` wrapper still counts as direct.
pub(crate) fn is_scss_map_value<N>(node: &N) -> bool
where
    N: AstNode<Language = CssLanguage>,
{
    matches!(
        scss_map_context(node),
        Some(context)
            if context.role == ScssMapRole::Value
                && context.position_kind == ScssMapPositionKind::Direct
    )
}

/// Whether a node is on the key or value side of the nearest enclosing
/// `ScssMapExpressionPair`.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ScssMapRole {
    /// The key side.
    ///
    /// Example: in `(fn((a: b)): value)`, both `fn((a: b))` and the nested
    /// `(a: b)` are on the key side.
    Key,
    /// The value side.
    ///
    /// Example: in `(key: (a, b))`, `(a, b)`, `a`, and `b` are on the value
    /// side.
    Value,
}

/// Whether a node is the pair's outer key/value expression or something nested
/// inside it.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ScssMapPositionKind {
    /// The outer key or value expression itself.
    ///
    /// Examples:
    /// - in `(key: value)`, `key` and `value` are `Direct`
    /// - in `(key: (a, b))`, `(a, b)` is `Direct`
    Direct,
    /// A descendant inside that expression.
    ///
    /// Example: in `(key: (a, b))`, `a` and `b` are `Nested`.
    Nested,
}

/// The payload wrapped by an outer parenthesized map value like
/// `key: (<payload>)`.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ScssMapOuterParenthesizedValuePayloadKind {
    /// The wrapper contains a scalar, like `key: (value)`.
    Scalar,
    /// The wrapper contains a list, like `key: (a, b)`.
    List,
    /// The wrapper contains a nested map, like
    /// `key: (other-key: other-value)`.
    Map,
}

/// Shared SCSS map context used by expression formatters.
///
/// Example: in `(key: (a, b))`, the outer `(a, b)` is `Value + Direct`, while
/// `a` and `b` are `Value + Nested`.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct ScssMapContext {
    pub(crate) role: ScssMapRole,
    pub(crate) position_kind: ScssMapPositionKind,

    /// The payload kind inside the outer wrapper in `key: (<payload>)`.
    ///
    /// When this is `Some(...)`, the current node is that outer wrapper.
    pub(crate) outer_parenthesized_value_payload_kind:
        Option<ScssMapOuterParenthesizedValuePayloadKind>,

    /// `true` for the list `a, b` in `key: (a, b)`.
    pub(crate) is_outer_parenthesized_value_list: bool,
}

/// Returns the SCSS map context for `node`, if it is inside a map pair.
///
/// Example: in `(key: (a, b))`, the outer `(a, b)` is `Value + Direct`, while
/// `a` is `Value + Nested`.
pub(crate) fn scss_map_context<N>(node: &N) -> Option<ScssMapContext>
where
    N: AstNode<Language = CssLanguage>,
{
    let (pair, role) = enclosing_pair_and_role(node)?;

    match role {
        ScssMapRole::Key => Some(ScssMapContext {
            role,
            position_kind: position_kind(node, &pair.key().ok()?),
            outer_parenthesized_value_payload_kind: None,
            is_outer_parenthesized_value_list: false,
        }),
        ScssMapRole::Value => {
            let value = pair.value().ok()?;

            let outer_parenthesized_value = outer_parenthesized_value(&value);
            let outer_parenthesized_value_payload_kind = if outer_parenthesized_value
                .as_ref()
                .is_some_and(|parenthesized| parenthesized.syntax() == node.syntax())
            {
                outer_parenthesized_value
                    .as_ref()
                    .and_then(outer_parenthesized_value_payload_kind)
            } else {
                None
            };

            let outer_parenthesized_list = outer_parenthesized_value
                .as_ref()
                .and_then(outer_parenthesized_value_list);
            let is_outer_parenthesized_value_list = outer_parenthesized_list
                .as_ref()
                .is_some_and(|list| list.syntax() == node.syntax());

            Some(ScssMapContext {
                role,
                position_kind: position_kind(node, &value),
                outer_parenthesized_value_payload_kind,
                is_outer_parenthesized_value_list,
            })
        }
    }
}

/// Finds the nearest enclosing map pair and whether `node` is on its key or
/// value side.
///
/// Descendants keep the same side. In `(fn((a: b)): value)`, the nested
/// `(a: b)` is still on the key side.
fn enclosing_pair_and_role<N>(node: &N) -> Option<(ScssMapExpressionPair, ScssMapRole)>
where
    N: AstNode<Language = CssLanguage>,
{
    let range = node.syntax().text_trimmed_range();

    node.syntax()
        .ancestors()
        .skip(1)
        .filter_map(ScssMapExpressionPair::cast)
        .find_map(|pair| {
            let is_key = pair
                .key()
                .ok()
                .is_some_and(|key| key.syntax().text_trimmed_range().contains_range(range));

            if is_key {
                return Some((pair, ScssMapRole::Key));
            }

            pair.value().ok().and_then(|value| {
                value
                    .syntax()
                    .text_trimmed_range()
                    .contains_range(range)
                    .then_some((pair, ScssMapRole::Value))
            })
        })
}

/// Classifies whether `node` is the outer expression for `expression` or a
/// descendant nested inside it.
///
/// A single-item `ScssExpression` wrapper still counts as the same outer
/// expression.
fn position_kind<N>(node: &N, expression: &AnyScssExpression) -> ScssMapPositionKind
where
    N: AstNode<Language = CssLanguage>,
{
    if expression.syntax() == node.syntax()
        || unwrap_single_expression_item(expression)
            .is_some_and(|item| item.syntax() == node.syntax())
    {
        ScssMapPositionKind::Direct
    } else {
        ScssMapPositionKind::Nested
    }
}

/// Returns the outer `( ... )` in a value like `key: (value)`.
///
/// A single-item `ScssExpression` wrapper still counts as the same outer
/// parenthesized value.
fn outer_parenthesized_value(value: &AnyScssExpression) -> Option<ScssParenthesizedExpression> {
    value
        .as_scss_parenthesized_expression()
        .cloned()
        .or_else(|| {
            unwrap_single_expression_item(value)
                .and_then(|item| item.as_scss_parenthesized_expression().cloned())
        })
}

/// Returns the list in a value like `key: (a, b)`.
fn outer_parenthesized_value_list(
    parenthesized: &ScssParenthesizedExpression,
) -> Option<ScssListExpression> {
    let expression = parenthesized.expression().ok()?;

    expression.as_scss_list_expression().cloned().or_else(|| {
        unwrap_single_expression_item(&expression)
            .and_then(|item| item.as_scss_list_expression().cloned())
    })
}

/// Classifies the payload of an outer parenthesized value wrapper.
///
/// Examples:
/// - `key: (value)` -> `Scalar`
/// - `key: (a, b)` -> `List`
/// - `key: (other-key: other-value)` -> `Map`
fn outer_parenthesized_value_payload_kind(
    parenthesized: &ScssParenthesizedExpression,
) -> Option<ScssMapOuterParenthesizedValuePayloadKind> {
    let expression = parenthesized.expression().ok()?;

    let is_map = expression.as_scss_map_expression().is_some()
        || unwrap_single_expression_item(&expression)
            .is_some_and(|item| item.as_scss_map_expression().is_some());
    if is_map {
        return Some(ScssMapOuterParenthesizedValuePayloadKind::Map);
    }

    let is_list = expression.as_scss_list_expression().is_some()
        || unwrap_single_expression_item(&expression)
            .is_some_and(|item| item.as_scss_list_expression().is_some());
    if is_list {
        return Some(ScssMapOuterParenthesizedValuePayloadKind::List);
    }

    Some(ScssMapOuterParenthesizedValuePayloadKind::Scalar)
}
