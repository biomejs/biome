use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsImportNamespaceClause;
use biome_js_syntax::JsImportNamespaceClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportNamespaceClause;

impl FormatNodeRule<JsImportNamespaceClause> for FormatJsImportNamespaceClause {
    fn fmt_fields(&self, node: &JsImportNamespaceClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportNamespaceClauseFields {
            type_token,
            star_token,
            as_token,
            local_name,
            from_token,
            source,
            assertion,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space()])?;
        }

        write![
            f,
            [
                star_token.format(),
                space(),
                as_token.format(),
                space(),
                local_name.format(),
                space(),
                from_token.format(),
                space(),
                source.format(),
            ]
        ]?;

        if let Some(assertion) = assertion {
            write!(f, [space(), assertion.format()])?;
        }

        Ok(())
    }
}
