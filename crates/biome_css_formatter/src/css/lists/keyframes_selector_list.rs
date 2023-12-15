use crate::prelude::*;
use biome_css_syntax::CssKeyframesSelectorList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesSelectorList;
impl FormatRule<CssKeyframesSelectorList> for FormatCssKeyframesSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssKeyframesSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
