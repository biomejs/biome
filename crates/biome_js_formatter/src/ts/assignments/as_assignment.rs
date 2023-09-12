use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::{format_args, write};
use biome_js_syntax::{AnyJsAssignment, AnyTsType, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use biome_js_syntax::{TsAsAssignment, TsSatisfiesAssignment};
use biome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatTsAsAssignment;

impl FormatNodeRule<TsAsAssignment> for FormatTsAsAssignment {
    fn fmt_fields(&self, node: &TsAsAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        TsAsOrSatisfiesAssignment::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &TsAsAssignment) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsAsAssignment {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        TsAsOrSatisfiesAssignment::from(self.clone()).needs_parentheses_with_parent(parent)
    }
}

declare_node_union! {
    pub(crate) TsAsOrSatisfiesAssignment = TsAsAssignment | TsSatisfiesAssignment
}

impl Format<JsFormatContext> for TsAsOrSatisfiesAssignment {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let assignment = self.assignment()?;
        let operation_token = self.operation_token()?;
        let ty = self.ty()?;

        write![f, [assignment.format(), space(), operation_token.format()]]?;

        if f.comments().has_leading_own_line_comment(ty.syntax()) {
            write!(f, [indent(&format_args![hard_line_break(), ty.format()])])
        } else {
            write!(f, [space(), ty.format()])
        }
    }
}

impl NeedsParentheses for TsAsOrSatisfiesAssignment {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        matches!(
            parent.kind(),
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION
                | JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT
                | JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT
                | JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
                | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
                | JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY
        )
    }
}

impl TsAsOrSatisfiesAssignment {
    fn assignment(&self) -> SyntaxResult<AnyJsAssignment> {
        match self {
            TsAsOrSatisfiesAssignment::TsAsAssignment(assignment) => assignment.assignment(),
            TsAsOrSatisfiesAssignment::TsSatisfiesAssignment(assignment) => assignment.assignment(),
        }
    }

    fn operation_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            TsAsOrSatisfiesAssignment::TsAsAssignment(assignment) => assignment.as_token(),
            TsAsOrSatisfiesAssignment::TsSatisfiesAssignment(assignment) => {
                assignment.satisfies_token()
            }
        }
    }

    fn ty(&self) -> SyntaxResult<AnyTsType> {
        match self {
            TsAsOrSatisfiesAssignment::TsAsAssignment(assignment) => assignment.ty(),
            TsAsOrSatisfiesAssignment::TsSatisfiesAssignment(assignment) => assignment.ty(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::assert_needs_parentheses;
    use biome_js_syntax::TsAsAssignment;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("a as number = 'test'", TsAsAssignment);
        assert_needs_parentheses!("(a as number)! = 'test'", TsAsAssignment);
        assert_needs_parentheses!("(<number>(a as number)) = 'test'", TsAsAssignment);
        assert_needs_parentheses!("++(a as number)", TsAsAssignment);
        assert_needs_parentheses!("(a as number)--", TsAsAssignment);
        assert_needs_parentheses!("({ a: a as number } = { a: 5 })", TsAsAssignment);
    }
}
