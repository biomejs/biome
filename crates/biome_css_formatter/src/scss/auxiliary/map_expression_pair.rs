use crate::prelude::*;
use crate::utils::scss_expression::unwrap_single_expression_item;
use crate::utils::scss_map::{is_in_scss_map_key, is_scss_map_value};
use biome_css_syntax::{AnyScssExpression, ScssMapExpressionPair, ScssMapExpressionPairFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMapExpressionPair;
impl FormatNodeRule<ScssMapExpressionPair> for FormatScssMapExpressionPair {
    fn fmt_fields(&self, node: &ScssMapExpressionPair, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMapExpressionPairFields {
            key,
            colon_token,
            value,
        } = node.as_fields();

        let child_value_manages_its_own_breaking =
            value.as_ref().is_ok_and(value_manages_its_own_breaking);

        // Pairs inside a map key may wrap after `:` so long keys do not push a
        // scalar value too far to the right.
        let allow_scalar_value_wrap = is_in_scss_map_key(node);
        let value_is_direct_map_value = value.as_ref().ok().is_some_and(is_scss_map_value);

        let formatted_value = format_with(|f| {
            if child_value_manages_its_own_breaking {
                // Keep `key: (` on one line and let the value formatter decide
                // its own internal breaks.
                write!(f, [space(), value.format()])
            } else if allow_scalar_value_wrap && value_is_direct_map_value {
                // Allow long keys to wrap a scalar value after the colon:
                //
                // "very long key":
                //   "value",
                write!(
                    f,
                    [indent(&format_args![
                        soft_line_break_or_space(),
                        value.format()
                    ])]
                )
            } else {
                write!(f, [space(), value.format()])
            }
        });

        write!(
            f,
            [group(&format_args![
                key.format(),
                group(&format_args![colon_token.format(), formatted_value])
            ])]
        )
    }
}

/// Returns `true` when the expression value can break internally without
/// needing the enclosing `key: value` pair to also break after the colon.
///
/// Parenthesized, list, and map values own their internal layout, so the pair
/// formatter keeps `key: (` on the same line and lets the child decide where to
/// break.
fn value_manages_its_own_breaking(value: &AnyScssExpression) -> bool {
    matches!(
        value,
        AnyScssExpression::ScssListExpression(_)
            | AnyScssExpression::ScssMapExpression(_)
            | AnyScssExpression::ScssParenthesizedExpression(_)
    ) || unwrap_single_expression_item(value).is_some_and(|item| {
        matches!(
            item,
            biome_css_syntax::AnyScssExpressionItem::ScssListExpression(_)
                | biome_css_syntax::AnyScssExpressionItem::ScssMapExpression(_)
                | biome_css_syntax::AnyScssExpressionItem::ScssParenthesizedExpression(_)
        )
    })
}
