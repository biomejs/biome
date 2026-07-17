use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{
    SvelteBindFunctionBindingInitializerClause, SvelteBindFunctionBindingInitializerClauseFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteBindFunctionBindingInitializerClause;

impl FormatNodeRule<SvelteBindFunctionBindingInitializerClause>
    for FormatSvelteBindFunctionBindingInitializerClause
{
    fn fmt_fields(
        &self,
        node: &SvelteBindFunctionBindingInitializerClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteBindFunctionBindingInitializerClauseFields { eq_token, value } = node.as_fields();

        write!(f, [eq_token.format(), value.format()])
    }
}
