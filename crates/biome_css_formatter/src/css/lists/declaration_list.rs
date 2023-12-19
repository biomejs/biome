use crate::prelude::*;
use biome_css_syntax::CssDeclarationList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationList;
impl FormatRule<CssDeclarationList> for FormatCssDeclarationList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssDeclarationList, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
