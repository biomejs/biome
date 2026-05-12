use crate::prelude::*;
use biome_css_syntax::{
    ScssInterpolatedPseudoClassValueArguments, ScssInterpolatedPseudoClassValueArgumentsFields,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedPseudoClassValueArguments;
impl FormatNodeRule<ScssInterpolatedPseudoClassValueArguments>
    for FormatScssInterpolatedPseudoClassValueArguments
{
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedPseudoClassValueArguments,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedPseudoClassValueArgumentsFields { values } = node.as_fields();

        values.format().fmt(f)
    }
}
