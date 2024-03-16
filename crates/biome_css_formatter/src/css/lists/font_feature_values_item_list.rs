use crate::prelude::*;
use biome_css_syntax::CssFontFeatureValuesItemList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFeatureValuesItemList;
impl FormatRule<CssFontFeatureValuesItemList> for FormatCssFontFeatureValuesItemList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssFontFeatureValuesItemList, f: &mut CssFormatter) -> FormatResult<()> {
        // This is one of the few cases where we _do_ want to respect empty
        // lines from the input, so we can use `join_nodes_with_hardline`.
        let mut join = f.join_nodes_with_hardline();

        for item in node {
            join.entry(item.syntax(), &format_or_verbatim(item.format()));
        }

        join.finish()
    }
}
