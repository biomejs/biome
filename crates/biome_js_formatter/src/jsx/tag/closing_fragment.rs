use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{JsxClosingFragment, JsxClosingFragmentFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxClosingFragment;

impl FormatNodeRule<JsxClosingFragment> for FormatJsxClosingFragment {
    fn fmt_fields(&self, node: &JsxClosingFragment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxClosingFragmentFields {
            r_angle_token,
            slash_token,
            l_angle_token,
        } = node.as_fields();

        let mut has_own_line_comment = false;
        let mut has_comment = false;

        for comment in f
            .comments()
            .leading_dangling_trailing_comments(node.syntax())
        {
            has_comment = true;
            has_own_line_comment = has_own_line_comment || comment.kind().is_line()
        }

        let format_comments = format_with(|f| {
            if has_own_line_comment {
                write!(f, [hard_line_break()])?;
            } else if has_comment {
                write!(f, [space()])?;
            }

            write!(f, [format_dangling_comments(node.syntax())])
        });

        write![
            f,
            [
                l_angle_token.format(),
                slash_token.format(),
                indent(&format_comments),
                has_own_line_comment.then_some(hard_line_break()),
                r_angle_token.format(),
            ]
        ]
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsxClosingFragment,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }
}
