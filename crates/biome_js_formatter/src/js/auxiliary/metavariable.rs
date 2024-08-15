use crate::prelude::*;
use biome_js_syntax::JsMetavariable;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsMetavariable;
impl FormatNodeRule<JsMetavariable> for FormatJsMetavariable {
    fn fmt_fields(&self, node: &JsMetavariable, f: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
