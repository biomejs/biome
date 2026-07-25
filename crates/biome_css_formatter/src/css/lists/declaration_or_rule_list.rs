use crate::prelude::*;
use biome_css_syntax::CssDeclarationOrRuleList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationOrRuleList;
impl FormatRule<CssDeclarationOrRuleList> for FormatCssDeclarationOrRuleList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssDeclarationOrRuleList, f: &mut CssFormatter) -> FormatResult<()> {
        // The value of a `style` attribute belongs on the attribute's own line
        // for as long as it fits there, so its declarations are separated by a
        // space that turns into a line break only once they no longer do.
        if f.options().is_html_style_attribute() {
            let mut join = f.join_nodes_with_soft_line();

            for declaration_or_rule in node {
                join.entry(
                    declaration_or_rule.syntax(),
                    &format_or_verbatim(declaration_or_rule.format()),
                );
            }

            return join.finish();
        }

        // This is one of the few cases where we _do_ want to respect empty
        // lines from the input, so we can use `join_nodes_with_hardline`.
        let mut join = f.join_nodes_with_hardline();

        for declaration_or_rule in node {
            join.entry(
                declaration_or_rule.syntax(),
                &format_or_verbatim(declaration_or_rule.format()),
            );
        }

        join.finish()
    }
}
