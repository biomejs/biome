use crate::prelude::*;
use rome_formatter::write;

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
            write!(
                f,
                [
                    dotdotdot_token.format(),
                    expression.format(),
                    line_suffix_boundary()
                ]
            )
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
