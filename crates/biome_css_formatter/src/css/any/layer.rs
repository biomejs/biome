//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssLayer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssLayer;
impl FormatRule<AnyCssLayer> for FormatAnyCssLayer {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssLayer, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssLayer::CssBogusLayer(node) => node.format().fmt(f),
            AnyCssLayer::CssLayerDeclaration(node) => node.format().fmt(f),
            AnyCssLayer::CssLayerReference(node) => node.format().fmt(f),
        }
    }
}
