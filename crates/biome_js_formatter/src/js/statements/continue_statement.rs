use crate::prelude::*;
use biome_formatter::write;

use crate::utils::FormatStatementSemicolon;

use biome_js_syntax::JsContinueStatement;
use biome_js_syntax::JsContinueStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsContinueStatement;

impl FormatNodeRule<JsContinueStatement> for FormatJsContinueStatement {
    fn fmt_fields(&self, node: &JsContinueStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsContinueStatementFields {
            continue_token,
            label,
            semicolon_token,
        } = node.as_fields();

        write!(f, [continue_token.format()])?;

        if let Some(label) = &label {
            write!(f, [space(), label.format()])?;
        }

        write!(f, [FormatStatementSemicolon::new(semicolon_token.as_ref())])
    }
}
