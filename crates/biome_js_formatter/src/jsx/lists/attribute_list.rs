use crate::prelude::*;
use biome_formatter::FormatContext;

use biome_js_syntax::JsxAttributeList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxAttributeList;

impl FormatRule<JsxAttributeList> for FormatJsxAttributeList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsxAttributeList, f: &mut JsFormatter) -> FormatResult<()> {
        let line_break = if f.context().options().single_attribute_per_line().value() {
            hard_line_break()
        } else {
            soft_line_break_or_space()
        };

        f.join_with(&line_break)
            .entries(node.iter().formatted())
            .finish()
    }
}
