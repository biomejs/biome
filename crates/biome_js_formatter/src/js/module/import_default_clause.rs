use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsImportDefaultClause;
use biome_js_syntax::JsImportDefaultClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportDefaultClause;

impl FormatNodeRule<JsImportDefaultClause> for FormatJsImportDefaultClause {
    fn fmt_fields(&self, node: &JsImportDefaultClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportDefaultClauseFields {
            type_token,
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
