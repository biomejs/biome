//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnyHtmlRootItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlRootItem;
impl FormatRule<AnyHtmlRootItem> for FormatAnyHtmlRootItem {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyHtmlRootItem, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyHtmlRootItem::AnyHtmlElement(node) => node.format().fmt(f),
            AnyHtmlRootItem::HtmlDirective(node) => node.format().fmt(f),
        }
    }
}
