//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssInterpolatedNthValuePart;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssInterpolatedNthValuePart;
impl FormatRule<AnyScssInterpolatedNthValuePart> for FormatAnyScssInterpolatedNthValuePart {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyScssInterpolatedNthValuePart,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyScssInterpolatedNthValuePart::CssNumber(node) => node.format().fmt(f),
            AnyScssInterpolatedNthValuePart::ScssInterpolation(node) => node.format().fmt(f),
        }
    }
}
