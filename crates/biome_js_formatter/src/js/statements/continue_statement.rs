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

    fn fmt_dangling_comments(
        &self,
        node: &JsContinueStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        if !f.comments().has_dangling_comments(node.syntax()) {
            return Ok(());
        }
        let content =
            format_with(|f| write!(f, [space(), format_dangling_comments(node.syntax())]));
        write!(f, [line_suffix(&content), expand_parent()])
    }
}
