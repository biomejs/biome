use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{JsIdentifierAssignment, JsIdentifierAssignmentFields};

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
