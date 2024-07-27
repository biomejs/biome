use crate::prelude::*;

use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::TsSatisfiesAssignment;

use super::as_assignment::format_as_or_satisfies_assignment;

#[derive(Debug, Clone, Default)]
pub struct FormatTsSatisfiesAssignment;

impl FormatNodeRule<TsSatisfiesAssignment> for FormatTsSatisfiesAssignment {
    fn fmt_fields(&self, node: &TsSatisfiesAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        format_as_or_satisfies_assignment(
            f,
            node.assignment()?,
            node.satisfies_token()?,
            node.ty()?,
        )
    }

    fn needs_parentheses(&self, item: &TsSatisfiesAssignment) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_needs_parentheses;
    use biome_js_syntax::TsSatisfiesAssignment;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("a satisfies number = 'test'", TsSatisfiesAssignment);
        assert_needs_parentheses!("(a satisfies number)! = 'test'", TsSatisfiesAssignment);
        assert_needs_parentheses!(
            "(<number>(a satisfies number)) = 'test'",
            TsSatisfiesAssignment
        );
        assert_needs_parentheses!("++(a satisfies number)", TsSatisfiesAssignment);
        assert_needs_parentheses!("(a satisfies number)--", TsSatisfiesAssignment);
        assert_needs_parentheses!(
            "({ a: a satisfies number } = { a: 5 })",
            TsSatisfiesAssignment
        );
    }
}
