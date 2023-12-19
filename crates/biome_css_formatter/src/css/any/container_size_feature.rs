//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerSizeFeature;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerSizeFeature;
impl FormatRule<AnyCssContainerSizeFeature> for FormatAnyCssContainerSizeFeature {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssContainerSizeFeature, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssContainerSizeFeature::CssSizeFeaturePlain(node) => node.format().fmt(f),
            AnyCssContainerSizeFeature::CssSizeFeatureBoolean(node) => node.format().fmt(f),
            AnyCssContainerSizeFeature::CssSizeFeatureRange(node) => node.format().fmt(f),
            AnyCssContainerSizeFeature::CssSizeFeatureRangeInterval(node) => node.format().fmt(f),
        }
    }
}
