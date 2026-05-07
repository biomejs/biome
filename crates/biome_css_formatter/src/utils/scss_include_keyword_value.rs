use biome_css_syntax::{
    AnyScssExpression, ScssKeywordArgument, ScssParenthesizedExpression,
    scss_include_keyword_argument_owner,
};
use biome_rowan::AstNode;

/// Detects top-level include keyword parens.
///
/// Examples: `@include mix($arg: (a))`, `@include mix($arg: 2 * (a))`.
pub(crate) fn is_top_level_include_keyword_parenthesized_value(
    node: &ScssParenthesizedExpression,
) -> bool {
    let Some(keyword_argument) = scss_include_keyword_argument_owner(node.syntax()) else {
        return false;
    };

    !is_nested_in_keyword_parentheses(node, &keyword_argument)
}

/// Detects include keyword values containing top-level parens.
///
/// Example: `@include mix($arg: 2 * (a))`.
pub(crate) fn has_top_level_include_keyword_parenthesized_value(
    keyword_argument: &ScssKeywordArgument,
    value: &AnyScssExpression,
) -> bool {
    value
        .syntax()
        .descendants()
        .filter_map(ScssParenthesizedExpression::cast)
        .any(|parenthesized| {
            scss_include_keyword_argument_owner(parenthesized.syntax())
                .is_some_and(|owner| owner == *keyword_argument)
                && !is_nested_in_keyword_parentheses(&parenthesized, keyword_argument)
        })
}

/// Detects the inner `(a)` in `@include mix($arg: ((a)))`.
fn is_nested_in_keyword_parentheses(
    node: &ScssParenthesizedExpression,
    keyword_argument: &ScssKeywordArgument,
) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .take_while(|ancestor| ancestor != keyword_argument.syntax())
        .any(|ancestor| ScssParenthesizedExpression::can_cast(ancestor.kind()))
}
