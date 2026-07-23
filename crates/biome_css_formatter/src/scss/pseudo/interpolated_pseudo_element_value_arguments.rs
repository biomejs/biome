use crate::prelude::*;
use biome_css_syntax::{
    ScssInterpolatedPseudoElementValueArguments, ScssInterpolatedPseudoElementValueArgumentsFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedPseudoElementValueArguments;
impl FormatNodeRule<ScssInterpolatedPseudoElementValueArguments>
    for FormatScssInterpolatedPseudoElementValueArguments
{
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedPseudoElementValueArguments,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedPseudoElementValueArgumentsFields { values } = node.as_fields();

        write!(f, [values.format()])
    }
}
