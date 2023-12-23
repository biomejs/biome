use crate::prelude::*;
use biome_css_syntax::CssKeyframesItemList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesItemList;
impl FormatRule<CssKeyframesItemList> for FormatCssKeyframesItemList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssKeyframesItemList, f: &mut CssFormatter) -> FormatResult<()> {
        // This is one of the few cases where we _do_ want to respect empty
        // lines from the input, so we can use `join_nodes_with_hardline`.
        let mut joiner = f.join_nodes_with_hardline();

        for item in node.iter() {
            joiner.entry(item.syntax(), &item.format());
        }

        joiner.finish()
    }
}
