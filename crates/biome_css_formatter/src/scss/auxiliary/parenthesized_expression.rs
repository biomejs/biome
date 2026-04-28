use crate::prelude::*;
use crate::utils::scss_expression::{
    include_keyword_argument_before_argument_list, unwrap_single_expression_item,
};
use crate::utils::scss_map::{ScssMapOuterParenthesizedValuePayloadKind, scss_map_context};
use biome_css_syntax::{ScssParenthesizedExpression, ScssParenthesizedExpressionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssParenthesizedExpression;

impl FormatNodeRule<ScssParenthesizedExpression> for FormatScssParenthesizedExpression {
    fn fmt_fields(
        &self,
        node: &ScssParenthesizedExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = node.as_fields();
        let map_context = scss_map_context(node);

        let outer_payload_kind =
            map_context.and_then(|context| context.outer_parenthesized_value_payload_kind);
        // Only `key: ((nested: map))`-style wrappers force expansion here.
        // `key: (value)` and `key: (a, b)` keep their normal scalar/list
        // behavior unless they break for other reasons.
        let has_nested_parenthesized_include_item = has_nested_parenthesized_include_item(node);
        let should_expand = outer_payload_kind
            == Some(ScssMapOuterParenthesizedValuePayloadKind::Map)
            || has_nested_parenthesized_include_item;
        let trailing_comma = has_nested_parenthesized_include_item.then_some(token(","));

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&format_args![expression.format(), trailing_comma]),
                r_paren_token.format()
            ])
            .should_expand(should_expand)]
        )
    }
}

/// Returns `true` for the outer value in `@include mix($arg: ((a)))`.
fn has_nested_parenthesized_include_item(node: &ScssParenthesizedExpression) -> bool {
    if include_keyword_argument_before_argument_list(node.syntax()).is_none() {
        return false;
    }

    node.expression().ok().is_some_and(|expression| {
        expression.as_scss_parenthesized_expression().is_some()
            || unwrap_single_expression_item(&expression)
                .is_some_and(|item| item.as_scss_parenthesized_expression().is_some())
    })
}
