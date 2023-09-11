use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{JsxOpeningFragment, JsxOpeningFragmentFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxOpeningFragment;

impl FormatNodeRule<JsxOpeningFragment> for FormatJsxOpeningFragment {
    fn fmt_fields(&self, node: &JsxOpeningFragment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxOpeningFragmentFields {
            r_angle_token,
            l_angle_token,
        } = node.as_fields();

        let has_own_line_comment = f
            .comments()
            .leading_dangling_trailing_comments(node.syntax())
            .any(|comment| comment.kind().is_line());

        let format_comments = format_with(|f| {
            if has_own_line_comment {
                write!(f, [hard_line_break()])?;
            }

            write!(f, [format_dangling_comments(node.syntax())])
        });

        write![
            f,
            [
                l_angle_token.format(),
                indent(&format_comments),
                has_own_line_comment.then_some(hard_line_break()),
                r_angle_token.format()
            ]
        ]
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsxOpeningFragment,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled as part of `fmt_fields`
        Ok(())
    }
}
