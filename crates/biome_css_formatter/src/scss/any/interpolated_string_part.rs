//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssInterpolatedStringPart;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssInterpolatedStringPart;
impl FormatRule<AnyScssInterpolatedStringPart> for FormatAnyScssInterpolatedStringPart {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssInterpolatedStringPart, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssInterpolatedStringPart::ScssInterpolation(node) => node.format().fmt(f),
            AnyScssInterpolatedStringPart::ScssStringText(node) => node.format().fmt(f),
        }
    }
}
