use crate::prelude::*;
use biome_css_syntax::{
    ScssInterpolatedPseudoElementSelectorArguments,
    ScssInterpolatedPseudoElementSelectorArgumentsFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedPseudoElementSelectorArguments;
impl FormatNodeRule<ScssInterpolatedPseudoElementSelectorArguments>
    for FormatScssInterpolatedPseudoElementSelectorArguments
{
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedPseudoElementSelectorArguments,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedPseudoElementSelectorArgumentsFields { selectors } = node.as_fields();

        write!(f, [selectors.format()])
    }
}
