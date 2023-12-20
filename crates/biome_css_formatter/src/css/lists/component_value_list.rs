use crate::prelude::*;
use biome_css_syntax::CssComponentValueList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComponentValueList;
impl FormatRule<CssComponentValueList> for FormatCssComponentValueList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssComponentValueList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
