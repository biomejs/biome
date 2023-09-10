use crate::prelude::*;

use biome_js_syntax::JsxAttributeList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxAttributeList;

impl FormatRule<JsxAttributeList> for FormatJsxAttributeList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsxAttributeList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()
    }
}
