use crate::prelude::*;
use biome_css_syntax::CssKeyframesItemList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesItemList;
impl FormatRule<CssKeyframesItemList> for FormatCssKeyframesItemList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssKeyframesItemList, f: &mut CssFormatter) -> FormatResult<()> {
        let mut joiner = f.join_nodes_with_hardline();

        for item in node.iter() {
            joiner.entry(item.syntax(), &item.format());
        }

        joiner.finish()
    }
}
