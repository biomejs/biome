use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsImportCombinedClause;
use biome_js_syntax::JsImportCombinedClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportCombinedClause;

impl FormatNodeRule<JsImportCombinedClause> for FormatJsImportCombinedClause {
    fn fmt_fields(&self, node: &JsImportCombinedClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportCombinedClauseFields {
            default_specifier,
            comma_token,
            specifier,
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
                specifier.format(),
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
