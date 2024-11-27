use crate::prelude::*;
use biome_js_syntax::JsImportAssertionEntryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportAssertionEntryList;
impl FormatRule<JsImportAssertionEntryList> for FormatJsImportAssertionEntryList {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsImportAssertionEntryList, f: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
