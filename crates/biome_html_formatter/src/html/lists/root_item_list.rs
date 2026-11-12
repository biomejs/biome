use crate::{html::lists::element_list::FormatHtmlElementList, prelude::*};
use biome_html_syntax::HtmlRootItemList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlRootItemList;

impl FormatRule<HtmlRootItemList> for FormatHtmlRootItemList {
    type Context = HtmlFormatContext;

    fn fmt(&self, node: &HtmlRootItemList, f: &mut HtmlFormatter) -> FormatResult<()> {
        FormatHtmlElementList::default().fmt_root_items(node, f)
    }
}
