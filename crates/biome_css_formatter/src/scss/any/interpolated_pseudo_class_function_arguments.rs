//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssInterpolatedPseudoClassFunctionArguments;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssInterpolatedPseudoClassFunctionArguments;
impl FormatRule<AnyScssInterpolatedPseudoClassFunctionArguments>
    for FormatAnyScssInterpolatedPseudoClassFunctionArguments
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyScssInterpolatedPseudoClassFunctionArguments,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node { AnyScssInterpolatedPseudoClassFunctionArguments :: ScssInterpolatedPseudoClassNthArguments (node) => node . format () . fmt (f) , AnyScssInterpolatedPseudoClassFunctionArguments :: ScssInterpolatedPseudoClassRelativeSelectorArguments (node) => node . format () . fmt (f) , AnyScssInterpolatedPseudoClassFunctionArguments :: ScssInterpolatedPseudoClassSelectorArguments (node) => node . format () . fmt (f) , AnyScssInterpolatedPseudoClassFunctionArguments :: ScssInterpolatedPseudoClassValueArguments (node) => node . format () . fmt (f) , }
    }
}
