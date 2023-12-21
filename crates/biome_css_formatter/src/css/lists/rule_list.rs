use crate::prelude::*;
use biome_css_syntax::CssRuleList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRuleList;
impl FormatRule<CssRuleList> for FormatCssRuleList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssRuleList, f: &mut CssFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for rule in node {
            join.entry(rule.syntax(), &format_or_verbatim(rule.format()));
        }

        join.finish()
    }
}
