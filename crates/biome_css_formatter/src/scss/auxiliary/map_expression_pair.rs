use crate::prelude::*;
use crate::utils::scss_context::is_in_scss_include_arguments;
use crate::utils::scss_expression::is_self_breaking_value;
use crate::utils::scss_map::{is_in_scss_map_key, is_scss_map_value};
use crate::utils::scss_separator_comments::FormatScssSeparatorComments;
use biome_css_syntax::{ScssMapExpressionPair, ScssMapExpressionPairFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMapExpressionPair;
impl FormatNodeRule<ScssMapExpressionPair> for FormatScssMapExpressionPair {
    fn fmt_node(&self, node: &ScssMapExpressionPair, f: &mut CssFormatter) -> FormatResult<()> {
        self.fmt_node_with_scss_separator_comments(node, f)
    }

    fn fmt_fields(&self, node: &ScssMapExpressionPair, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMapExpressionPairFields {
            key,
            colon_token,
            value,
        } = node.as_fields();

        let child_value_is_self_breaking = value.as_ref().is_ok_and(is_self_breaking_value);

        // Pairs inside a map key may wrap after `:` so long keys do not push a
        // scalar value too far to the right.
        let allow_scalar_value_wrap =
            is_in_scss_map_key(node) || is_in_scss_include_arguments(node.syntax());
        let value_is_direct_map_value = value.as_ref().ok().is_some_and(is_scss_map_value);

        let formatted_value = format_with(|f| {
            if child_value_is_self_breaking {
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

    fn fmt_leading_comments(
        &self,
        node: &ScssMapExpressionPair,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        self.fmt_leading_scss_separator_comments(node, f)
    }
}
