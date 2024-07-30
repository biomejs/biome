use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::TsNonNullAssertionAssignment;
use biome_js_syntax::TsNonNullAssertionAssignmentFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsNonNullAssertionAssignment;

impl FormatNodeRule<TsNonNullAssertionAssignment> for FormatTsNonNullAssertionAssignment {
    fn fmt_fields(
        &self,
        node: &TsNonNullAssertionAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsNonNullAssertionAssignmentFields {
            assignment,
            excl_token,
        } = node.as_fields();
        write![f, [assignment.format(), excl_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsNonNullAssertionAssignment) -> bool {
        item.needs_parentheses()
    }
}
