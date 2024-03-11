//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssQueryFeature;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssQueryFeature;
impl FormatRule<AnyCssQueryFeature> for FormatAnyCssQueryFeature {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssQueryFeature, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssQueryFeature::CssQueryFeatureBoolean(node) => node.format().fmt(f),
            AnyCssQueryFeature::CssQueryFeaturePlain(node) => node.format().fmt(f),
            AnyCssQueryFeature::CssQueryFeatureRange(node) => node.format().fmt(f),
            AnyCssQueryFeature::CssQueryFeatureRangeInterval(node) => node.format().fmt(f),
            AnyCssQueryFeature::CssQueryFeatureReverseRange(node) => node.format().fmt(f),
        }
    }
}
