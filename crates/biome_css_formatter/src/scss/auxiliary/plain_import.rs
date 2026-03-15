use crate::prelude::*;
use crate::utils::import::write_import_payload;
use biome_css_syntax::{ScssPlainImport, ScssPlainImportFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssPlainImport;
impl FormatNodeRule<ScssPlainImport> for FormatScssPlainImport {
    fn fmt_fields(&self, node: &ScssPlainImport, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssPlainImportFields {
            url,
            layer,
            supports,
            media,
        } = node.as_fields();

        write_import_payload(f, &url, layer.as_ref(), supports.as_ref(), &media)
    }
}
