use crate::prelude::*;

use crate::utils::{FormatStatementBody, FormatStatementSemicolon};
use biome_formatter::{format_args, write};
use biome_js_syntax::JsDoWhileStatementFields;
use biome_js_syntax::{AnyJsStatement, JsDoWhileStatement};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsDoWhileStatement;

impl FormatNodeRule<JsDoWhileStatement> for FormatJsDoWhileStatement {
    fn fmt_fields(&self, node: &JsDoWhileStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDoWhileStatementFields {
            do_token,
            body,
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            semicolon_token,
        } = node.as_fields();

        let body = body?;
        let l_paren_token = l_paren_token?;
        let r_paren_token = r_paren_token?;

        write!(
            f,
            [group(&format_args![
                do_token.format(),
                FormatStatementBody::new(&body)
            ])]
        )?;

        if matches!(body, AnyJsStatement::JsBlockStatement(_)) {
            write!(f, [space()])?;
        } else {
            write!(f, [hard_line_break()])?;
        }

        write!(
            f,
            [
                while_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&test.format())),
                r_paren_token.format(),
                FormatStatementSemicolon::new(semicolon_token.as_ref())
            ]
        )
    }
}
