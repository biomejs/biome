use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::JsTryFinallyStatement;
use biome_js_syntax::JsTryFinallyStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsTryFinallyStatement;

impl FormatNodeRule<JsTryFinallyStatement> for FormatJsTryFinallyStatement {
    fn fmt_fields(&self, node: &JsTryFinallyStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsTryFinallyStatementFields {
            try_token,
            body,
            catch_clause,
            finally_clause,
        } = node.as_fields();

        write![f, [try_token.format(), space(), body.format()]]?;

        if let Some(catch_clause) = catch_clause {
            write!(f, [space(), catch_clause.format()])?;
        }

        write!(f, [space(), finally_clause.format()])
    }
}
