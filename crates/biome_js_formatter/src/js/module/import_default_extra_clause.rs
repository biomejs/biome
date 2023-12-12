use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsImportDefaultExtraClause;
use biome_js_syntax::JsImportDefaultExtraClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportDefaultExtraClause;

impl FormatNodeRule<JsImportDefaultExtraClause> for FormatJsImportDefaultExtraClause {
    fn fmt_fields(
        &self,
        node: &JsImportDefaultExtraClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsImportDefaultExtraClauseFields {
            default_specifier,
            comma_token,
            extra_specifier,
            from_token,
            source,
            assertion,
        } = node.as_fields();
        write![
            f,
            [
                default_specifier.format(),
                comma_token.format(),
                space(),
                extra_specifier.format(),
                space(),
                from_token.format(),
                space(),
                source.format(),
            ]
        ]?;

        if let Some(assertion) = assertion {
            write!(f, [assertion.format()])?;
        }

        Ok(())
    }
}
