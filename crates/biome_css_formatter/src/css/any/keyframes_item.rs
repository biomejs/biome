//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssKeyframesItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssKeyframesItem;
impl FormatRule<AnyCssKeyframesItem> for FormatAnyCssKeyframesItem {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssKeyframesItem, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssKeyframesItem::CssBogusKeyframesItem(node) => node.format().fmt(f),
            AnyCssKeyframesItem::CssKeyframesItem(node) => node.format().fmt(f),
        }
    }
}
