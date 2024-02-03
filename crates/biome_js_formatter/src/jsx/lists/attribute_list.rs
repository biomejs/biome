use crate::prelude::*;
use biome_formatter::AttributePosition;

use biome_js_syntax::JsxAttributeList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxAttributeList;

impl FormatRule<JsxAttributeList> for FormatJsxAttributeList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsxAttributeList, f: &mut JsFormatter) -> FormatResult<()> {
        let line_break = if f.options().attribute_position() == AttributePosition::Multiline {
            hard_line_break()
        } else {
            soft_line_break_or_space()
        };

        f.join_with(&line_break)
            .entries(node.iter().formatted())
            .finish()
    }
}
