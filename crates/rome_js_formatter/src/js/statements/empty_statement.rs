use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::{JsEmptyStatement, JsEmptyStatementFields, JsSyntaxKind};
use biome_rowan::{AstNode, SyntaxNodeOptionExt};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsEmptyStatement;

impl FormatNodeRule<JsEmptyStatement> for FormatJsEmptyStatement {
    fn fmt_fields(&self, node: &JsEmptyStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsEmptyStatementFields { semicolon_token } = node.as_fields();
        let parent_kind = node.syntax().parent().kind();

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
}
