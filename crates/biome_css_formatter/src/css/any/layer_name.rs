//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssLayerName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssLayerName;
impl FormatRule<AnyCssLayerName> for FormatAnyCssLayerName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssLayerName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssLayerName::CssIdentifier(node) => node.format().fmt(f),
            AnyCssLayerName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
        }
    }
}
