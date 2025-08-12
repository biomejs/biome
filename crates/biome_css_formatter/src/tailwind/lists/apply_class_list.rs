use crate::prelude::*;
use biome_css_syntax::TwApplyClassList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwApplyClassList;
impl FormatRule<TwApplyClassList> for FormatTwApplyClassList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &TwApplyClassList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join_with(space())
            .entries(node.iter().formatted())
            .finish()
    }
}
