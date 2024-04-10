use crate::prelude::*;
use biome_formatter::write;

use crate::utils::format_node_without_comments::FormatAnyJsExpressionWithoutComments;
use biome_js_syntax::{JsxSpreadChild, JsxSpreadChildFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxSpreadChild;

impl FormatNodeRule<JsxSpreadChild> for FormatJsxSpreadChild {
    fn fmt_fields(&self, node: &JsxSpreadChild, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxSpreadChildFields {
            l_curly_token,
            dotdotdot_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        let expression = expression?;

        let format_inner = format_with(|f| {
            if f.comments().is_suppressed(expression.syntax()) {
                write!(
                    f,
                    [
                        dotdotdot_token.format(),
                        expression.format(),
                        line_suffix_boundary()
                    ]
                )
            } else {
                write!(
                    f,
                    [
                        format_leading_comments(expression.syntax()),
                        dotdotdot_token.format(),
                    ]
                )?;
                FormatAnyJsExpressionWithoutComments.fmt(&expression, f)?;
                write!(
                    f,
                    [
                        format_dangling_comments(expression.syntax()).with_soft_block_indent(),
                        format_trailing_comments(expression.syntax()),
                        line_suffix_boundary()
                    ]
                )
            }
        });

        write!(f, [l_curly_token.format()])?;

        if f.comments().has_comments(expression.syntax()) {
            write!(f, [soft_block_indent(&format_inner)])?;
        } else {
            write!(f, [format_inner])?;
        }

        write!(f, [r_curly_token.format()])
    }
}
