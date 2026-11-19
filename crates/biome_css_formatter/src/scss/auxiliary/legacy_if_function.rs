use crate::prelude::*;
use biome_css_syntax::{ScssLegacyIfFunction, ScssLegacyIfFunctionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssLegacyIfFunction;

impl FormatNodeRule<ScssLegacyIfFunction> for FormatScssLegacyIfFunction {
    fn fmt_fields(&self, node: &ScssLegacyIfFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssLegacyIfFunctionFields {
            if_token,
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [
                if_token.format()?.with_text_case(CssCase::Preserve),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(
                        &format_args![items.format(), format_dangling_comments(node.syntax())],
                        should_insert_space
                    ),
                    r_paren_token.format()
                ])
            ]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _: &ScssLegacyIfFunction,
        _: &mut CssFormatter,
    ) -> FormatResult<()> {
        Ok(())
    }
}
