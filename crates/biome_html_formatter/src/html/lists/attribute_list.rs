use crate::prelude::*;
use biome_html_syntax::HtmlAttributeList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeList;
impl FormatRule<HtmlAttributeList> for FormatHtmlAttributeList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &HtmlAttributeList, f: &mut HtmlFormatter) -> FormatResult<()> {
        // The formatting of this node is handled by `HtmlOpeningElement` and `HtmlSelfClosingElement` instead.
        //
        // This is because whether or not the element children breaks is partially dependent whether or not the
        // attributes break, and to be able to handle the `bracketSameLine` option correctly.
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
