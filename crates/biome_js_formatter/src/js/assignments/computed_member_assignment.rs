use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{AnyJsComputedMember, JsComputedMemberAssignment, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsComputedMemberAssignment;

impl FormatNodeRule<JsComputedMemberAssignment> for FormatJsComputedMemberAssignment {
    fn fmt_fields(
        &self,
        node: &JsComputedMemberAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        AnyJsComputedMember::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &JsComputedMemberAssignment) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsComputedMemberAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        false
    }
}
