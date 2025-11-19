use crate::prelude::*;
use biome_css_syntax::CssAttrName;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttrName;

impl FormatRule<CssAttrName> for FormatCssAttrName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssAttrName, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
