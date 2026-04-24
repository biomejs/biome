use crate::prelude::*;
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
        let is_outer_parenthesized_map_value = outer_payload_kind.is_some();

        // Only `key: ((nested: map))`-style wrappers force expansion here.
        // `key: (value)` and `key: (a, b)` keep their normal scalar/list
        // behavior unless they break for other reasons.
        let should_expand = is_outer_parenthesized_map_value
            && outer_payload_kind == Some(ScssMapOuterParenthesizedValuePayloadKind::Map);

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&expression.format()),
                r_paren_token.format()
            ])
            .should_expand(should_expand)]
        )
    }
}
