use crate::prelude::*;
use crate::utils::comment_trivia::has_line_comment;
use biome_css_syntax::{AnyScssImportItem, ScssImportAtRule, ScssImportItemList};
use biome_formatter::write;
use biome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssImportItemList;
impl FormatRule<ScssImportItemList> for FormatScssImportItemList {
    type Context = CssFormatContext;

    /// Fills SCSS imports, breaking before comment-started rows.
    ///
    /// ```scss
    /// @import "a",
    /// // c
    /// "b";
    /// ```
    fn fmt(&self, node: &ScssImportItemList, f: &mut CssFormatter) -> FormatResult<()> {
        let mut fill = f.fill();
        let import_line_comment = has_import_line_comment(node);
        let mut prev_comment_row = false;
        let mut is_first = true;

        for (element, formatted) in node.elements().zip(node.format_separated(",")) {
            let item = element.node().ok();
            let is_comment_row =
                has_leading_line_comment(item) || (is_first && import_line_comment);
            let break_before = prev_comment_row || is_comment_row;
            let separator = format_once(move |f| {
                if break_before {
                    write!(f, [hard_line_break()])
                } else {
                    write!(f, [soft_line_break_or_space()])
                }
            });

            fill.entry(&separator, &formatted);

            prev_comment_row = is_comment_row;
            is_first = false;
        }

        fill.finish()
    }
}

/// Detects import items that start after a line comment.
///
/// ```scss
/// @import "a",
/// // c
/// "b";
/// ```
fn has_leading_line_comment(item: Option<&AnyScssImportItem>) -> bool {
    item.is_some_and(|item| {
        item.syntax()
            .first_token()
            .is_some_and(|token| has_line_comment(token.leading_trivia()))
    })
}

/// Detects line comments after the `@import` keyword.
///
/// ```scss
/// @import // c
/// "a";
/// ```
fn has_import_line_comment(node: &ScssImportItemList) -> bool {
    node.parent::<ScssImportAtRule>()
        .and_then(|rule| rule.import_token().ok())
        .is_some_and(|token| has_line_comment(token.trailing_trivia()))
}
