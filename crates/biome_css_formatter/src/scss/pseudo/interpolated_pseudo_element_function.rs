use crate::prelude::*;
use biome_css_syntax::{
    ScssInterpolatedPseudoElementFunction, ScssInterpolatedPseudoElementFunctionFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedPseudoElementFunction;
impl FormatNodeRule<ScssInterpolatedPseudoElementFunction>
    for FormatScssInterpolatedPseudoElementFunction
{
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedPseudoElementFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedPseudoElementFunctionFields {
            name,
            l_paren_token,
            arguments,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&arguments.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
