use biome_rowan::AstNode;

use crate::{
    parentheses::NeedsParentheses, AnyJsAssignment, AnyJsAssignmentPattern,
    JsArrayAssignmentPattern, JsComputedMemberAssignment, JsForOfStatement, JsIdentifierAssignment,
    JsObjectAssignmentPattern, JsStaticMemberAssignment, JsSyntaxKind, JsSyntaxNode,
    TsAsAssignment, TsNonNullAssertionAssignment, TsSatisfiesAssignment, TsTypeAssertionAssignment,
};

impl NeedsParentheses for AnyJsAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::JsComputedMemberAssignment(assignment) => assignment.needs_parentheses(),
            Self::JsIdentifierAssignment(assignment) => assignment.needs_parentheses(),
            Self::JsStaticMemberAssignment(assignment) => assignment.needs_parentheses(),
            Self::TsAsAssignment(assignment) => assignment.needs_parentheses(),
            Self::TsSatisfiesAssignment(assignment) => assignment.needs_parentheses(),
            Self::TsNonNullAssertionAssignment(assignment) => assignment.needs_parentheses(),
            Self::TsTypeAssertionAssignment(assignment) => assignment.needs_parentheses(),
            Self::JsParenthesizedAssignment(_) | Self::JsBogusAssignment(_) => false,
        }
    }
}

impl NeedsParentheses for AnyJsAssignmentPattern {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::AnyJsAssignment(assignment) => assignment.needs_parentheses(),
            Self::JsArrayAssignmentPattern(assignment) => assignment.needs_parentheses(),
            Self::JsObjectAssignmentPattern(assignment) => assignment.needs_parentheses(),
        }
    }
}

impl NeedsParentheses for JsArrayAssignmentPattern {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for JsComputedMemberAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for JsIdentifierAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        let Ok(name) = self.name_token() else {
            return false;
        };
        match name.text_trimmed() {
            "async" => self
                .parent::<JsForOfStatement>()
                .is_some_and(|for_of| for_of.await_token().is_none()),
            "let" => self
                .syntax()
                .parent()
                .is_some_and(|parent| parent.kind() == JsSyntaxKind::JS_FOR_OF_STATEMENT),
            _ => false,
        }
    }
}

impl NeedsParentheses for JsObjectAssignmentPattern {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for JsStaticMemberAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for TsAsAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        ts_as_or_satisfies_needs_parens(self.syntax())
    }
}

impl NeedsParentheses for TsNonNullAssertionAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for TsSatisfiesAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        ts_as_or_satisfies_needs_parens(self.syntax())
    }
}

impl NeedsParentheses for TsTypeAssertionAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        self.syntax().parent().is_some_and(|parent| {
            matches!(
                parent.kind(),
                JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION
                    | JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT
                    | JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT
                    | JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
                    | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
            )
        })
    }
}

#[inline]
fn ts_as_or_satisfies_needs_parens(node: &JsSyntaxNode) -> bool {
    node.parent().is_some_and(|parent| {
        matches!(
            parent.kind(),
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION
                | JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT
                | JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT
                | JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
                | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
                | JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY
        )
    })
}
