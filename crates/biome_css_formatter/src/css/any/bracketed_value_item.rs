//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssBracketedValueItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssBracketedValueItem;
impl FormatRule<AnyCssBracketedValueItem> for FormatAnyCssBracketedValueItem {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssBracketedValueItem, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssBracketedValueItem::AnyCssCustomIdentifier(node) => node.format().fmt(f),
            AnyCssBracketedValueItem::CssGenericDelimiter(node) => node.format().fmt(f),
        }
    }
}
