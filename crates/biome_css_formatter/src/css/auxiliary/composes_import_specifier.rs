use crate::prelude::*;
use biome_css_syntax::{CssComposesImportSpecifier, CssComposesImportSpecifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComposesImportSpecifier;
impl FormatNodeRule<CssComposesImportSpecifier> for FormatCssComposesImportSpecifier {
    fn fmt_fields(
        &self,
        node: &CssComposesImportSpecifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssComposesImportSpecifierFields { from_token, source } = node.as_fields();

        write![f, [space(), from_token.format(), space(), source.format()]]
    }
}
