use crate::prelude::*;
use biome_formatter::{Buffer, CstFormatContext, write};
use biome_js_syntax::JsBlockStatement;
use biome_js_syntax::{AnyJsStatement, JsEmptyStatement};

use biome_js_syntax::JsBlockStatementFields;
use biome_js_syntax::JsSyntaxKind;
use biome_rowan::{AstNode, AstNodeList, SyntaxNodeOptionExt};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBlockStatement;

impl FormatNodeRule<JsBlockStatement> for FormatJsBlockStatement {
    fn fmt_fields(&self, node: &JsBlockStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsBlockStatementFields {
            l_curly_token,
            statements,
            r_curly_token,
        } = node.as_fields();

        write!(f, [l_curly_token.format()])?;

        let r_curly_token = r_curly_token?;

        let comments = f.context().comments();
        if is_empty_block(node, comments) {
            let has_dangling_comments = comments.has_dangling_comments(node.syntax());

            for stmt in statements
                .iter()
                .filter_map(|stmt| JsEmptyStatement::cast(stmt.into_syntax()))
            {
                f.state_mut().track_token(&stmt.semicolon_token()?)
            }

            if has_dangling_comments {
                write!(
                    f,
                    [format_dangling_comments(node.syntax()).with_block_indent()]
                )?;
            } else if is_non_collapsible(node) {
                write!(f, [hard_line_break()])?;
            }
        } else {
            write!(f, [block_indent(&statements.format())])?;
        }

        write!(f, [r_curly_token.format()])
    }

    fn fmt_dangling_comments(&self, _: &JsBlockStatement, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}

fn is_empty_block(block: &JsBlockStatement, comments: &JsComments) -> bool {
    // add extra branch to avoid formatting the same code twice and generating different code,
    // here is an example:
    // ```js
    //     try
    // /* missing comment */
    // {;}
    // finally {}
    // ```
    // if we don't add the extra branch, this function will return false, because  `block.statement` has one empty statement,
    // and would be formatted as :
    // ```js
    //     try
    // /* missing comment */
    // {}
    // finally {}
    // ```
    // for the second time, the function would return true, because the block is empty and `parent.syntax.kind` is  `JS_TRY_FINALLY_STATEMENT`, which would hit the branch `Some(_) => true`,
    // finally the code would be formatted as:
    // ```js
    // try
    /* missing comment */
    // {
    // } finally {
    // }
    // ```
    block.statements().is_empty()
        || block.statements().iter().all(|s| {
            matches!(s, AnyJsStatement::JsEmptyStatement(_))
                && !comments.has_comments(s.syntax())
                && !comments.is_suppressed(s.syntax())
        })
}

// Formatting of curly braces for an:
// * empty block: same line `{}`,
// * empty block that is the 'cons' or 'alt' of an if statement: two lines `{\n}`
// * non empty block: put each stmt on its own line: `{\nstmt1;\nstmt2;\n}`
// * non empty block with comments (trailing comments on {, or leading comments on })
fn is_non_collapsible(block: &JsBlockStatement) -> bool {
    // reference https://github.com/prettier/prettier/blob/b188c905cfaeb238a122b4a95c230da83f2f3226/src/language-js/print/block.js#L19
    let parent = block.syntax().parent();
    match parent.kind() {
        Some(
            JsSyntaxKind::JS_FUNCTION_BODY
            | JsSyntaxKind::JS_FOR_STATEMENT
            | JsSyntaxKind::JS_WHILE_STATEMENT
            | JsSyntaxKind::JS_DO_WHILE_STATEMENT
            | JsSyntaxKind::TS_MODULE_DECLARATION
            | JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION,
        ) => false,
        // prettier collapse the catch block when it don't have `finalizer`, insert a new line when it has `finalizer`
        Some(JsSyntaxKind::JS_CATCH_CLAUSE) => {
            // SAFETY: since parent node have `Some(kind)`, this must not be `None`
            let parent_unwrap = parent.unwrap();
            let finally_clause = parent_unwrap.next_sibling();
            matches!(
                finally_clause.map(|finally| finally.kind()),
                Some(JsSyntaxKind::JS_FINALLY_CLAUSE),
            )
        }
        Some(_) => true,
        None => false,
    }
}
