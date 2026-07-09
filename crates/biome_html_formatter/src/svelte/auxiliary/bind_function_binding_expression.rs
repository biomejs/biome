use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_html_syntax::{
    SvelteBindFunctionBindingExpression, SvelteBindFunctionBindingExpressionFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteBindFunctionBindingExpression;

impl FormatNodeRule<SvelteBindFunctionBindingExpression>
    for FormatSvelteBindFunctionBindingExpression
{
    fn fmt_fields(
        &self,
        node: &SvelteBindFunctionBindingExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteBindFunctionBindingExpressionFields {
            l_curly_token,
            get,
            comma_token,
            set,
            r_curly_token,
        } = node.as_fields();

        // Match Prettier's `surroundWithSoftline` for bind SequenceExpressions:
        // fits on one line as `{get, set}`, breaks with indented get/set when needed.
        write!(
            f,
            [group(&format_args![
                l_curly_token.format(),
                soft_block_indent(&format_args![
                    get.format(),
                    comma_token.format(),
                    soft_line_break_or_space(),
                    set.format(),
                ]),
                r_curly_token.format(),
            ])]
        )
    }
}
