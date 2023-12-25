use crate::prelude::*;
use biome_css_syntax::CssPageAtRuleItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageAtRuleItemList;
impl FormatRule<CssPageAtRuleItemList> for FormatCssPageAtRuleItemList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssPageAtRuleItemList, f: &mut CssFormatter) -> FormatResult<()> {
        // This is one of the few cases where we _do_ want to respect empty
        // lines from the input, so we can use `join_nodes_with_hardline`.
        let mut joiner = f.join_nodes_with_hardline();

        for item in node.iter() {
            joiner.entry(item.syntax(), &item.format());
        }

        joiner.finish()
    }
}
