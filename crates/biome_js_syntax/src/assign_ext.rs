use biome_rowan::{SyntaxResult, declare_node_union};
use std::iter;

use crate::static_value::StaticValue;
use crate::{
    AnyJsAssignment, AnyJsExpression, JsComputedMemberAssignment, JsStaticMemberAssignment,
};

impl AnyJsAssignment {
    /// Returns the assignment target inside any parentheses that wrap the target itself.
    ///
    /// An assignment target is the part of an assignment that receives the value, such as `a`
    /// in `a = b`, or `a.b` in the destructuring assignment `[a.b] = c`. This method removes
    /// parentheses written around the target alone: `(a)` in `(a) = b` returns `a`, and
    /// `((a.b))` in `((a.b)) = c` returns `a.b`.
    ///
    /// Parentheses around a whole assignment, as in `(a = b)`, aren't part of the target. They
    /// wrap an expression, and [AnyJsExpression::omit_parentheses] removes them.
    ///
    /// Returns `self` when the target isn't parenthesized. When the code is incomplete and a
    /// pair of parentheses has no target inside, returns that pair of parentheses.
    pub fn omit_target_parentheses(self) -> Self {
        let first = self
            .as_js_parenthesized_assignment()
            .and_then(|parenthesized| parenthesized.assignment().ok());

        iter::successors(first, |assignment| {
            assignment
                .as_js_parenthesized_assignment()?
                .assignment()
                .ok()
        })
        .last()
        .unwrap_or(self)
    }
}

declare_node_union! {
    pub AnyJsMemberAssignment = JsComputedMemberAssignment | JsStaticMemberAssignment
}

impl AnyJsMemberAssignment {
    pub fn object(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            Self::JsComputedMemberAssignment(assignment) => assignment.object(),
            Self::JsStaticMemberAssignment(assignment) => assignment.object(),
        }
    }

    /// Returns the name of the assigned member when it's known without running the code:
    /// `a.b`, `a["b"]`, and ``a[`b`]`` all return `b`.
    ///
    /// Returns `None` for private names such as `a.#b`, and for computed members
    /// that aren't a string, such as `a[b]` or `a[0]`.
    pub fn member_name(&self) -> Option<StaticValue> {
        let value = match self {
            Self::JsStaticMemberAssignment(assignment) => {
                StaticValue::String(assignment.member().ok()?.as_js_name()?.value_token().ok()?)
            }
            Self::JsComputedMemberAssignment(assignment) => {
                let member = assignment.member().ok()?.omit_parentheses();
                let result = member.as_static_value()?;
                if !matches!(result, StaticValue::String(_) | StaticValue::EmptyString(_)) {
                    return None;
                }
                result
            }
        };
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use biome_js_parser::{JsParserOptions, parse_module};
    use biome_rowan::AstNode;

    use biome_js_factory::syntax::{AnyJsAssignment, JsAssignmentExpression, JsSyntaxKind};

    /// Returns the kind and source text of the first assignment target in `source` after
    /// removing the parentheses around it.
    fn omit_first_target_parentheses(source: &str) -> (JsSyntaxKind, String) {
        let parsed = parse_module(source, JsParserOptions::default());
        let target = parsed
            .tree()
            .syntax()
            .descendants()
            .find_map(AnyJsAssignment::cast)
            .expect("source must contain an assignment target")
            .omit_target_parentheses();
        (
            target.syntax().kind(),
            target.syntax().text_trimmed().to_string(),
        )
    }

    #[test]
    fn omits_parentheses_around_target() {
        assert_eq!(
            omit_first_target_parentheses("(a) = b;"),
            (JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT, "a".to_string())
        );
        assert_eq!(
            omit_first_target_parentheses("((a.b)) = c;"),
            (JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT, "a.b".to_string())
        );
        assert_eq!(
            omit_first_target_parentheses("[(a[b])] = c;"),
            (
                JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT,
                "a[b]".to_string()
            )
        );
    }

    #[test]
    fn keeps_unparenthesized_target() {
        assert_eq!(
            omit_first_target_parentheses("a.b = c;"),
            (JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT, "a.b".to_string())
        );
    }

    #[test]
    fn keeps_parentheses_inside_target() {
        assert_eq!(
            omit_first_target_parentheses("((a).b) = c;"),
            (
                JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT,
                "(a).b".to_string()
            )
        );
    }

    #[test]
    fn ignores_parentheses_around_whole_assignment() {
        let parsed = parse_module("(a = b);", JsParserOptions::default());
        let assignment = parsed
            .tree()
            .syntax()
            .descendants()
            .find_map(JsAssignmentExpression::cast)
            .expect("source must contain an assignment");
        let target = assignment
            .left()
            .expect("assignment must have a target")
            .into_syntax();
        let target = AnyJsAssignment::cast(target)
            .expect("target must be a simple assignment target")
            .omit_target_parentheses();
        assert_eq!(
            target.syntax().kind(),
            JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT
        );
        assert_eq!(target.syntax().text_trimmed().to_string(), "a");
    }
}
