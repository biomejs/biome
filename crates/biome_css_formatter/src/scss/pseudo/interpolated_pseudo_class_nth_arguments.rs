use crate::prelude::*;
use biome_css_syntax::{
    ScssInterpolatedPseudoClassNthArguments, ScssInterpolatedPseudoClassNthArgumentsFields,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedPseudoClassNthArguments;
impl FormatNodeRule<ScssInterpolatedPseudoClassNthArguments>
    for FormatScssInterpolatedPseudoClassNthArguments
{
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedPseudoClassNthArguments,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedPseudoClassNthArgumentsFields { selector } = node.as_fields();

        selector.format().fmt(f)
    }
}
