use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsNamespaceImportSpecifier;
use biome_js_syntax::JsNamespaceImportSpecifierFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsNamespaceImportSpecifier;

impl FormatNodeRule<JsNamespaceImportSpecifier> for FormatJsNamespaceImportSpecifier {
    fn fmt_fields(
        &self,
        node: &JsNamespaceImportSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsNamespaceImportSpecifierFields {
            star_token,
            as_token,
            local_name,
        } = node.as_fields();

        write![
            f,
            [
                star_token.format(),
                space(),
                as_token.format(),
                space(),
                local_name.format()
            ]
        ]
    }
}
