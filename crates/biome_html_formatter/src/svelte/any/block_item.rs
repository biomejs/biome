//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_html_syntax::AnySvelteBlockItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteBlockItem;
impl FormatRule<AnySvelteBlockItem> for FormatAnySvelteBlockItem {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnySvelteBlockItem, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnySvelteBlockItem::SvelteEachAsKeyedItem(node) => node.format().fmt(f),
            AnySvelteBlockItem::SvelteEachKeyedItem(node) => node.format().fmt(f),
        }
    }
}
