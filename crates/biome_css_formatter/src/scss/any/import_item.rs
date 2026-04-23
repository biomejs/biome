//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssImportItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssImportItem;
impl FormatRule<AnyScssImportItem> for FormatAnyScssImportItem {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssImportItem, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssImportItem::CssString(node) => node.format().fmt(f),
            AnyScssImportItem::ScssInterpolatedString(node) => node.format().fmt(f),
            AnyScssImportItem::ScssPlainImport(node) => node.format().fmt(f),
        }
    }
}
