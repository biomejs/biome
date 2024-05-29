//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssComposesImportSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssComposesImportSource;
impl FormatRule<AnyCssComposesImportSource> for FormatAnyCssComposesImportSource {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssComposesImportSource, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssComposesImportSource::CssIdentifier(node) => node.format().fmt(f),
            AnyCssComposesImportSource::CssString(node) => node.format().fmt(f),
        }
    }
}
