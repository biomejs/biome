use crate::prelude::*;
use biome_css_syntax::{
    ScssInterpolatedPseudoClassSelectorArguments,
    ScssInterpolatedPseudoClassSelectorArgumentsFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedPseudoClassSelectorArguments;
impl FormatNodeRule<ScssInterpolatedPseudoClassSelectorArguments>
    for FormatScssInterpolatedPseudoClassSelectorArguments
{
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedPseudoClassSelectorArguments,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedPseudoClassSelectorArgumentsFields { selectors } = node.as_fields();

        write!(f, [selectors.format()])
    }
}
