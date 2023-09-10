use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use crate::ts::assignments::as_assignment::TsAsOrSatisfiesAssignment;
use biome_js_syntax::JsSyntaxNode;
use biome_js_syntax::TsSatisfiesAssignment;

#[derive(Debug, Clone, Default)]
pub struct FormatTsSatisfiesAssignment;

impl FormatNodeRule<TsSatisfiesAssignment> for FormatTsSatisfiesAssignment {
    fn fmt_fields(&self, node: &TsSatisfiesAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        TsAsOrSatisfiesAssignment::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &TsSatisfiesAssignment) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsSatisfiesAssignment {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        TsAsOrSatisfiesAssignment::from(self.clone()).needs_parentheses_with_parent(parent)
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
