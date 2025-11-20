use crate::prelude::*;
use crate::verbatim::format_js_verbatim_node;
use biome_formatter::FormatResult;
use biome_js_syntax::JsxText;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxText;

impl FormatNodeRule<JsxText> for FormatJsxText {
    fn fmt_fields(&self, node: &JsxText, f: &mut JsFormatter) -> FormatResult<()> {
        // Formatting a [JsxText] on its own isn't supported. Format as verbatim. A text should always be formatted
        // through its [JsxChildList]
        format_js_verbatim_node(node.syntax()).fmt(f)
    }
}
