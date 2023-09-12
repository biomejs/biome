use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsFinallyClause;
use biome_js_syntax::JsFinallyClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsFinallyClause;

impl FormatNodeRule<JsFinallyClause> for FormatJsFinallyClause {
    fn fmt_fields(&self, node: &JsFinallyClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsFinallyClauseFields {
            finally_token,
            body,
        } = node.as_fields();

        write![f, [finally_token.format(), space(), body.format()]]
    }
}
