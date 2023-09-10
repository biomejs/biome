use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::JsExport;
use biome_js_syntax::JsExportFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExport;

impl FormatNodeRule<JsExport> for FormatJsExport {
    fn fmt_fields(&self, node: &JsExport, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportFields {
            decorators,
            export_token,
            export_clause,
        } = node.as_fields();

        write![
            f,
            [
                decorators.format(),
                export_token.format(),
                space(),
                export_clause.format()
            ]
        ]
    }
}
