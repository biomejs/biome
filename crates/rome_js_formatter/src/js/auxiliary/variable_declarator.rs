use crate::prelude::*;
use crate::utils::AnyJsAssignmentLike;
use biome_js_syntax::JsVariableDeclarator;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsVariableDeclarator;

impl FormatNodeRule<JsVariableDeclarator> for FormatJsVariableDeclarator {
    fn fmt_fields(&self, node: &JsVariableDeclarator, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [AnyJsAssignmentLike::from(node.clone())]]
    }
}
