use crate::prelude::*;
use biome_formatter::write;
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

        write!(
            f,
            [group(&biome_formatter::format_args![
                l_curly_token.format(),
                get.format(),
                comma_token.format(),
                space(),
                set.format(),
                r_curly_token.format()
            ])]
        )
    }
}
