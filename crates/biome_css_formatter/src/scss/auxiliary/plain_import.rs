use crate::prelude::*;
use crate::utils::import::FormatImportClause;
use biome_css_syntax::{ScssPlainImport, ScssPlainImportFields};
use biome_formatter::write;

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

        let import_clause = FormatImportClause::new(url, layer, supports, media);

        write!(f, [group(&import_clause)])
    }
}
