use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{JsxClosingElement, JsxClosingElementFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxClosingElement;

impl FormatNodeRule<JsxClosingElement> for FormatJsxClosingElement {
    fn fmt_fields(&self, node: &JsxClosingElement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxClosingElementFields {
            l_angle_token,
            slash_token,
            name,
            r_angle_token,
        } = node.as_fields();
        let name = name?;

        let mut name_has_leading_comment = false;
        let mut name_has_own_line_leading_comment = false;
        for leading_comment in f.comments().leading_comments(name.syntax()) {
            name_has_leading_comment = true;
            name_has_own_line_leading_comment =
                name_has_own_line_leading_comment || leading_comment.kind().is_line()
        }

        let format_name = format_with(|f| {
            if name_has_own_line_leading_comment {
                write!(f, [hard_line_break()])?;
            } else if name_has_leading_comment {
                write!(f, [space()])?;
            }
            if name_has_own_line_leading_comment {
                write!(f, [block_indent(&name.format()), hard_line_break()])
            } else {
                write!(f, [name.format()])
            }
        });

        write![
            f,
            [
                l_angle_token.format(),
                slash_token.format(),
                &format_name,
                line_suffix_boundary(),
                r_angle_token.format(),
            ]
        ]
    }
}
