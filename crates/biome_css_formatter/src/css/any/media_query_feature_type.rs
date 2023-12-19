//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssMediaQueryFeatureType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssMediaQueryFeatureType;
impl FormatRule<AnyCssMediaQueryFeatureType> for FormatAnyCssMediaQueryFeatureType {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssMediaQueryFeatureType, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssMediaQueryFeatureType::CssMediaQueryFeaturePlain(node) => node.format().fmt(f),
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureBoolean(node) => node.format().fmt(f),
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureCompare(node) => node.format().fmt(f),
            AnyCssMediaQueryFeatureType::CssMediaQueryFeatureRange(node) => node.format().fmt(f),
        }
    }
}
