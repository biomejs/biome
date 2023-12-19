use crate::{prelude::*, separated::FormatAstSeparatedListExtension};
use biome_css_syntax::CssSelectorList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSelectorList;
impl FormatRule<CssSelectorList> for FormatCssSelectorList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssSelectorList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join_with(&hard_line_break())
            .entries(node.format_separated(","))
            .finish()
    }
}
