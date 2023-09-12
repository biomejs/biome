//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsModuleItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsModuleItem;
impl FormatRule<AnyJsModuleItem> for FormatAnyJsModuleItem {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsModuleItem, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsModuleItem::AnyJsStatement(node) => node.format().fmt(f),
            AnyJsModuleItem::JsExport(node) => node.format().fmt(f),
            AnyJsModuleItem::JsImport(node) => node.format().fmt(f),
        }
    }
}
