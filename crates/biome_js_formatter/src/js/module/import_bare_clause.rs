use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsImportBareClause;
use biome_js_syntax::JsImportBareClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportBareClause;

impl FormatNodeRule<JsImportBareClause> for FormatJsImportBareClause {
    fn fmt_fields(&self, node: &JsImportBareClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportBareClauseFields { source, attribute } = node.as_fields();

        write!(f, [source.format()])?;

        if let Some(attribute) = attribute {
            write!(f, [attribute.format()])?;
        }

        Ok(())
    }
}
