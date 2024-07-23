use crate::prelude::*;
use biome_js_syntax::JsGritMetavariable;
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsGritMetavariable;
impl FormatNodeRule<JsGritMetavariable> for FormatJsGritMetavariable {
    fn fmt_fields(&self, node: &JsGritMetavariable, f: &mut JsFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
