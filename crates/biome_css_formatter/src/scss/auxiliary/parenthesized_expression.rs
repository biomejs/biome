use crate::prelude::*;
use crate::utils::scss_map::{
    scss_map_context, ScssMapOuterParenthesizedValuePayloadKind,
};
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
        let group_id = f.group_id("scss_parenthesized_expression");

        let outer_payload_kind =
            map_context.and_then(|context| context.outer_parenthesized_value_payload_kind);
        let is_outer_parenthesized_map_value = outer_payload_kind.is_some();

        // Only `key: ((nested: map))`-style wrappers force expansion here.
        // `key: (value)` and `key: (a, b)` keep their normal scalar/list
        // behavior unless they break for other reasons.
        let should_expand = is_outer_parenthesized_map_value
            && outer_payload_kind == Some(ScssMapOuterParenthesizedValuePayloadKind::Map);

        // `key: (value,)` is a singleton list, not the same SCSS as
        // `key: (value)`, so scalar payloads must never gain a trailing comma
        // here.
        //
        // List/map payloads already use list/map syntax, and their own
        // formatters decide whether they need a trailing comma.
        let inner_expression_owns_trailing_comma = matches!(
            outer_payload_kind,
            Some(
                ScssMapOuterParenthesizedValuePayloadKind::List
                    | ScssMapOuterParenthesizedValuePayloadKind::Map
            )
        );

        let should_print_trailing_comma = is_outer_parenthesized_map_value
            && outer_payload_kind != Some(ScssMapOuterParenthesizedValuePayloadKind::Scalar)
            && !inner_expression_owns_trailing_comma;

        let trailing_comma = format_with(|f| {
            if should_print_trailing_comma {
                write!(f, [if_group_breaks(&token(",")).with_group_id(Some(group_id))])
            } else {
                Ok(())
            }
        });

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&format_args![expression.format(), trailing_comma]),
                r_paren_token.format()
            ])
            .with_group_id(Some(group_id))
            .should_expand(should_expand)]
        )
    }
}
