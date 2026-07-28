use crate::prelude::*;
use biome_css_syntax::CssDeclarationList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationList;
impl FormatRule<CssDeclarationList> for FormatCssDeclarationList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssDeclarationList, f: &mut CssFormatter) -> FormatResult<()> {
        // A `style` attribute stays on the tag's line while its declarations fit.
        if f.options().is_html_style_attribute() {
            let mut join = f.join_nodes_with_soft_line();

            for declaration in node {
                join.entry(
                    declaration.syntax(),
                    &format_or_verbatim(declaration.format()),
                );
            }

            return join.finish();
        }

        // This is one of the few cases where we _do_ want to respect empty
        // lines from the input, so we can use `join_nodes_with_hardline`.
        let mut join = f.join_nodes_with_hardline();

        for declaration in node {
            join.entry(
                declaration.syntax(),
                &format_or_verbatim(declaration.format()),
            );
        }

        join.finish()
    }
}
