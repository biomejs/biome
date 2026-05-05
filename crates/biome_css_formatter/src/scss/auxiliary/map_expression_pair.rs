use crate::prelude::*;
use crate::utils::scss_expression::is_self_breaking_value;
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

        let is_self_breaking = value.as_ref().is_ok_and(is_self_breaking_value);

        write!(
            f,
            [group(&format_args![
                key.format(),
                colon_token.format(),
                FormatScssMapExpressionPairValue {
                    value: &value.format(),
                    is_self_breaking,
                }
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

/// Formats the value in `key: value`.
///
/// Self-breaking values such as `key: (a, b)` stay after the colon; scalar
/// values may break as `key:\n  value`.
struct FormatScssMapExpressionPairValue<'a> {
    value: &'a dyn Format<CssFormatContext>,
    is_self_breaking: bool,
}

impl Format<CssFormatContext> for FormatScssMapExpressionPairValue<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        if self.is_self_breaking {
            write!(f, [space(), self.value])
        } else {
            write!(
                f,
                [indent(&format_args![
                    soft_line_break_or_space(),
                    self.value
                ])]
            )
        }
    }
}
