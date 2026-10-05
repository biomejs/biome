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
        let content = format_with(|f| {
            if arguments.is_some() {
                arguments.format().fmt(f)
            } else {
                format_dangling_comments(node.syntax()).fmt(f)
            }
        });

        write!(
            f,
            [
                name.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&content),
                    r_paren_token.format()
                ])
            ]
        )
    }

    fn fmt_dangling_comments(
        &self,
        node: &ScssInterpolatedPseudoClassFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        if node.arguments().is_none() {
            Ok(())
        } else {
            format_dangling_comments(node.syntax())
                .with_soft_block_indent()
                .fmt(f)
        }
    }
}
