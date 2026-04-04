//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssRootItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssRootItem;
impl FormatRule<AnyCssRootItem> for FormatAnyCssRootItem {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssRootItem, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssRootItem::AnyCssRule(node) => node.format().fmt(f),
            AnyCssRootItem::CssBogus(node) => node.format().fmt(f),
            AnyCssRootItem::ScssDeclaration(node) => node.format().fmt(f),
        }
    }
}
