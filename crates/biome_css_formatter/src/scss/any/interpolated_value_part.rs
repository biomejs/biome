//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssInterpolatedValuePart;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssInterpolatedValuePart;
impl FormatRule<AnyScssInterpolatedValuePart> for FormatAnyScssInterpolatedValuePart {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssInterpolatedValuePart, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssInterpolatedValuePart::AnyCssDimension(node) => node.format().fmt(f),
            AnyScssInterpolatedValuePart::CssIdentifier(node) => node.format().fmt(f),
            AnyScssInterpolatedValuePart::CssNumber(node) => node.format().fmt(f),
            AnyScssInterpolatedValuePart::ScssInterpolation(node) => node.format().fmt(f),
            AnyScssInterpolatedValuePart::ScssNamespacedVariable(node) => node.format().fmt(f),
            AnyScssInterpolatedValuePart::ScssVariable(node) => node.format().fmt(f),
        }
    }
}
