use crate::prelude::*;
use biome_css_syntax::CssRuleList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRuleList;
impl FormatRule<CssRuleList> for FormatCssRuleList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssRuleList, f: &mut CssFormatter) -> FormatResult<()> {
        // This is one of the few cases where we _do_ want to respect empty
        // lines from the input, so we can use `join_nodes_with_hardline`.
        let mut join = f.join_nodes_with_hardline();

        for rule in node {
            join.entry(rule.syntax(), &format_or_verbatim(rule.format()));
        }

        join.finish()
    }
}
