use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsShorthandNamedImportSpecifier;
use biome_js_syntax::JsShorthandNamedImportSpecifierFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsShorthandNamedImportSpecifier;

impl FormatNodeRule<JsShorthandNamedImportSpecifier> for FormatJsShorthandNamedImportSpecifier {
    fn fmt_fields(
        &self,
        node: &JsShorthandNamedImportSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsShorthandNamedImportSpecifierFields {
            type_token,
            local_name,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space()])?;
        }

        write![f, [local_name.format()]]
    }
}
