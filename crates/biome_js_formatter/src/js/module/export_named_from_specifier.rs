use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::JsExportNamedFromSpecifier;
use biome_js_syntax::JsExportNamedFromSpecifierFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExportNamedFromSpecifier;

impl FormatNodeRule<JsExportNamedFromSpecifier> for FormatJsExportNamedFromSpecifier {
    fn fmt_fields(
        &self,
        node: &JsExportNamedFromSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsExportNamedFromSpecifierFields {
            type_token,
            source_name,
            export_as,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space()])?;
        }

        write!(f, [source_name.format()])?;

        if let Some(export_as) = export_as {
            write!(f, [space(), export_as.format()])?;
        }

        Ok(())
    }
}
