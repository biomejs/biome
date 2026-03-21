//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssInterpolatedIdentifierPart;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssInterpolatedIdentifierPart;
impl FormatRule<AnyScssInterpolatedIdentifierPart> for FormatAnyScssInterpolatedIdentifierPart {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyScssInterpolatedIdentifierPart,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyScssInterpolatedIdentifierPart::CssCustomIdentifier(node) => node.format().fmt(f),
            AnyScssInterpolatedIdentifierPart::CssIdentifier(node) => node.format().fmt(f),
            AnyScssInterpolatedIdentifierPart::ScssInterpolation(node) => node.format().fmt(f),
        }
    }
}
