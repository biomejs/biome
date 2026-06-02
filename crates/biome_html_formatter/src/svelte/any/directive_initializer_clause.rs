use crate::prelude::*;
use biome_html_syntax::AnySvelteDirectiveInitializerClause;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteDirectiveInitializerClause;

impl FormatRule<AnySvelteDirectiveInitializerClause> for FormatAnySvelteDirectiveInitializerClause {
    type Context = HtmlFormatContext;

    fn fmt(
        &self,
        node: &AnySvelteDirectiveInitializerClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        match node {
            AnySvelteDirectiveInitializerClause::HtmlAttributeInitializerClause(node) => {
                node.format().fmt(f)
            }
            AnySvelteDirectiveInitializerClause::SvelteBindFunctionBindingInitializerClause(
                node,
            ) => node.format().fmt(f),
        }
    }
}
