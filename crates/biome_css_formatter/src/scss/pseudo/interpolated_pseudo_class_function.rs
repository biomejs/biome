use crate::prelude::*;
use biome_css_syntax::{
    ScssInterpolatedPseudoClassFunction, ScssInterpolatedPseudoClassFunctionFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedPseudoClassFunction;
impl FormatNodeRule<ScssInterpolatedPseudoClassFunction>
    for FormatScssInterpolatedPseudoClassFunction
{
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedPseudoClassFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedPseudoClassFunctionFields {
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
