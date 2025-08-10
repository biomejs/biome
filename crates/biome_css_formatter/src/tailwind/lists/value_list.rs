use crate::prelude::*;
use biome_css_syntax::TwValueList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwValueList;
impl FormatRule<TwValueList> for FormatTwValueList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &TwValueList, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
