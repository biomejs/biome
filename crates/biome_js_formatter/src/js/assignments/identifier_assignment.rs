use crate::prelude::*;
use biome_formatter::write;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{JsForOfStatement, JsIdentifierAssignmentFields, JsSyntaxKind};
use biome_js_syntax::{JsIdentifierAssignment, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsIdentifierAssignment;

impl FormatNodeRule<JsIdentifierAssignment> for FormatJsIdentifierAssignment {
    fn fmt_fields(&self, node: &JsIdentifierAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        let JsIdentifierAssignmentFields { name_token } = node.as_fields();

        write![f, [name_token.format()]]
    }

    fn needs_parentheses(&self, item: &JsIdentifierAssignment) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsIdentifierAssignment {
    #[inline]
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        let Ok(name) = self.name_token() else {
            return false;
        };
        match name.text_trimmed() {
            "async" => JsForOfStatement::cast_ref(parent)
                .is_some_and(|for_of| for_of.await_token().is_none()),
            "let" => parent.kind() == JsSyntaxKind::JS_FOR_OF_STATEMENT,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsIdentifierAssignment;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("for ((async) of []) {}", JsIdentifierAssignment);

        assert_not_needs_parentheses!("for await (async of []) {}", JsIdentifierAssignment);
        assert_not_needs_parentheses!("for (test of []) {}", JsIdentifierAssignment);
    }
}
