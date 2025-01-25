use crate::prelude::*;
use biome_formatter::{format_args, write, CstFormatContext};

use crate::utils::FormatStatementBody;
use biome_js_syntax::JsIfStatement;
use biome_js_syntax::JsIfStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsIfStatement;

impl FormatNodeRule<JsIfStatement> for FormatJsIfStatement {
    fn fmt_fields(&self, node: &JsIfStatement, f: &mut JsFormatter) -> FormatResult<()> {
        use biome_js_syntax::AnyJsStatement::*;

        let JsIfStatementFields {
            if_token,
            l_paren_token,
            test,
            r_paren_token,
            consequent,
            else_clause,
        } = node.as_fields();

        let l_paren_token = l_paren_token?;
        let r_paren_token = r_paren_token?;
        let consequent = consequent?;

        write!(
            f,
            [group(&format_args![
                if_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&test.format())),
                r_paren_token.format(),
                FormatStatementBody::new(&consequent),
            ]),]
        )?;

        if let Some(else_clause) = else_clause {
            let comments = f.context().comments();
            let dangling_comments = comments.dangling_comments(node.syntax());
            let dangling_line_comment = dangling_comments
                .last()
                .is_some_and(|comment| comment.kind().is_line());
            let has_dangling_comments = !dangling_comments.is_empty();

            let trailing_line_comment = comments
                .trailing_comments(consequent.syntax())
                .iter()
                .any(|comment| comment.kind().is_line());

            let else_on_same_line = matches!(consequent, JsBlockStatement(_))
                && !trailing_line_comment
                && !dangling_line_comment;

            if else_on_same_line {
                write!(f, [space()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }

            if has_dangling_comments {
                write!(f, [format_dangling_comments(node.syntax())])?;

                if trailing_line_comment || dangling_line_comment {
                    write!(f, [hard_line_break()])?
                } else {
                    write!(f, [space()])?;
                }
            }

            write!(f, [else_clause.format()])?;
        }

        Ok(())
    }

    fn fmt_dangling_comments(&self, _: &JsIfStatement, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
