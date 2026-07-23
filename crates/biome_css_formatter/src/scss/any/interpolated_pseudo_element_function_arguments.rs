//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssInterpolatedPseudoElementFunctionArguments;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssInterpolatedPseudoElementFunctionArguments;
impl FormatRule<AnyScssInterpolatedPseudoElementFunctionArguments>
    for FormatAnyScssInterpolatedPseudoElementFunctionArguments
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyScssInterpolatedPseudoElementFunctionArguments,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node { AnyScssInterpolatedPseudoElementFunctionArguments :: ScssInterpolatedPseudoElementSelectorArguments (node) => node . format () . fmt (f) , AnyScssInterpolatedPseudoElementFunctionArguments :: ScssInterpolatedPseudoElementValueArguments (node) => node . format () . fmt (f) , }
    }
}
