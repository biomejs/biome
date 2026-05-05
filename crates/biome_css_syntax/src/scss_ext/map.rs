use crate::{
    AnyScssExpression, CssLanguage, ScssListExpression, ScssMapExpression, ScssMapExpressionPair,
    ScssParenthesizedExpression, unwrap_single_expression_item,
};
use biome_rowan::AstNode;

/// Returns `true` for the outer key expression in a SCSS map pair.
///
/// Example: in `(fn((a: b)): value)`, `fn((a: b))` returns `true`, but the
/// nested `(a: b)` returns `false`.
pub fn is_scss_map_key<N>(node: &N) -> bool
where
    N: AstNode<Language = CssLanguage>,
{
    matches!(
        ScssMapContext::from_node(node),
        Some(context)
            if context.role == ScssMapRole::Key
                && context.position_kind == ScssMapPositionKind::Direct
    )
}

/// Returns `true` when `node` is anywhere on the key side of a SCSS map pair.
///
/// Example: in `(fn((a: b)): value)`, both `fn((a: b))` and the nested
/// `(a: b)` return `true`.
pub fn is_in_scss_map_key<N>(node: &N) -> bool
where
    N: AstNode<Language = CssLanguage>,
{
    matches!(
        ScssMapContext::from_node(node),
        Some(context) if context.role == ScssMapRole::Key
    )
}

/// Returns `true` when `node` is the direct value expression of a SCSS map
/// pair.
///
/// Example: in `(key: (a, b))`, the outer `(a, b)` returns `true`, but `a`
/// does not. A single-item `ScssExpression` wrapper still counts as direct.
pub fn is_scss_map_value<N>(node: &N) -> bool
where
    N: AstNode<Language = CssLanguage>,
{
    matches!(
        ScssMapContext::from_node(node),
        Some(context)
            if context.role == ScssMapRole::Value
                && context.position_kind == ScssMapPositionKind::Direct
    )
}

/// Returns `true` for the outer value wrapper whose payload is a map.
///
/// Example: in `(key: (a: b))`, `(a: b)` returns `true`.
pub fn is_scss_map_outer_parenthesized_value_map(node: &ScssParenthesizedExpression) -> bool {
    let Some((pair, ScssMapRole::Value)) = enclosing_pair_and_role(node) else {
        return false;
    };
    let Ok(value) = pair.value() else {
        return false;
    };

    outer_parenthesized_value(&value).is_some_and(|parenthesized| {
        parenthesized.syntax() == node.syntax()
            && outer_parenthesized_value_map(&parenthesized).is_some()
    })
}

/// Returns `true` for the list payload inside an outer value wrapper.
///
/// Example: in `(key: (a, b))`, `a, b` returns `true`.
pub fn is_scss_map_outer_parenthesized_value_list(node: &ScssListExpression) -> bool {
    let Some((pair, ScssMapRole::Value)) = enclosing_pair_and_role(node) else {
        return false;
    };
    let Ok(value) = pair.value() else {
        return false;
    };

    outer_parenthesized_value(&value)
        .and_then(|parenthesized| outer_parenthesized_value_list(&parenthesized))
        .is_some_and(|list| list.syntax() == node.syntax())
}

/// Whether a node is on the key or value side of the nearest enclosing
/// `ScssMapExpressionPair`.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ScssMapRole {
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

/// Whether a node is the pair's outer key/value expression or nested inside it.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ScssMapPositionKind {
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

/// Shared SCSS map context for syntax consumers.
///
/// Example: in `(key: (a, b))`, the outer `(a, b)` is `Value + Direct`, while
/// `a` is `Value + Nested`.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ScssMapContext {
    /// Whether the node is on the pair key or value side.
    pub role: ScssMapRole,
    /// Whether the node is the outer expression or nested inside it.
    pub position_kind: ScssMapPositionKind,
}

impl ScssMapContext {
    /// Returns the SCSS map context for `node`, if it is inside a map pair.
    ///
    /// Example: in `(key: (a, b))`, the outer `(a, b)` is `Value + Direct`,
    /// while `a` is `Value + Nested`.
    pub fn from_node<N>(node: &N) -> Option<Self>
    where
        N: AstNode<Language = CssLanguage>,
    {
        let (pair, role) = enclosing_pair_and_role(node)?;

        match role {
            ScssMapRole::Key => Some(Self {
                role,
                position_kind: position_kind(node, &pair.key().ok()?),
            }),
            ScssMapRole::Value => {
                let value = pair.value().ok()?;

                Some(Self {
                    role,
                    position_kind: position_kind(node, &value),
                })
            }
        }
    }
}

/// Finds the nearest enclosing map pair and the side containing `node`.
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

/// Classifies whether `node` is the outer expression or a nested descendant.
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

/// Returns the map payload inside `key: (a: b)`.
fn outer_parenthesized_value_map(
    parenthesized: &ScssParenthesizedExpression,
) -> Option<ScssMapExpression> {
    let expression = parenthesized.expression().ok()?;

    expression.as_scss_map_expression().cloned().or_else(|| {
        unwrap_single_expression_item(&expression)
            .and_then(|item| item.as_scss_map_expression().cloned())
    })
}

/// Returns the list payload inside `key: (a, b)`.
fn outer_parenthesized_value_list(
    parenthesized: &ScssParenthesizedExpression,
) -> Option<ScssListExpression> {
    let expression = parenthesized.expression().ok()?;

    expression.as_scss_list_expression().cloned().or_else(|| {
        unwrap_single_expression_item(&expression)
            .and_then(|item| item.as_scss_list_expression().cloned())
    })
}
