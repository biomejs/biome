use crate::prelude::*;
use biome_css_syntax::CssDocumentMatcherList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDocumentMatcherList;
impl FormatRule<CssDocumentMatcherList> for FormatCssDocumentMatcherList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssDocumentMatcherList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
