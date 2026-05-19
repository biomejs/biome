use crate::prelude::*;
use biome_css_syntax::{
    ScssInterpolatedPseudoClassRelativeSelectorArguments,
    ScssInterpolatedPseudoClassRelativeSelectorArgumentsFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedPseudoClassRelativeSelectorArguments;
impl FormatNodeRule<ScssInterpolatedPseudoClassRelativeSelectorArguments>
    for FormatScssInterpolatedPseudoClassRelativeSelectorArguments
{
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedPseudoClassRelativeSelectorArguments,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedPseudoClassRelativeSelectorArgumentsFields { selectors } =
            node.as_fields();

        write!(f, [selectors.format()])
    }
}
