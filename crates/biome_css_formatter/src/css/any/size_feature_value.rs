//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSizeFeatureValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSizeFeatureValue;
impl FormatRule<AnyCssSizeFeatureValue> for FormatAnyCssSizeFeatureValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSizeFeatureValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSizeFeatureValue::CssNumber(node) => node.format().fmt(f),
            AnyCssSizeFeatureValue::AnyCssDimension(node) => node.format().fmt(f),
            AnyCssSizeFeatureValue::CssIdentifier(node) => node.format().fmt(f),
            AnyCssSizeFeatureValue::CssRatio(node) => node.format().fmt(f),
            AnyCssSizeFeatureValue::CssAnyFunction(node) => node.format().fmt(f),
        }
    }
}
