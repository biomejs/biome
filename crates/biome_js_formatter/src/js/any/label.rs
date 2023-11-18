//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::JsLabel;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsLabel;
impl FormatRule<AnyJsBinding> for FormatAnyJsLabel {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsLabel, f: &mut JsFormatter) -> FormatResult<()> {
        node.format().fmt(f)
    }
}
