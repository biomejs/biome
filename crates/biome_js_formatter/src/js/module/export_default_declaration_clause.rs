use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{JsExportDefaultDeclarationClause, JsExportDefaultDeclarationClauseFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExportDefaultDeclarationClause;

impl FormatNodeRule<JsExportDefaultDeclarationClause> for FormatJsExportDefaultDeclarationClause {
    fn fmt_fields(
        &self,
        node: &JsExportDefaultDeclarationClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsExportDefaultDeclarationClauseFields {
            default_token,
            declaration,
            semicolon_token: _,
        } = node.as_fields();

        write![f, [default_token.format(), space(), declaration.format()]]
    }
}
