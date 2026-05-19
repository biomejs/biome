use crate::prelude::*;
use biome_css_syntax::{
    ScssInterpolatedPseudoClassNthArguments, ScssInterpolatedPseudoClassNthArgumentsFields,
};
use biome_formatter::write;

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

        write!(f, [selector.format()])
    }
}
