use crate::prelude::*;

use biome_js_syntax::TsImportTypeQualifier;
use biome_js_syntax::TsImportTypeQualifierFields;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsImportTypeQualifier;

impl FormatNodeRule<TsImportTypeQualifier> for FormatTsImportTypeQualifier {
    fn fmt_fields(&self, node: &TsImportTypeQualifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsImportTypeQualifierFields { dot_token, right } = node.as_fields();

        write![f, [dot_token.format(), right.format()]]
    }
}
