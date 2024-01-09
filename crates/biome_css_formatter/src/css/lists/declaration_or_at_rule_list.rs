use crate::prelude::*;
use biome_css_syntax::CssDeclarationOrAtRuleList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationOrAtRuleList;
impl FormatRule<CssDeclarationOrAtRuleList> for FormatCssDeclarationOrAtRuleList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssDeclarationOrAtRuleList, f: &mut CssFormatter) -> FormatResult<()> {
        // This is one of the few cases where we _do_ want to respect empty
        // lines from the input, so we can use `join_nodes_with_hardline`.
        let mut join = f.join_nodes_with_hardline();

        for declaration_or_at_rule in node {
            join.entry(
                declaration_or_at_rule.syntax(),
                &format_or_verbatim(declaration_or_at_rule.format()),
            );
        }

        join.finish()
    }
}
