//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssImportLayer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssImportLayer;
impl FormatRule<AnyCssImportLayer> for FormatAnyCssImportLayer {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssImportLayer, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssImportLayer::CssImportAnonymousLayer(node) => node.format().fmt(f),
            AnyCssImportLayer::CssImportNamedLayer(node) => node.format().fmt(f),
        }
    }
}
