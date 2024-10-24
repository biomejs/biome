//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsImportAttributeEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsImportAttributeEntry;
impl FormatRule<AnyJsImportAttributeEntry> for FormatAnyJsImportAttributeEntry {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsImportAttributeEntry, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsImportAttributeEntry::JsBogusImportAttributeEntry(node) => node.format().fmt(f),
            AnyJsImportAttributeEntry::JsImportAttributeEntry(node) => node.format().fmt(f),
        }
    }
}
