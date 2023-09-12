use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::TsExportDeclareClause;
use biome_js_syntax::TsExportDeclareClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsExportDeclareClause;

impl FormatNodeRule<TsExportDeclareClause> for FormatTsExportDeclareClause {
    fn fmt_fields(&self, node: &TsExportDeclareClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsExportDeclareClauseFields {
            declare_token,
            declaration,
        } = node.as_fields();

        write![f, [declare_token.format(), space(), declaration.format(),]]
    }
}
