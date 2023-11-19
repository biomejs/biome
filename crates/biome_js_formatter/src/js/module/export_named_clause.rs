use crate::prelude::*;
use biome_formatter::write;

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
            let should_insert_space_around_brackets = f.options().bracket_spacing().value();
            write!(
                f,
                [group(&soft_block_indent_with_maybe_space(
                    &specifiers.format(),
                    should_insert_space_around_brackets
                ),)]
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
