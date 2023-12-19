use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::JsDefaultImportSpecifier;
use biome_js_syntax::JsDefaultImportSpecifierFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsDefaultImportSpecifier;

impl FormatNodeRule<JsDefaultImportSpecifier> for FormatJsDefaultImportSpecifier {
    fn fmt_fields(&self, node: &JsDefaultImportSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDefaultImportSpecifierFields { local_name } = node.as_fields();
        write![f, [local_name.format()]]
    }
}
