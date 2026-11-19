//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssInterpolatedUrlValuePart;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssInterpolatedUrlValuePart;
impl FormatRule<AnyScssInterpolatedUrlValuePart> for FormatAnyScssInterpolatedUrlValuePart {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyScssInterpolatedUrlValuePart,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyScssInterpolatedUrlValuePart::ScssInterpolation(node) => node.format().fmt(f),
            AnyScssInterpolatedUrlValuePart::ScssUrlText(node) => node.format().fmt(f),
        }
    }
}
