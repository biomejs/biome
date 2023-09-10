use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::JsExportNamedShorthandSpecifier;
use biome_js_syntax::JsExportNamedShorthandSpecifierFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExportNamedShorthandSpecifier;

impl FormatNodeRule<JsExportNamedShorthandSpecifier> for FormatJsExportNamedShorthandSpecifier {
    fn fmt_fields(
        &self,
        node: &JsExportNamedShorthandSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsExportNamedShorthandSpecifierFields { type_token, name } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space()])?;
        }

        write![f, [name.format()]]
    }
}
