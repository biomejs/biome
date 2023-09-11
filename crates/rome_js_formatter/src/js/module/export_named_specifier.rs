use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::JsExportNamedSpecifier;
use biome_js_syntax::JsExportNamedSpecifierFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExportNamedSpecifier;

impl FormatNodeRule<JsExportNamedSpecifier> for FormatJsExportNamedSpecifier {
    fn fmt_fields(&self, node: &JsExportNamedSpecifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportNamedSpecifierFields {
            type_token,
            local_name,
            as_token,
            exported_name,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space()])?;
        }

        write![
            f,
            [
                local_name.format(),
                space(),
                as_token.format(),
                space(),
                exported_name.format()
            ]
        ]
    }
}
