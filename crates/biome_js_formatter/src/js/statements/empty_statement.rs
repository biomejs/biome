use crate::prelude::*;
use biome_formatter::{write, CstFormatContext};

use biome_js_syntax::{JsEmptyStatement, JsEmptyStatementFields, JsSyntaxKind};
use biome_rowan::{AstNode, SyntaxNodeOptionExt};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsEmptyStatement;

impl FormatNodeRule<JsEmptyStatement> for FormatJsEmptyStatement {
    fn fmt_fields(&self, node: &JsEmptyStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsEmptyStatementFields { semicolon_token } = node.as_fields();
        let parent_kind = node.syntax().parent().kind();

        let leading_comments_with_break = f
            .context()
            .comments()
            .leading_comments(node.syntax())
            .iter()
            .any(|comment| comment.lines_before() > 0 || comment.kind().is_line());

        if leading_comments_with_break {
            write!(f, [hard_line_break()])?;
        }
        write!(f, [format_leading_comments(node.syntax())])?;

        if matches!(
            parent_kind,
            Some(
                JsSyntaxKind::JS_DO_WHILE_STATEMENT
                    | JsSyntaxKind::JS_IF_STATEMENT
                    | JsSyntaxKind::JS_ELSE_CLAUSE
                    | JsSyntaxKind::JS_WHILE_STATEMENT
                    | JsSyntaxKind::JS_FOR_IN_STATEMENT
                    | JsSyntaxKind::JS_FOR_OF_STATEMENT
                    | JsSyntaxKind::JS_FOR_STATEMENT
                    | JsSyntaxKind::JS_WITH_STATEMENT
            )
        ) {
            write!(f, [semicolon_token.format()])
        } else {
            write!(f, [format_removed(&semicolon_token?)])
        }
    }

    fn fmt_leading_comments(&self, _: &JsEmptyStatement, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
