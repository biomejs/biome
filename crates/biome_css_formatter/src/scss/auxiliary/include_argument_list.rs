use crate::prelude::*;
use crate::utils::scss_closing_comments::{
    ScssIncludeClosingCommentSpacing, owns_include_closing_comments, write_include_closing_comments,
};
use biome_css_syntax::{ScssIncludeArgumentList, ScssIncludeArgumentListFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssIncludeArgumentList;

impl FormatNodeRule<ScssIncludeArgumentList> for FormatScssIncludeArgumentList {
    fn fmt_fields(&self, node: &ScssIncludeArgumentList, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssIncludeArgumentListFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();
        let closing_comments = format_with(|f| {
            write_include_closing_comments(
                node.syntax(),
                ScssIncludeClosingCommentSpacing::AdaptiveSpace,
                f,
            )
        });

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                indent(&format_args![
                    soft_line_break(),
                    items.format(),
                    closing_comments
                ]),
                soft_line_break(),
                r_paren_token.format()
            ])]
        )
    }

    fn fmt_dangling_comments(
        &self,
        node: &ScssIncludeArgumentList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        // `fmt_fields` prints comments before the closing `)` itself so they
        // stay inside `@include (...)`.
        if owns_include_closing_comments(node.syntax(), f) {
            Ok(())
        } else {
            format_dangling_comments(node.syntax()).fmt(f)
        }
    }
}
