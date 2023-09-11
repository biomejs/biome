use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::JsCatchClause;
use biome_js_syntax::JsCatchClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsCatchClause;

impl FormatNodeRule<JsCatchClause> for FormatJsCatchClause {
    fn fmt_fields(&self, node: &JsCatchClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCatchClauseFields {
            catch_token,
            declaration,
            body,
        } = node.as_fields();

        write!(f, [catch_token.format(), space()])?;

        if let Some(declaration) = declaration {
            write![f, [declaration.format(), space()]]?;
        }

        write!(f, [body.format()])
    }
}
