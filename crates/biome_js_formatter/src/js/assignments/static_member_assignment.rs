use crate::js::expressions::static_member_expression::AnyJsStaticMemberLike;
use crate::prelude::*;

use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsStaticMemberAssignment;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsStaticMemberAssignment;

impl FormatNodeRule<JsStaticMemberAssignment> for FormatJsStaticMemberAssignment {
    fn fmt_fields(&self, node: &JsStaticMemberAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        AnyJsStaticMemberLike::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &JsStaticMemberAssignment) -> bool {
        item.needs_parentheses()
    }
}
