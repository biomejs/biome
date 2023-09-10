use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatStatementSemicolon;

use biome_js_syntax::JsExportNamedClause;
use biome_js_syntax::JsExportNamedClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExportNamedClause;

impl FormatNodeRule<JsExportNamedClause> for FormatJsExportNamedClause {
    fn fmt_fields(&self, node: &JsExportNamedClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportNamedClauseFields {
            type_token,
            l_curly_token,
            specifiers,
            r_curly_token,
            semicolon_token,
        } = node.as_fields();

        if let Some(type_token) = &type_token {
            write!(f, [type_token.format(), space()])?;
        }

        write!(f, [l_curly_token.format()])?;

        if specifiers.is_empty() {
            write!(
                f,
                [format_dangling_comments(node.syntax()).with_block_indent()]
            )?;
        } else {
            write!(
                f,
                [group(&format_args![
                    soft_line_indent_or_space(&specifiers.format()),
                    soft_line_break_or_space(),
                ])]
            )?;
        }

        write!(
            f,
            [
                r_curly_token.format(),
                FormatStatementSemicolon::new(semicolon_token.as_ref())
            ]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsExportNamedClause,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled as part of `fmt_fields`
        Ok(())
    }
}
