use std::borrow::Cow;

use biome_rowan::{match_ast, AstNode, AstNodeList, SyntaxResult};

use crate::{
    assign_ext::AnyJsMemberAssignment,
    binary_like_expression::{
        should_flatten, AnyJsBinaryLikeExpression, AnyJsBinaryLikeLeftExpression,
    },
    expression_left_side::AnyJsExpressionLeftSide,
    AnyJsAssignmentPattern, AnyJsComputedMember, AnyJsExpression, AnyJsForInitializer,
    AnyJsFunctionBody, AnyJsLiteralExpression, AnyJsMemberExpression, AnyJsStatement,
    JsArrayExpression, JsArrowFunctionExpression, JsAssignmentExpression, JsAwaitExpression,
    JsBigintLiteralExpression, JsBinaryExpression, JsBinaryOperator, JsBooleanLiteralExpression,
    JsCallExpression, JsClassExpression, JsComputedMemberAssignment, JsComputedMemberExpression,
    JsComputedMemberName, JsConditionalExpression, JsExpressionStatement, JsForStatement,
    JsFunctionExpression, JsIdentifierExpression, JsImportCallExpression, JsImportMetaExpression,
    JsInExpression, JsInstanceofExpression, JsLogicalExpression, JsNewExpression,
    JsNewTargetExpression, JsNullLiteralExpression, JsNumberLiteralExpression, JsObjectExpression,
    JsParenthesizedExpression, JsPostUpdateExpression, JsPreUpdateExpression, JsPreUpdateOperator,
    JsRegexLiteralExpression, JsSequenceExpression, JsStaticMemberExpression,
    JsStringLiteralExpression, JsSuperExpression, JsSyntaxKind, JsSyntaxNode, JsTemplateExpression,
    JsThisExpression, JsUnaryExpression, JsUnaryOperator, JsYieldExpression, JsxTagExpression,
    TsAsExpression, TsInstantiationExpression, TsNonNullAssertionExpression, TsSatisfiesExpression,
    TsTypeAssertionExpression,
};

use super::NeedsParentheses;

impl NeedsParentheses for AnyJsExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::JsImportMetaExpression(expr) => expr.needs_parentheses(),
            Self::AnyJsLiteralExpression(expr) => expr.needs_parentheses(),
            Self::JsArrayExpression(expr) => expr.needs_parentheses(),
            Self::JsArrowFunctionExpression(expr) => expr.needs_parentheses(),
            Self::JsAssignmentExpression(expr) => expr.needs_parentheses(),
            Self::JsAwaitExpression(expr) => expr.needs_parentheses(),
            Self::JsBinaryExpression(expr) => expr.needs_parentheses(),
            Self::JsCallExpression(expr) => expr.needs_parentheses(),
            Self::JsClassExpression(expr) => expr.needs_parentheses(),
            Self::JsComputedMemberExpression(expr) => expr.needs_parentheses(),
            Self::JsConditionalExpression(expr) => expr.needs_parentheses(),
            Self::JsFunctionExpression(expr) => expr.needs_parentheses(),
            Self::JsIdentifierExpression(expr) => expr.needs_parentheses(),
            Self::JsImportCallExpression(expr) => expr.needs_parentheses(),
            Self::JsInExpression(expr) => expr.needs_parentheses(),
            Self::JsInstanceofExpression(expr) => expr.needs_parentheses(),
            Self::JsLogicalExpression(expr) => expr.needs_parentheses(),
            Self::JsNewExpression(expr) => expr.needs_parentheses(),
            Self::JsObjectExpression(expr) => expr.needs_parentheses(),
            Self::JsParenthesizedExpression(expr) => expr.needs_parentheses(),
            Self::JsPostUpdateExpression(expr) => expr.needs_parentheses(),
            Self::JsPreUpdateExpression(expr) => expr.needs_parentheses(),
            Self::JsSequenceExpression(expr) => expr.needs_parentheses(),
            Self::JsStaticMemberExpression(expr) => expr.needs_parentheses(),
            Self::JsSuperExpression(expr) => expr.needs_parentheses(),
            Self::JsTemplateExpression(expr) => expr.needs_parentheses(),
            Self::JsThisExpression(expr) => expr.needs_parentheses(),
            Self::JsUnaryExpression(expr) => expr.needs_parentheses(),
            Self::JsBogusExpression(_) => false,
            Self::JsYieldExpression(expr) => expr.needs_parentheses(),
            Self::JsxTagExpression(expr) => expr.needs_parentheses(),
            Self::JsNewTargetExpression(expr) => expr.needs_parentheses(),
            Self::TsAsExpression(expr) => expr.needs_parentheses(),
            Self::TsSatisfiesExpression(expr) => expr.needs_parentheses(),
            Self::TsNonNullAssertionExpression(expr) => expr.needs_parentheses(),
            Self::TsTypeAssertionExpression(expr) => expr.needs_parentheses(),
            Self::TsInstantiationExpression(expr) => expr.needs_parentheses(),
        }
    }
}

impl NeedsParentheses for AnyJsExpressionLeftSide {
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::AnyJsExpression(expression) => expression.needs_parentheses(),
            Self::JsPrivateName(_) => false,
            Self::AnyJsAssignmentPattern(assignment) => assignment.needs_parentheses(),
        }
    }
}

impl NeedsParentheses for AnyJsLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::JsBigintLiteralExpression(expr) => expr.needs_parentheses(),
            Self::JsBooleanLiteralExpression(expr) => expr.needs_parentheses(),
            Self::JsNullLiteralExpression(expr) => expr.needs_parentheses(),
            Self::JsNumberLiteralExpression(expr) => expr.needs_parentheses(),
            Self::JsRegexLiteralExpression(expr) => expr.needs_parentheses(),
            Self::JsStringLiteralExpression(expr) => expr.needs_parentheses(),
        }
    }
}

impl NeedsParentheses for JsArrayExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for JsArrowFunctionExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        matches!(
            parent.kind(),
            // Cast like
            JsSyntaxKind::TS_AS_EXPRESSION
                | JsSyntaxKind::TS_SATISFIES_EXPRESSION
                | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
                // Unary like
                | JsSyntaxKind::JS_UNARY_EXPRESSION
                | JsSyntaxKind::JS_AWAIT_EXPRESSION
                // Binary-like
                | JsSyntaxKind::JS_LOGICAL_EXPRESSION
                | JsSyntaxKind::JS_BINARY_EXPRESSION
                | JsSyntaxKind::JS_INSTANCEOF_EXPRESSION
                | JsSyntaxKind::JS_IN_EXPRESSION
        ) || match JsConditionalExpression::try_cast(parent) {
            Ok(cond) => cond.test().is_ok_and(|test| test.syntax() == self.syntax()),
            Err(parent) => update_or_lower_expression_needs_parens(self.syntax(), parent),
        }
    }
}

impl NeedsParentheses for JsAssignmentExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        match_ast! {
            match &parent {
                JsAssignmentExpression(_) => false,
                // `[a = b]`
                JsComputedMemberName(_) => false,

                JsArrowFunctionExpression(arrow) => {
                    arrow.body().is_ok_and(|body| body.syntax() == self.syntax())
                },

                JsForStatement(for_statement) => {
                     let is_initializer = match for_statement.initializer() {
                        Some(AnyJsForInitializer::AnyJsExpression(expr)) => {
                            expr.syntax() == self.syntax()
                        }
                        None | Some(_) => false,
                    };
                    let is_update = for_statement
                        .update()
                        .is_some_and(|update| update.syntax() == self.syntax());
                    !(is_initializer || is_update)
                },
                JsExpressionStatement(_) => {
                    // Parenthesize `{ a } = { a: 5 }`
                    is_first_in_statement(
                        self.syntax(),
                        FirstInStatementMode::ExpressionStatementOrArrow,
                    ) && matches!(
                        self.left(),
                        Ok(AnyJsAssignmentPattern::JsObjectAssignmentPattern(_))
                    )
                },
                JsSequenceExpression(_) => {
                    let mut child = Cow::Borrowed(&parent);
                    for ancestor in parent.ancestors().skip(1) {
                        match ancestor.kind() {
                            JsSyntaxKind::JS_SEQUENCE_EXPRESSION
                            | JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => child = Cow::Owned(ancestor),
                            _ => {
                                let Some(for_statement) = JsForStatement::cast(ancestor) else {
                                    break;
                                };
                                let is_initializer = match for_statement.initializer() {
                                    Some(AnyJsForInitializer::AnyJsExpression(expression)) => {
                                        expression.syntax() == child.as_ref()
                                    }
                                    None | Some(_) => false,
                                };
                                let is_update = for_statement
                                    .update()
                                    .is_some_and(|update| update.syntax() == child.as_ref());
                                return !(is_initializer || is_update);
                            }
                        }
                    }
                    true
                },
                _ => true,
            }
        }
    }
}

impl NeedsParentheses for JsAwaitExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        await_or_yield_needs_parens(parent, self.syntax())
    }
}

impl NeedsParentheses for JsBigintLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for JsBinaryExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        binary_like_needs_parens(self.syntax())
    }
}

impl NeedsParentheses for JsBooleanLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for JsCallExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        parent.kind() == JsSyntaxKind::JS_NEW_EXPRESSION
            || (parent.kind() == JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
                && self.callee().map_or(true, |callee| {
                    let callee_range = callee.range();
                    let leftmost = AnyJsExpressionLeftSide::leftmost(callee);
                    // require parens for iife and
                    // when the leftmost expression is not a class expression or a function expression
                    callee_range != leftmost.range()
                        && matches!(
                            leftmost.syntax().kind(),
                            JsSyntaxKind::JS_CLASS_EXPRESSION
                                | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                        )
                }))
    }
}

impl NeedsParentheses for JsClassExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        (parent.kind() == JsSyntaxKind::JS_EXTENDS_CLAUSE && !self.decorators().is_empty())
            || matches!(
                parent.kind(),
                JsSyntaxKind::JS_CALL_EXPRESSION | JsSyntaxKind::JS_NEW_EXPRESSION
            )
            || is_first_in_statement(
                self.syntax(),
                FirstInStatementMode::ExpressionOrExportDefault,
            )
    }
}

impl NeedsParentheses for JsComputedMemberExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        parent.kind() == JsSyntaxKind::JS_NEW_EXPRESSION
            && (self.is_optional_chain() || member_chain_callee_needs_parens(self.clone().into()))
    }
}

impl NeedsParentheses for JsConditionalExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        match parent.kind() {
            JsSyntaxKind::JS_UNARY_EXPRESSION
            | JsSyntaxKind::JS_AWAIT_EXPRESSION
            | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
            | JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_SATISFIES_EXPRESSION
            // in spread
            | JsSyntaxKind::JSX_SPREAD_CHILD
            | JsSyntaxKind::JS_SPREAD
            | JsSyntaxKind::JSX_SPREAD_ATTRIBUTE
            // Binary-like
            | JsSyntaxKind::JS_LOGICAL_EXPRESSION
            | JsSyntaxKind::JS_BINARY_EXPRESSION
            | JsSyntaxKind::JS_INSTANCEOF_EXPRESSION
            | JsSyntaxKind::JS_IN_EXPRESSION => true,
            _ => {
                match JsConditionalExpression::try_cast(parent) {
                    Ok(cond) => cond.test().is_ok_and(|test| test.syntax() == self.syntax()),
                    Err(parent) => update_or_lower_expression_needs_parens(self.syntax(), parent),
                }
            }
        }
    }
}

impl NeedsParentheses for JsFunctionExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        matches!(
            parent.kind(),
            JsSyntaxKind::JS_CALL_EXPRESSION
                | JsSyntaxKind::JS_NEW_EXPRESSION
                | JsSyntaxKind::JS_TEMPLATE_EXPRESSION
        ) || is_first_in_statement(
            self.syntax(),
            FirstInStatementMode::ExpressionOrExportDefault,
        )
    }
}

impl NeedsParentheses for JsIdentifierExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        let Ok(name) = self.name().and_then(|x| x.value_token()) else {
            return false;
        };
        // In non-strict mode (sloppy mode), `let` may be a variable name.
        // To disambiguate `let` from a let-declaration,
        // we have to enclose `let` between parentheses in some dge cases.
        match parent.kind() {
            JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                // `(let)[0];`
                // `(let)[0] = 0;`
                // `for( (let)[0] = 0, b = 0;;;) {}`
                // `for( (let)[0] of []) {}`
                name.text_trimmed() == "let"
                    && parent
                        .ancestors()
                        .find(|x| {
                            !AnyJsExpression::can_cast(x.kind())
                                && !AnyJsMemberAssignment::can_cast(x.kind())
                        })
                        .filter(|x| {
                            matches!(
                                x.kind(),
                                JsSyntaxKind::JS_EXPRESSION_STATEMENT
                                    | JsSyntaxKind::JS_FOR_IN_STATEMENT
                                    | JsSyntaxKind::JS_FOR_OF_STATEMENT
                                    | JsSyntaxKind::JS_FOR_STATEMENT
                            )
                        })
                        .and_then(|x| x.first_child()?.first_token())
                        .is_some_and(|token| token == name)
            }
            JsSyntaxKind::JS_CALL_EXPRESSION
            | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
                // for ( (let).a of [] ) {}
                name.text_trimmed() == "let"
                    && parent
                        .ancestors()
                        .find(|x| {
                            !AnyJsExpression::can_cast(x.kind())
                                && !AnyJsMemberAssignment::can_cast(x.kind())
                        })
                        .filter(|x| x.kind() == JsSyntaxKind::JS_FOR_OF_STATEMENT)
                        .and_then(|x| x.first_child()?.first_token())
                        .is_some_and(|token| token == name)
            }
            JsSyntaxKind::TS_AS_EXPRESSION | JsSyntaxKind::TS_SATISFIES_EXPRESSION => {
                // `(let) as unknown satisfies unknown;`
                // `(type) as unknown satisfies unknown;`
                matches!(
                    name.text_trimmed(),
                    "await" | "interface" | "let" | "module" | "type" | "using" | "yield"
                ) && parent
                    .ancestors()
                    .skip(1)
                    .find(|x| {
                        !matches!(
                            x.kind(),
                            JsSyntaxKind::TS_AS_EXPRESSION | JsSyntaxKind::TS_SATISFIES_EXPRESSION
                        )
                    })
                    .is_some_and(|x| x.kind() == JsSyntaxKind::JS_EXPRESSION_STATEMENT)
            }
            _ => false,
        }
    }
}

impl NeedsParentheses for JsImportCallExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        self.syntax()
            .parent()
            .is_some_and(|parent| parent.kind() == JsSyntaxKind::JS_NEW_EXPRESSION)
    }
}

impl NeedsParentheses for JsImportMetaExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for JsInExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        is_in_for_initializer(self) || binary_like_needs_parens(self.syntax())
    }
}
/// Add parentheses if the `in` is inside of a `for` initializer (see tests).
fn is_in_for_initializer(expression: &JsInExpression) -> bool {
    let statement = expression
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyJsStatement::cast);
    match statement {
        Some(AnyJsStatement::JsForInStatement(for_in_statement)) => for_in_statement
            .initializer()
            .is_ok_and(|initializer| initializer.range().contains(expression.range().start())),
        Some(AnyJsStatement::JsForStatement(for_statement)) => for_statement
            .initializer()
            .is_some_and(|initializer| initializer.range().contains(expression.range().start())),
        _ => false,
    }
}

impl NeedsParentheses for JsInstanceofExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        binary_like_needs_parens(self.syntax())
    }
}

impl NeedsParentheses for JsLogicalExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        if let Some(parent) = self.parent::<JsLogicalExpression>() {
            parent.operator() != self.operator()
        } else {
            binary_like_needs_parens(self.syntax())
        }
    }
}

impl NeedsParentheses for JsNewExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        self.syntax()
            .parent()
            .is_some_and(|node| node.kind() == JsSyntaxKind::JS_EXTENDS_CLAUSE)
    }
}

impl NeedsParentheses for JsNewTargetExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for JsNullLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for JsNumberLiteralExpression {
    fn needs_parentheses(&self) -> bool {
        self.syntax().parent().is_some_and(|parent| {
            matches!(
                parent.kind(),
                JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                    | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT,
            ) || AnyJsComputedMember::cast(parent)
                .and_then(|member| member.object().ok())
                .is_some_and(|object| object.syntax() == self.syntax())
        })
    }
}

impl NeedsParentheses for JsObjectExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        matches!(parent.kind(), JsSyntaxKind::JS_EXTENDS_CLAUSE)
            || is_first_in_statement(
                self.syntax(),
                FirstInStatementMode::ExpressionStatementOrArrow,
            )
    }
}

impl NeedsParentheses for JsParenthesizedExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for JsPostUpdateExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        unary_like_expression_needs_parens(self.syntax())
    }
}

impl NeedsParentheses for JsPreUpdateExpression {
    fn needs_parentheses(&self) -> bool {
        if let Some(unary) = self.parent::<JsUnaryExpression>() {
            let parent_operator = unary.operator();
            let operator = self.operator();
            (parent_operator == Ok(JsUnaryOperator::Plus)
                && operator == Ok(JsPreUpdateOperator::Increment))
                || (parent_operator == Ok(JsUnaryOperator::Minus)
                    && operator == Ok(JsPreUpdateOperator::Decrement))
        } else {
            unary_like_expression_needs_parens(self.syntax())
        }
    }
}

impl NeedsParentheses for JsRegexLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for JsSequenceExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        !matches!(
            parent.kind(),
            JsSyntaxKind::JS_RETURN_STATEMENT |
            // There's a precedence for writing `x++, y++`
            JsSyntaxKind::JS_FOR_STATEMENT |
            JsSyntaxKind::JS_EXPRESSION_STATEMENT |
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION  |
            // Handled as part of the arrow function formatting
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
        )
    }
}

impl NeedsParentheses for JsStaticMemberExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        matches!(parent.kind(), JsSyntaxKind::JS_NEW_EXPRESSION)
            && (self.is_optional_chain() || member_chain_callee_needs_parens(self.clone().into()))
    }
}

impl NeedsParentheses for JsStringLiteralExpression {
    fn needs_parentheses(&self) -> bool {
        if let Some(expression_statement) = self.parent::<JsExpressionStatement>() {
            expression_statement
                .syntax()
                .parent()
                .is_some_and(|grand_parent| {
                    matches!(
                        grand_parent.kind(),
                        JsSyntaxKind::JS_STATEMENT_LIST | JsSyntaxKind::JS_MODULE_ITEM_LIST
                    )
                })
        } else {
            false
        }
    }
}

impl NeedsParentheses for JsSuperExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

/// `TemplateLiteral`'s are `PrimaryExpression's that never need parentheses.
impl NeedsParentheses for JsTemplateExpression {
    fn needs_parentheses(&self) -> bool {
        if self.tag().is_some() {
            let Some(parent) = self.syntax().parent() else {
                return false;
            };
            parent.kind() == JsSyntaxKind::JS_NEW_EXPRESSION
                && member_chain_callee_needs_parens(self.clone().into())
        } else {
            false
        }
    }
}

impl NeedsParentheses for JsThisExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for JsUnaryExpression {
    fn needs_parentheses(&self) -> bool {
        match self.parent::<AnyJsExpression>() {
            Some(AnyJsExpression::JsUnaryExpression(parent_unary)) => {
                let parent_operator = parent_unary.operator();
                let operator = self.operator();

                matches!(operator, Ok(JsUnaryOperator::Plus | JsUnaryOperator::Minus))
                    && parent_operator == operator
            }
            // A user typing `!foo instanceof Bar` probably intended `!(foo instanceof Bar)`,
            // so format to `(!foo) instance Bar` to what is really happening
            Some(AnyJsExpression::JsInstanceofExpression(_)) => true,
            // A user typing `!foo in bar` probably intended `!(foo instanceof Bar)`,
            // so format to `(!foo) in bar` to what is really happening
            Some(AnyJsExpression::JsInExpression(_)) => true,
            _ => unary_like_expression_needs_parens(self.syntax()),
        }
    }
}

impl NeedsParentheses for JsYieldExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        matches!(
            parent.kind(),
            JsSyntaxKind::JS_AWAIT_EXPRESSION | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
        ) || await_or_yield_needs_parens(parent, self.syntax())
    }
}

impl NeedsParentheses for JsxTagExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        match parent.kind() {
            JsSyntaxKind::JS_BINARY_EXPRESSION => {
                let binary = JsBinaryExpression::unwrap_cast(parent);
                let is_left = binary.left().map(AstNode::into_syntax).as_ref() == Ok(self.syntax());
                matches!(binary.operator(), Ok(JsBinaryOperator::LessThan)) && is_left
            }
            JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_SATISFIES_EXPRESSION
            | JsSyntaxKind::JS_AWAIT_EXPRESSION
            | JsSyntaxKind::JS_EXTENDS_CLAUSE
            | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_SEQUENCE_EXPRESSION
            | JsSyntaxKind::JS_UNARY_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
            | JsSyntaxKind::JS_SPREAD
            | JsSyntaxKind::JSX_SPREAD_ATTRIBUTE
            | JsSyntaxKind::JSX_SPREAD_CHILD => true,
            _ => matches!(
                parent.kind(),
                JsSyntaxKind::JS_CALL_EXPRESSION
                    | JsSyntaxKind::JS_NEW_EXPRESSION
                    | JsSyntaxKind::JS_TEMPLATE_EXPRESSION
            ),
        }
    }
}

impl NeedsParentheses for TsAsExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        as_or_satisfies_expression_needs_parens(self.syntax(), self.expression())
    }
}

impl NeedsParentheses for TsInstantiationExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        self.parent::<AnyJsMemberExpression>()
            .is_some_and(|member_expr| {
                member_expr
                    .object()
                    .map(|object| object.syntax() == self.syntax())
                    .unwrap_or(false)
            })
    }
}

impl NeedsParentheses for TsNonNullAssertionExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        parent.kind() == JsSyntaxKind::JS_EXTENDS_CLAUSE
            || (parent.kind() == JsSyntaxKind::JS_NEW_EXPRESSION
                && member_chain_callee_needs_parens(self.clone().into()))
    }
}

impl NeedsParentheses for TsSatisfiesExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        as_or_satisfies_expression_needs_parens(self.syntax(), self.expression())
    }
}

impl NeedsParentheses for TsTypeAssertionExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        match parent.kind() {
            JsSyntaxKind::TS_AS_EXPRESSION => true,
            JsSyntaxKind::TS_SATISFIES_EXPRESSION => true,
            _ => type_cast_like_needs_parens(self.syntax(), parent),
        }
    }
}

fn as_or_satisfies_expression_needs_parens(
    node: &JsSyntaxNode,
    expression: SyntaxResult<AnyJsExpression>,
) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    match parent.kind() {
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION
        // Binary-like
        | JsSyntaxKind::JS_LOGICAL_EXPRESSION
        | JsSyntaxKind::JS_BINARY_EXPRESSION
        | JsSyntaxKind::JS_INSTANCEOF_EXPRESSION
        | JsSyntaxKind::JS_IN_EXPRESSION => true,
        // `export default (function foo() {} as bar)` needs to be special
        // cased. All other default-exported as expressions can be written
        // without parentheses, but function expressions _without_ the
        // parentheses because `JsExportDefaultFunctionDeclaration`s and
        // the cast becomes invalid.
        JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE => {
            expression.is_ok_and(|expression| {
                matches!(
                    expression.syntax().kind(),
                    JsSyntaxKind::JS_FUNCTION_EXPRESSION
                )
            })
        }
        _ => {
            type_cast_like_needs_parens(node, parent)
        }
    }
}

fn await_or_yield_needs_parens(parent: JsSyntaxNode, node: &JsSyntaxNode) -> bool {
    debug_assert!(matches!(
        node.kind(),
        JsSyntaxKind::JS_AWAIT_EXPRESSION | JsSyntaxKind::JS_YIELD_EXPRESSION
    ));
    match parent.kind() {
        JsSyntaxKind::JS_UNARY_EXPRESSION
        | JsSyntaxKind::TS_AS_EXPRESSION
        | JsSyntaxKind::TS_SATISFIES_EXPRESSION
        // in spread
        | JsSyntaxKind::JSX_SPREAD_CHILD
        | JsSyntaxKind::JS_SPREAD
        | JsSyntaxKind::JSX_SPREAD_ATTRIBUTE
        // Binary-like
        | JsSyntaxKind::JS_LOGICAL_EXPRESSION
        | JsSyntaxKind::JS_BINARY_EXPRESSION
        | JsSyntaxKind::JS_INSTANCEOF_EXPRESSION
        | JsSyntaxKind::JS_IN_EXPRESSION => true,
        _ => {
            match JsConditionalExpression::try_cast(parent) {
                Ok(cond) => cond.test().is_ok_and(|test| test.syntax() == node),
                Err(parent) => update_or_lower_expression_needs_parens(node, parent),
            }
        }
    }
}

/// Implements the rules when a node needs parentheses that are common across all [AnyJsBinaryLikeExpression] nodes.
fn binary_like_needs_parens(node: &JsSyntaxNode) -> bool {
    debug_assert!(AnyJsBinaryLikeExpression::can_cast(node.kind()));
    let Some(parent) = node.parent() else {
        return false;
    };
    match parent.kind() {
        JsSyntaxKind::JS_EXTENDS_CLAUSE
        | JsSyntaxKind::TS_AS_EXPRESSION
        | JsSyntaxKind::TS_SATISFIES_EXPRESSION
        | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
        | JsSyntaxKind::JS_UNARY_EXPRESSION
        | JsSyntaxKind::JS_AWAIT_EXPRESSION
        | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
        // in spread
        | JsSyntaxKind::JSX_SPREAD_CHILD
        | JsSyntaxKind::JS_SPREAD
        | JsSyntaxKind::JSX_SPREAD_ATTRIBUTE
        // Callee
        | JsSyntaxKind::JS_CALL_EXPRESSION
        | JsSyntaxKind::JS_NEW_EXPRESSION
        // Template tag
        | JsSyntaxKind::JS_TEMPLATE_EXPRESSION
        // Static member
        | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
        | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => true,
        _ => {
            let Some(node) = AnyJsBinaryLikeExpression::cast_ref(node) else {
                return false;
            };
            match AnyJsBinaryLikeExpression::try_cast(parent) {
                Ok(parent) => {
                    let (Ok(operator), Ok(parent_operator)) = (node.operator(), parent.operator()) else {
                        // Just to be sure
                        return true;
                    };
                    let precedence = operator.precedence();
                    let parent_precedence = parent_operator.precedence();

                    // If the parent has a higher precedence than parentheses are necessary to not change the semantic meaning
                    // when re-parsing.
                    if parent_precedence > precedence {
                        return true;
                    }

                    let is_right =
                        parent.right().map(AstNode::into_syntax).as_ref() == Ok(node.syntax());
                    // `a ** b ** c`
                    if is_right && parent_precedence == precedence {
                        return true;
                    }

                    // Add parentheses around bitwise and bit shift operators
                    // `a * 3 >> 5` -> `(a * 3) >> 5`
                    if parent_precedence.is_bitwise() || parent_precedence.is_shift() {
                        return true;
                    }

                    // `a % 4 + 4` -> `(a % 4) + 4)`
                    if parent_precedence < precedence && operator.is_remainder() {
                        return parent_precedence.is_additive();
                    }

                    parent_precedence == precedence && !should_flatten(parent_operator, operator)
                }
                Err(parent) => {
                    AnyJsComputedMember::cast(parent)
                        .and_then(|member| member.object().ok())
                        .is_some_and(|object| object.syntax() == node.syntax())
                }
            }
        }
    }
}

fn member_chain_callee_needs_parens(node: AnyJsExpression) -> bool {
    let mut object_chain = std::iter::successors(Some(node), |expr| match expr {
        AnyJsExpression::JsStaticMemberExpression(expr) => expr.object().ok(),
        AnyJsExpression::JsComputedMemberExpression(expr) => expr.object().ok(),
        AnyJsExpression::JsTemplateExpression(expr) => expr.tag(),
        AnyJsExpression::TsNonNullAssertionExpression(expr) => expr.expression().ok(),
        _ => None,
    });
    object_chain.any(|object| matches!(object, AnyJsExpression::JsCallExpression(_)))
}

fn type_cast_like_needs_parens(node: &JsSyntaxNode, parent: JsSyntaxNode) -> bool {
    match parent.kind() {
        JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
        | JsSyntaxKind::JS_EXTENDS_CLAUSE
        | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
        | JsSyntaxKind::JS_UNARY_EXPRESSION
        | JsSyntaxKind::JS_AWAIT_EXPRESSION
        | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
        // Callee
        | JsSyntaxKind::JS_CALL_EXPRESSION
        | JsSyntaxKind::JS_NEW_EXPRESSION
        // template tag
        | JsSyntaxKind::JS_TEMPLATE_EXPRESSION
        // in spread
        | JsSyntaxKind::JSX_SPREAD_CHILD
        | JsSyntaxKind::JS_SPREAD
        | JsSyntaxKind::JSX_SPREAD_ATTRIBUTE
        // static member
        | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
        | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => true,
        _ => AnyJsComputedMember::cast(parent)
            .and_then(|member| member.object().ok())
            .is_some_and(|object| object.syntax() == node)
    }
}

/// Implements the shared logic for when parentheses are necessary for [JsPreUpdateExpression], [JsPostUpdateExpression], or [JsUnaryExpression] expressions.
/// Each expression may implement node specific rules, which is why calling `needs_parens` on the node is preferred.
fn unary_like_expression_needs_parens(expression: &JsSyntaxNode) -> bool {
    let Some(parent) = expression.parent() else {
        return false;
    };
    match JsBinaryExpression::try_cast(parent) {
        Ok(binary) => {
            matches!(binary.operator(), Ok(JsBinaryOperator::Exponent))
                && binary.left().map(AstNode::into_syntax).as_ref() == Ok(expression)
        }
        Err(parent) => update_or_lower_expression_needs_parens(expression, parent),
    }
}

/// Returns `true` if an expression with lower precedence than an update expression needs parentheses.
///
/// This is generally the case if the expression is used in a left hand side, or primary expression context.
fn update_or_lower_expression_needs_parens(
    expression: &JsSyntaxNode,
    parent: JsSyntaxNode,
) -> bool {
    matches!(
        parent.kind(),
        JsSyntaxKind::JS_EXTENDS_CLAUSE
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
            // Callee
            | JsSyntaxKind::JS_CALL_EXPRESSION
            | JsSyntaxKind::JS_NEW_EXPRESSION
            // Template tag
            | JsSyntaxKind::JS_TEMPLATE_EXPRESSION
            // Static member
            | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
    ) || AnyJsComputedMember::cast(parent)
        .and_then(|member| member.object().ok())
        .is_some_and(|object| object.syntax() == expression)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum FirstInStatementMode {
    /// Considers [JsExpressionStatement] and the body of [JsArrowFunctionExpression] as the first statement.
    ExpressionStatementOrArrow,
    /// Considers [JsExpressionStatement] and [JsExportDefaultExpressionClause] as the first statement.
    ExpressionOrExportDefault,
}

/// Returns `true` if this node is at the start of an expression (depends on the passed `mode`).
///
/// Traverses upwards the tree for as long as the `node` is the left most expression until the node isn't
/// the left most node or reached a statement.
fn is_first_in_statement(node: &JsSyntaxNode, mode: FirstInStatementMode) -> bool {
    let mut current = Cow::Borrowed(node);
    let mut is_not_first_iteration = false;
    while let Some(parent) = current.parent() {
        let parent = match parent.kind() {
            JsSyntaxKind::JS_EXPRESSION_STATEMENT => {
                return true;
            }
            JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
            | JsSyntaxKind::JS_TEMPLATE_EXPRESSION
            | JsSyntaxKind::JS_CALL_EXPRESSION
            | JsSyntaxKind::JS_NEW_EXPRESSION
            | JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_SATISFIES_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => parent,
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                let expr = JsSequenceExpression::unwrap_cast(parent);
                let is_left = expr
                    .left()
                    .is_ok_and(|left| left.syntax() == current.as_ref());
                if is_left {
                    expr.into_syntax()
                } else {
                    break;
                }
            }
            JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                let expr = JsComputedMemberExpression::unwrap_cast(parent);
                let is_object = expr
                    .object()
                    .is_ok_and(|object| object.syntax() == current.as_ref());
                if is_object {
                    expr.into_syntax()
                } else {
                    break;
                }
            }
            JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                let assignment = JsComputedMemberAssignment::unwrap_cast(parent);
                let is_object = assignment
                    .object()
                    .is_ok_and(|object| object.syntax() == current.as_ref());
                if is_object {
                    assignment.into_syntax()
                } else {
                    break;
                }
            }
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                let assignment = JsAssignmentExpression::unwrap_cast(parent);
                let is_left = assignment
                    .left()
                    .is_ok_and(|left| left.syntax() == current.as_ref());
                if is_left {
                    assignment.into_syntax()
                } else {
                    break;
                }
            }
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                let cond = JsConditionalExpression::unwrap_cast(parent);
                let is_test = cond
                    .test()
                    .is_ok_and(|test| test.syntax() == current.as_ref());
                if is_test {
                    cond.into_syntax()
                } else {
                    break;
                }
            }
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                if mode == FirstInStatementMode::ExpressionStatementOrArrow =>
            {
                if is_not_first_iteration
                    && matches!(
                        current.kind(),
                        JsSyntaxKind::JS_SEQUENCE_EXPRESSION
                            | JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION
                            | JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT
                            | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
                    )
                {
                    // The original node doesn't need parens,
                    // because an ancestor requires parens.
                    break;
                }
                let arrow = JsArrowFunctionExpression::unwrap_cast(parent);
                let is_body = arrow.body().is_ok_and(|body| match body {
                    AnyJsFunctionBody::AnyJsExpression(expression) => {
                        expression.syntax() == current.as_ref()
                    }
                    _ => false,
                });
                return is_body;
            }
            JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
                if mode == FirstInStatementMode::ExpressionOrExportDefault =>
            {
                return !is_not_first_iteration;
            }
            _ => {
                let Some(binary_like) = AnyJsBinaryLikeExpression::cast(parent) else {
                    break;
                };
                let is_left = binary_like.left().is_ok_and(|left| match left {
                    AnyJsBinaryLikeLeftExpression::AnyJsExpression(expression) => {
                        expression.syntax() == current.as_ref()
                    }
                    _ => false,
                });
                if is_left {
                    binary_like.into_syntax()
                } else {
                    break;
                }
            }
        };
        is_not_first_iteration = true;
        current = Cow::Owned(parent);
    }
    false
}

// #[cfg(test)]
// mod tests {
//     use crate::*;

//     #[test]
//     fn js_arrow_function_expression() {
//         assert_needs_parentheses!("new (a => test)()`", JsArrowFunctionExpression);
//         assert_needs_parentheses!("(a => test)()", JsArrowFunctionExpression);
//         assert_needs_parentheses!("(a => test).member", JsArrowFunctionExpression);
//         assert_needs_parentheses!("(a => test)[member]", JsArrowFunctionExpression);
//         assert_not_needs_parentheses!("object[a => a]", JsArrowFunctionExpression);
//         assert_needs_parentheses!("(a => a) as Function", JsArrowFunctionExpression);
//         assert_needs_parentheses!("(a => a)!", JsArrowFunctionExpression);
//         assert_needs_parentheses!("(a => a)`template`", JsArrowFunctionExpression);
//         assert_needs_parentheses!("+(a => a)", JsArrowFunctionExpression);
//         assert_needs_parentheses!("(a => a) && b", JsArrowFunctionExpression);
//         assert_needs_parentheses!("(a => a) instanceof b", JsArrowFunctionExpression);
//         assert_needs_parentheses!("(a => a) in b", JsArrowFunctionExpression);
//         assert_needs_parentheses!("(a => a) + b", JsArrowFunctionExpression);
//         assert_needs_parentheses!("await (a => a)", JsArrowFunctionExpression);
//         assert_needs_parentheses!(
//             "<Function>(a => a)",
//             JsArrowFunctionExpression,
//             JsFileSource::ts()
//         );
//         assert_needs_parentheses!("(a => a) ? b : c", JsArrowFunctionExpression);
//         assert_not_needs_parentheses!("a ? b => b : c", JsArrowFunctionExpression);
//         assert_not_needs_parentheses!("a ? b : c => c", JsArrowFunctionExpression);
//         assert_needs_parentheses!("class Test extends (a => a) {}", JsArrowFunctionExpression);
//     }

//     #[test]
//     fn js_assignment_expression() {
//         assert_not_needs_parentheses!("({ [a = 3]: value })", JsAssignmentExpression);
//         assert_not_needs_parentheses!("class Test { [a = 3]: value }", JsAssignmentExpression);
//         assert_not_needs_parentheses!("type Test  = { [a = 3]: value }", JsAssignmentExpression);
//         assert_not_needs_parentheses!("interface Test { [a = 3]: value }", JsAssignmentExpression);

//         assert_needs_parentheses!("a => (a = 3)", JsAssignmentExpression);
//         assert_not_needs_parentheses!("a => { a = 3 }", JsAssignmentExpression);

//         assert_not_needs_parentheses!("for(a = 3;;) {}", JsAssignmentExpression);
//         assert_not_needs_parentheses!("for(a = 3, b = 2;;) {}", JsAssignmentExpression[1]);
//         assert_not_needs_parentheses!("for(a = 3, b = 2, c= 3;;) {}", JsAssignmentExpression[2]);
//         assert_needs_parentheses!("for(; a = 3; ) {}", JsAssignmentExpression);
//         assert_not_needs_parentheses!("for(;;a = 3) {}", JsAssignmentExpression);

//         assert_not_needs_parentheses!("for ((a, a = 3);;) {}", JsAssignmentExpression);
//         assert_needs_parentheses!("for (; (a, a = 3);) {}", JsAssignmentExpression);
//         assert_not_needs_parentheses!("for (;;(a, a = 3)) {}", JsAssignmentExpression);

//         assert_not_needs_parentheses!("a = 3", JsAssignmentExpression);
//         assert_needs_parentheses!("({ a } = { a: 3 })", JsAssignmentExpression);
//     }

//     fn js_await_expression() {
//         assert_needs_parentheses!("(await a)`template`", JsAwaitExpression);
//         assert_needs_parentheses!("+(await a)", JsAwaitExpression);

//         assert_needs_parentheses!("(await a).b", JsAwaitExpression);
//         assert_needs_parentheses!("(await a)[b]", JsAwaitExpression);
//         assert_not_needs_parentheses!("a[await b]", JsAwaitExpression);

//         assert_needs_parentheses!("(await a)()", JsAwaitExpression);
//         assert_needs_parentheses!("new (await a)()", JsAwaitExpression);

//         assert_needs_parentheses!("(await a) && b", JsAwaitExpression);
//         assert_needs_parentheses!("(await a) + b", JsAwaitExpression);
//         assert_needs_parentheses!("(await a) instanceof b", JsAwaitExpression);
//         assert_needs_parentheses!("(await a) in b", JsAwaitExpression);

//         assert_needs_parentheses!("[...(await a)]", JsAwaitExpression);
//         assert_needs_parentheses!("({...(await b)})", JsAwaitExpression);
//         assert_needs_parentheses!("call(...(await b))", JsAwaitExpression);

//         assert_needs_parentheses!("class A extends (await b) {}", JsAwaitExpression);

//         assert_needs_parentheses!("(await b) as number", JsAwaitExpression);
//         assert_needs_parentheses!("(await b)!", JsAwaitExpression);

//         assert_needs_parentheses!("(await b) ? b : c", JsAwaitExpression);
//         assert_not_needs_parentheses!("a ? await b : c", JsAwaitExpression);
//         assert_not_needs_parentheses!("a ? b : await c", JsAwaitExpression);
//     }

//     #[test]
//     fn js_binary_expression() {
//         assert_needs_parentheses!("class X extends (4 + 4) {}", JsBinaryExpression);

//         assert_needs_parentheses!("(4 + 4) as number", JsBinaryExpression);
//         assert_needs_parentheses!("<number>(4 + 4)", JsBinaryExpression);
//         assert_needs_parentheses!("!(4 + 4)", JsBinaryExpression);
//         assert_needs_parentheses!("await (4 + 4)", JsBinaryExpression);
//         assert_needs_parentheses!("(4 + 4)!", JsBinaryExpression);

//         assert_needs_parentheses!("(4 + 4)()", JsBinaryExpression);
//         assert_needs_parentheses!("(4 + 4)?.()", JsBinaryExpression);
//         assert_needs_parentheses!("new (4 + 4)()", JsBinaryExpression);
//         assert_needs_parentheses!("(4 + 4)`template`", JsBinaryExpression);
//         assert_needs_parentheses!("[...(4 + 4)]", JsBinaryExpression);
//         assert_needs_parentheses!("({...(4 + 4)})", JsBinaryExpression);
//         assert_needs_parentheses!(
//             "<test {...(4 + 4)} />",
//             JsBinaryExpression,
//             JsFileSource::tsx()
//         );
//         assert_needs_parentheses!(
//             "<test>{...(4 + 4)}</test>",
//             JsBinaryExpression,
//             JsFileSource::tsx()
//         );

//         assert_needs_parentheses!("(4 + 4).member", JsBinaryExpression);
//         assert_needs_parentheses!("(4 + 4)[member]", JsBinaryExpression);
//         assert_not_needs_parentheses!("object[4 + 4]", JsBinaryExpression);

//         assert_needs_parentheses!("(4 + 4) * 3", JsBinaryExpression[1]);
//         assert_not_needs_parentheses!("(4 + 4) * 3", JsBinaryExpression[0]);

//         assert_needs_parentheses!("a ** b ** c", JsBinaryExpression[1]);
//         assert_not_needs_parentheses!("a ** b ** c", JsBinaryExpression[0]);

//         assert_needs_parentheses!("a * r >> 5", JsBinaryExpression[1]);
//         assert_not_needs_parentheses!("a * r >> 5", JsBinaryExpression[0]);

//         assert_needs_parentheses!("a * r | 4", JsBinaryExpression[1]);
//         assert_not_needs_parentheses!("a * r | 5", JsBinaryExpression[0]);

//         assert_needs_parentheses!("a % 4 + 4", JsBinaryExpression[1]);
//         assert_not_needs_parentheses!("a % 4 + 4", JsBinaryExpression[0]);

//         assert_needs_parentheses!("a == b == c", JsBinaryExpression[1]);
//         assert_not_needs_parentheses!("a == b == c", JsBinaryExpression[0]);
//     }

//     #[test]
//     fn js_call_expression() {
//         assert_needs_parentheses!("new (call())()", JsCallExpression);

//         assert_not_needs_parentheses!("a?.()!.c", JsCallExpression);
//         assert_not_needs_parentheses!("(a?.())!.c", JsCallExpression);

//         assert_not_needs_parentheses!("(call())()", JsCallExpression[1]);
//         assert_not_needs_parentheses!("getLogger().error(err);", JsCallExpression[0]);
//         assert_not_needs_parentheses!("getLogger().error(err);", JsCallExpression[1]);
//     }

//     #[test]
//     fn js_class_expression() {
//         assert_needs_parentheses!("console.log((class {})())", JsClassExpression);
//         assert_needs_parentheses!("console.log(new (class {})())", JsClassExpression);

//         assert_needs_parentheses!("(class {}).test", JsClassExpression);
//         assert_not_needs_parentheses!("a => class {} ", JsClassExpression);

//         assert_needs_parentheses!("export default (class  {})", JsClassExpression);
//     }

//     #[test]
//     fn js_computed_member_expression() {
//         assert_needs_parentheses!("new (test()[a])()", JsComputedMemberExpression);
//         assert_needs_parentheses!("new (test().a[b])()", JsComputedMemberExpression);
//         assert_needs_parentheses!(
//             "new (test()`template`[index])()",
//             JsComputedMemberExpression
//         );
//         assert_needs_parentheses!("new (test()![member])()", JsComputedMemberExpression);

//         assert_not_needs_parentheses!("new (test[a])()", JsComputedMemberExpression);
//     }

//     #[test]
//     fn js_conditional_expression() {
//         assert_needs_parentheses!("((a ? b : c)).member", JsConditionalExpression);
//         assert_needs_parentheses!("(a ? b : c).member", JsConditionalExpression);
//         assert_needs_parentheses!("(a ? b : c)[member]", JsConditionalExpression);
//         assert_not_needs_parentheses!("object[(a ? b : c)]", JsConditionalExpression);

//         assert_needs_parentheses!("new (true ? A : B)()", JsConditionalExpression);
//         assert_not_needs_parentheses!("new A(a ? b : c)", JsConditionalExpression);

//         assert_needs_parentheses!("(true ? A : B)()", JsConditionalExpression);
//         assert_not_needs_parentheses!("call(a ? b : c)", JsConditionalExpression);

//         assert_needs_parentheses!(
//             "(a ? b : c)`tagged template literal`",
//             JsConditionalExpression
//         );
//         assert_not_needs_parentheses!("tag`content ${a ? b : c}`", JsConditionalExpression);

//         assert_needs_parentheses!("-(a ? b : c)", JsConditionalExpression);

//         assert_needs_parentheses!("[...(a ? b : c)]", JsConditionalExpression);
//         assert_needs_parentheses!("({...(a ? b : c)})", JsConditionalExpression);
//         assert_needs_parentheses!("call(...(a ? b : c))", JsConditionalExpression);

//         assert_needs_parentheses!("a + (b ? c : d)", JsConditionalExpression);
//         assert_needs_parentheses!("(a ? b : c) + d", JsConditionalExpression);

//         assert_needs_parentheses!("a instanceof (b ? c : d)", JsConditionalExpression);
//         assert_needs_parentheses!("(a ? b : c) instanceof d", JsConditionalExpression);

//         assert_needs_parentheses!("a in (b ? c : d)", JsConditionalExpression);
//         assert_needs_parentheses!("(a ? b : c) in d", JsConditionalExpression);

//         assert_needs_parentheses!("a && (b ? c : d)", JsConditionalExpression);
//         assert_needs_parentheses!("(a ? b : c) && d", JsConditionalExpression);

//         assert_needs_parentheses!("await (a ? b : c)", JsConditionalExpression);
//         assert_needs_parentheses!(
//             "<a {...(a ? b : c)} />",
//             JsConditionalExpression,
//             JsFileSource::tsx()
//         );

//         assert_needs_parentheses!("(a ? b : c) as number;", JsConditionalExpression);
//         assert_needs_parentheses!("<number>(a ? b : c);", JsConditionalExpression);

//         assert_needs_parentheses!("class Test extends (a ? B : C) {}", JsConditionalExpression);
//         assert_needs_parentheses!("(a ? B : C)!", JsConditionalExpression);

//         assert_not_needs_parentheses!("a ? b : c", JsConditionalExpression);
//         assert_not_needs_parentheses!("(a ? b : c)", JsConditionalExpression);

//         assert_needs_parentheses!("({ a: 'test' } ? B : C)!", JsConditionalExpression);
//         assert_not_needs_parentheses!(
//             "console.log({ a: 'test' } ? B : C )",
//             JsConditionalExpression
//         );
//         assert_not_needs_parentheses!("a ? b : c", JsConditionalExpression);
//     }

//     #[test]
//     fn js_function_expression() {
//         assert_needs_parentheses!("console.log((function () {})())", JsFunctionExpression);
//         assert_needs_parentheses!("console.log(new (function () {})())", JsFunctionExpression);

//         assert_needs_parentheses!("(function() {}).test", JsFunctionExpression);
//         assert_not_needs_parentheses!("a => function () {} ", JsFunctionExpression);

//         assert_needs_parentheses!(
//             "console.log((function () {})`template`)",
//             JsFunctionExpression
//         );

//         assert_needs_parentheses!("export default (function () {})", JsFunctionExpression);
//     }

//     #[test]
//     fn js_in_expression() {
//         assert_needs_parentheses!("class X extends (a in b) {}", JsInExpression);

//         assert_needs_parentheses!("(a in b) as number", JsInExpression);
//         assert_needs_parentheses!("<number>(a in b)", JsInExpression);
//         assert_needs_parentheses!("!(a in b)", JsInExpression);
//         assert_needs_parentheses!("await (a in b)", JsInExpression);
//         assert_needs_parentheses!("(a in b)!", JsInExpression);

//         assert_needs_parentheses!("(a in b)()", JsInExpression);
//         assert_needs_parentheses!("(a in b)?.()", JsInExpression);
//         assert_needs_parentheses!("new (a in b)()", JsInExpression);
//         assert_needs_parentheses!("(a in b)`template`", JsInExpression);
//         assert_needs_parentheses!("[...(a in b)]", JsInExpression);
//         assert_needs_parentheses!("({...(a in b)})", JsInExpression);
//         assert_needs_parentheses!(
//             "<test {...(a in b)} />",
//             JsInExpression,
//             JsFileSource::tsx()
//         );
//         assert_needs_parentheses!(
//             "<test>{...(a in b)}</test>",
//             JsInExpression,
//             JsFileSource::tsx()
//         );

//         assert_needs_parentheses!("(a in b).member", JsInExpression);
//         assert_needs_parentheses!("(a in b)[member]", JsInExpression);
//         assert_not_needs_parentheses!("object[a in b]", JsInExpression);

//         assert_needs_parentheses!("(a in b) + c", JsInExpression);

//         assert_not_needs_parentheses!("a in b > c", JsInExpression);
//         assert_not_needs_parentheses!("a in b instanceof C", JsInExpression);
//         assert_not_needs_parentheses!("a in b in c", JsInExpression[0]);
//         assert_not_needs_parentheses!("a in b in c", JsInExpression[1]);
//     }

//     #[test]
//     fn js_for_in_needs_parentheses() {
//         assert_needs_parentheses!("for (let a = (b in c);;);", JsInExpression);
//         assert_needs_parentheses!("for (a && (b in c);;);", JsInExpression);
//         assert_needs_parentheses!("for (a => (b in c);;);", JsInExpression);
//         assert_needs_parentheses!(
//             "function* g() {
//   for (yield (a in b);;);
// }",
//             JsInExpression
//         );
//         assert_needs_parentheses!(
//             "async function f() {
//   for (await (a in b);;);
// }",
//             JsInExpression
//         );

//         assert_not_needs_parentheses!("for (;a in b;);", JsInExpression);
//         assert_not_needs_parentheses!("for (;;a in b);", JsInExpression);
//         assert_not_needs_parentheses!(
//             r#"
//         for (function () { a in b }();;);
//         "#,
//             JsInExpression
//         );
//     }

//     #[test]
//     fn js_instanceof_expression() {
//         assert_needs_parentheses!(
//             "class X extends (a instanceof b) {}",
//             JsInstanceofExpression
//         );

//         assert_needs_parentheses!("(a instanceof B) as number", JsInstanceofExpression);
//         assert_needs_parentheses!("<number>(a instanceof B)", JsInstanceofExpression);
//         assert_needs_parentheses!("!(a instanceof B)", JsInstanceofExpression);
//         assert_needs_parentheses!("await (a instanceof B)", JsInstanceofExpression);
//         assert_needs_parentheses!("(a instanceof B)!", JsInstanceofExpression);

//         assert_needs_parentheses!("(a instanceof B)()", JsInstanceofExpression);
//         assert_needs_parentheses!("(a instanceof B)?.()", JsInstanceofExpression);
//         assert_needs_parentheses!("new (a instanceof B)()", JsInstanceofExpression);
//         assert_needs_parentheses!("(a instanceof B)`template`", JsInstanceofExpression);
//         assert_needs_parentheses!("[...(a instanceof B)]", JsInstanceofExpression);
//         assert_needs_parentheses!("({...(a instanceof B)})", JsInstanceofExpression);
//         assert_needs_parentheses!(
//             "<test {...(a instanceof B)} />",
//             JsInstanceofExpression,
//             JsFileSource::tsx()
//         );
//         assert_needs_parentheses!(
//             "<test>{...(a instanceof B)}</test>",
//             JsInstanceofExpression,
//             JsFileSource::tsx()
//         );

//         assert_needs_parentheses!("(a instanceof B).member", JsInstanceofExpression);
//         assert_needs_parentheses!("(a instanceof B)[member]", JsInstanceofExpression);
//         assert_not_needs_parentheses!("object[a instanceof B]", JsInstanceofExpression);

//         assert_needs_parentheses!("(a instanceof B) + c", JsInstanceofExpression);

//         assert_not_needs_parentheses!("a instanceof B > c", JsInstanceofExpression);
//         assert_not_needs_parentheses!("a instanceof B in c", JsInstanceofExpression);
//         assert_not_needs_parentheses!("a instanceof B instanceof c", JsInstanceofExpression[0]);
//         assert_not_needs_parentheses!("a instanceof B instanceof c", JsInstanceofExpression[1]);
//     }

//     #[test]
//     fn js_logical_expression() {
//         assert_needs_parentheses!("class X extends (a && b) {}", JsLogicalExpression);

//         assert_needs_parentheses!("(a && b) as number", JsLogicalExpression);
//         assert_needs_parentheses!("<number>(a && b)", JsLogicalExpression);
//         assert_needs_parentheses!("!(a && b)", JsLogicalExpression);
//         assert_needs_parentheses!("await (a && b)", JsLogicalExpression);
//         assert_needs_parentheses!("(a && b)!", JsLogicalExpression);

//         assert_needs_parentheses!("(a && b)()", JsLogicalExpression);
//         assert_needs_parentheses!("(a && b)?.()", JsLogicalExpression);
//         assert_needs_parentheses!("new (a && b)()", JsLogicalExpression);
//         assert_needs_parentheses!("(a && b)`template`", JsLogicalExpression);
//         assert_needs_parentheses!("[...(a && b)]", JsLogicalExpression);
//         assert_needs_parentheses!("({...(a && b)})", JsLogicalExpression);
//         assert_needs_parentheses!(
//             "<test {...(a && b)} />",
//             JsLogicalExpression,
//             JsFileSource::tsx()
//         );
//         assert_needs_parentheses!(
//             "<test>{...(a && b)}</test>",
//             JsLogicalExpression,
//             JsFileSource::tsx()
//         );

//         assert_needs_parentheses!("(a && b).member", JsLogicalExpression);
//         assert_needs_parentheses!("(a && b)[member]", JsLogicalExpression);
//         assert_not_needs_parentheses!("object[a && b]", JsLogicalExpression);

//         assert_needs_parentheses!("(a && b) || c", JsLogicalExpression[1]);
//         assert_needs_parentheses!("(a && b) in c", JsLogicalExpression);
//         assert_needs_parentheses!("(a && b) instanceof c", JsLogicalExpression);
//         assert_needs_parentheses!("(a && b) + c", JsLogicalExpression);

//         assert_not_needs_parentheses!("a && b && c", JsLogicalExpression[0]);
//         assert_not_needs_parentheses!("a && b && c", JsLogicalExpression[1]);
//     }

//     #[test]
//     fn js_number_literal_expression() {
//         assert_needs_parentheses!("(5).test", JsNumberLiteralExpression);
//         assert_needs_parentheses!("(5)[test]", JsNumberLiteralExpression);
//         assert_not_needs_parentheses!("test[5]", JsNumberLiteralExpression);
//     }

//     #[test]
//     fn js_object_expression() {
//         assert_needs_parentheses!("class A extends ({}) {}", JsObjectExpression);
//         assert_needs_parentheses!("({a: 5})", JsObjectExpression);
//         assert_needs_parentheses!("a => ({ a: 5})", JsObjectExpression);

//         assert_needs_parentheses!("a => ({ a: 'test' })`template`", JsObjectExpression);
//         assert_needs_parentheses!("({ a: 'test' }).member", JsObjectExpression);
//         assert_needs_parentheses!("({ a: 'test' })[member]", JsObjectExpression);
//         assert_needs_parentheses!("({ a: 'test' })()", JsObjectExpression);
//         assert_needs_parentheses!("new ({ a: 'test' })()", JsObjectExpression);
//         assert_needs_parentheses!("({ a: 'test' }) as number", JsObjectExpression);
//         assert_needs_parentheses!("({ a: 'test' })!", JsObjectExpression);
//         assert_needs_parentheses!("({ a: 'test' }), b, c", JsObjectExpression);
//         assert_needs_parentheses!("({ a: 'test' }) + 5", JsObjectExpression);
//         assert_needs_parentheses!("({ a: 'test' }) && true", JsObjectExpression);
//         assert_needs_parentheses!("({ a: 'test' }) instanceof A", JsObjectExpression);
//         assert_needs_parentheses!("({ a: 'test' }) in B", JsObjectExpression);

//         assert_not_needs_parentheses!("() => ({}, a);", JsObjectExpression);
//         assert_not_needs_parentheses!("() => (a = {});", JsObjectExpression);
//         assert_not_needs_parentheses!("() => ({}.prop = 0);", JsObjectExpression);
//         assert_not_needs_parentheses!("() => ({}['prop'] = 0);", JsObjectExpression);
//     }

//     #[test]
//     fn js_post_update_expression() {
//         assert_needs_parentheses!("class A extends (A++) {}", JsPostUpdateExpression);

//         assert_needs_parentheses!("(a++).b", JsPostUpdateExpression);
//         assert_needs_parentheses!("(a++)[b]", JsPostUpdateExpression);
//         assert_not_needs_parentheses!("a[b++]", JsPostUpdateExpression);

//         assert_needs_parentheses!("(a++)`template`", JsPostUpdateExpression);

//         assert_needs_parentheses!("(a++)()", JsPostUpdateExpression);
//         assert_needs_parentheses!("new (a++)()", JsPostUpdateExpression);

//         assert_needs_parentheses!("(a++)!", JsPostUpdateExpression);

//         assert_needs_parentheses!("(a++) ** 3", JsPostUpdateExpression);
//         assert_not_needs_parentheses!("(a++) + 3", JsPostUpdateExpression);
//     }

//     #[test]
//     fn js_pre_update_expression() {
//         // valid, but should become +(++a)
//         assert_needs_parentheses!("+ ++a", JsPreUpdateExpression);
//         assert_needs_parentheses!("class A extends (++A) {}", JsPreUpdateExpression);

//         assert_needs_parentheses!("(++a).b", JsPreUpdateExpression);
//         assert_needs_parentheses!("(++a)[b]", JsPreUpdateExpression);
//         assert_not_needs_parentheses!("a[++b]", JsPreUpdateExpression);

//         assert_needs_parentheses!("(++a)`template`", JsPreUpdateExpression);

//         assert_needs_parentheses!("(++a)()", JsPreUpdateExpression);
//         assert_needs_parentheses!("new (++a)()", JsPreUpdateExpression);

//         assert_needs_parentheses!("(++a)!", JsPreUpdateExpression);

//         assert_needs_parentheses!("(++a) ** 3", JsPreUpdateExpression);
//         assert_not_needs_parentheses!("(++a) + 3", JsPreUpdateExpression);
//     }

//     #[test]
//     fn js_sequence_expression() {
//         assert_not_needs_parentheses!("function test() { return a, b }", JsSequenceExpression);
//         assert_not_needs_parentheses!("for (let i, x; i++, x++;) {}", JsSequenceExpression);
//         assert_not_needs_parentheses!("a, b;", JsSequenceExpression);
//         assert_not_needs_parentheses!("a, b, c", JsSequenceExpression[0]);
//         assert_not_needs_parentheses!("a, b, c", JsSequenceExpression[1]);
//         assert_not_needs_parentheses!("a => a, b", JsSequenceExpression);
//     }

//     #[test]
//     fn js_static_member_expression() {
//         assert_needs_parentheses!("new (test().a)()", JsStaticMemberExpression);
//         assert_needs_parentheses!("new (test()[a].b)()", JsStaticMemberExpression);
//         assert_needs_parentheses!("new (test()`template`.length)()", JsStaticMemberExpression);
//         assert_needs_parentheses!("new (test()!.member)()", JsStaticMemberExpression);

//         assert_not_needs_parentheses!("new (test.a)()", JsStaticMemberExpression);
//     }

//     #[test]
//     fn js_string_literal_expression() {
//         assert_needs_parentheses!("{ 'test'; }", JsStringLiteralExpression);
//         assert_needs_parentheses!(
//             r#"
//             {
//                 console.log(5);
//                 'test';
//             }
//             "#,
//             JsStringLiteralExpression
//         );
//         assert_needs_parentheses!(
//             r#"
//             function Test () {
//                 ('test');
//             }
//             "#,
//             JsStringLiteralExpression
//         );
//         assert_needs_parentheses!(
//             r#"
//             class A {
//                 static {
//                     ('test');
//                 }
//             }
//             "#,
//             JsStringLiteralExpression
//         );
//         assert_needs_parentheses!(
//             "('test');",
//             JsStringLiteralExpression,
//             JsFileSource::ts().with_module_kind(ModuleKind::Module)
//         );

//         assert_not_needs_parentheses!("console.log('a')", JsStringLiteralExpression);
//     }

//     #[test]
//     fn js_unary_expression() {
//         assert_needs_parentheses!("class A extends (!B) {}", JsUnaryExpression);

//         assert_needs_parentheses!("(!foo) instanceof Bar", JsUnaryExpression);
//         assert_needs_parentheses!("(!foo) instanceof Bar", JsUnaryExpression);
//         assert_needs_parentheses!("(~foo) in bar", JsUnaryExpression);
//         assert_needs_parentheses!("(~foo) in bar", JsUnaryExpression);

//         assert_needs_parentheses!("(+a).b", JsUnaryExpression);
//         assert_needs_parentheses!("(+a)[b]", JsUnaryExpression);
//         assert_not_needs_parentheses!("a[+b]", JsUnaryExpression);

//         assert_needs_parentheses!("(+a)`template`", JsUnaryExpression);

//         assert_needs_parentheses!("(+a)()", JsUnaryExpression);
//         assert_needs_parentheses!("new (+a)()", JsUnaryExpression);

//         assert_needs_parentheses!("(+a)!", JsUnaryExpression);

//         assert_needs_parentheses!("(+a) ** 3", JsUnaryExpression);
//         assert_not_needs_parentheses!("(+a) + 3", JsUnaryExpression);
//     }

//     #[test]
//     fn js_yield_expression() {
//         assert_needs_parentheses!(
//             "function* test() { (yield a)`template` }",
//             JsYieldExpression
//         );
//         assert_needs_parentheses!("function* test() { +(yield a) }", JsYieldExpression);

//         assert_needs_parentheses!("function* test() { (yield a).b }", JsYieldExpression);
//         assert_needs_parentheses!("function* test() { (yield a)[b] }", JsYieldExpression);
//         assert_not_needs_parentheses!("function* test() { a[yield b] }", JsYieldExpression);

//         assert_needs_parentheses!("function* test() { (yield a)() }", JsYieldExpression);
//         assert_needs_parentheses!("function* test() { new (yield a)() }", JsYieldExpression);

//         assert_needs_parentheses!("function* test() { (yield a) && b }", JsYieldExpression);
//         assert_needs_parentheses!("function* test() { (yield a) + b }", JsYieldExpression);
//         assert_needs_parentheses!(
//             "function* test() { (yield a) instanceof b }",
//             JsYieldExpression
//         );
//         assert_needs_parentheses!("function* test() { (yield a) in b }", JsYieldExpression);

//         assert_needs_parentheses!("function* test() { [...(yield a)] }", JsYieldExpression);
//         assert_needs_parentheses!("function* test() { ({...(yield b)}) }", JsYieldExpression);
//         assert_needs_parentheses!("function* test() { call(...(yield b)) }", JsYieldExpression);

//         assert_needs_parentheses!(
//             "function* test() { class A extends (yield b) {} }",
//             JsYieldExpression
//         );

//         assert_needs_parentheses!(
//             "function* test() { (yield b) as number }",
//             JsYieldExpression
//         );
//         assert_needs_parentheses!("function* test() { (yield b)! }", JsYieldExpression);

//         assert_needs_parentheses!("function* test() { (yield b) ? b : c }", JsYieldExpression);
//         assert_not_needs_parentheses!("function* test() { a ? yield b : c }", JsYieldExpression);
//         assert_not_needs_parentheses!("function* test() { a ? b : yield c }", JsYieldExpression);
//     }

//     #[test]
//     fn ts_as_expression() {
//         assert_needs_parentheses!("5 as number ? true : false", TsAsExpression);
//         assert_needs_parentheses!("cond ? x as number : false", TsAsExpression);
//         assert_needs_parentheses!("cond ? true : x as number", TsAsExpression);

//         assert_needs_parentheses!("class X extends (B as number) {}", TsAsExpression);

//         assert_needs_parentheses!("(x as Function)()", TsAsExpression);
//         assert_needs_parentheses!("(x as Function)?.()", TsAsExpression);
//         assert_needs_parentheses!("new (x as Function)()", TsAsExpression);

//         assert_needs_parentheses!("<number>(x as any)", TsAsExpression);
//         assert_needs_parentheses!("(x as any)`template`", TsAsExpression);
//         assert_needs_parentheses!("!(x as any)", TsAsExpression);
//         assert_needs_parentheses!("[...(x as any)]", TsAsExpression);
//         assert_needs_parentheses!("({...(x as any)})", TsAsExpression);
//         assert_needs_parentheses!(
//             "<test {...(x as any)} />",
//             TsAsExpression,
//             JsFileSource::tsx()
//         );
//         assert_needs_parentheses!(
//             "<test>{...(x as any)}</test>",
//             TsAsExpression,
//             JsFileSource::tsx()
//         );
//         assert_needs_parentheses!("await (x as any)", TsAsExpression);
//         assert_needs_parentheses!("(x as any)!", TsAsExpression);

//         assert_needs_parentheses!("(x as any).member", TsAsExpression);
//         assert_needs_parentheses!("(x as any)[member]", TsAsExpression);
//         assert_not_needs_parentheses!("object[x as any]", TsAsExpression);

//         assert_needs_parentheses!("(x as any) + (y as any)", TsAsExpression[0]);
//         assert_needs_parentheses!("(x as any) + (y as any)", TsAsExpression[1]);

//         assert_needs_parentheses!("(x as any) && (y as any)", TsAsExpression[0]);
//         assert_needs_parentheses!("(x as any) && (y as any)", TsAsExpression[1]);

//         assert_needs_parentheses!("(x as any) in (y as any)", TsAsExpression[0]);
//         assert_needs_parentheses!("(x as any) in (y as any)", TsAsExpression[1]);

//         assert_needs_parentheses!("(x as any) instanceof (y as any)", TsAsExpression[0]);
//         assert_needs_parentheses!("(x as any) instanceof (y as any)", TsAsExpression[1]);

//         assert_not_needs_parentheses!("x as number as string", TsAsExpression[1]);

//         // default-exported function expressions require parentheses, otherwise
//         // the end of the function ends the export declaration, and the `as`
//         // gets treated as a new statement.
//         assert_needs_parentheses!(
//             "export default (function foo(){} as typeof console.log)",
//             TsAsExpression
//         );
//         assert_not_needs_parentheses!("export default foo as bar", TsAsExpression);
//     }

//     #[test]
//     fn ts_satisfies_expression() {
//         assert_needs_parentheses!("5 satisfies number ? true : false", TsSatisfiesExpression);
//         assert_needs_parentheses!("cond ? x satisfies number : false", TsSatisfiesExpression);
//         assert_needs_parentheses!("cond ? true : x satisfies number", TsSatisfiesExpression);

//         assert_needs_parentheses!(
//             "class X extends (B satisfies number) {}",
//             TsSatisfiesExpression
//         );

//         assert_needs_parentheses!("(x satisfies Function)()", TsSatisfiesExpression);
//         assert_needs_parentheses!("(x satisfies Function)?.()", TsSatisfiesExpression);
//         assert_needs_parentheses!("new (x satisfies Function)()", TsSatisfiesExpression);

//         assert_needs_parentheses!("<number>(x satisfies any)", TsSatisfiesExpression);
//         assert_needs_parentheses!("(x satisfies any)`template`", TsSatisfiesExpression);
//         assert_needs_parentheses!("!(x satisfies any)", TsSatisfiesExpression);
//         assert_needs_parentheses!("[...(x satisfies any)]", TsSatisfiesExpression);
//         assert_needs_parentheses!("({...(x satisfies any)})", TsSatisfiesExpression);
//         assert_needs_parentheses!(
//             "<test {...(x satisfies any)} />",
//             TsSatisfiesExpression,
//             JsFileSource::tsx()
//         );
//         assert_needs_parentheses!(
//             "<test>{...(x satisfies any)}</test>",
//             TsSatisfiesExpression,
//             JsFileSource::tsx()
//         );
//         assert_needs_parentheses!("await (x satisfies any)", TsSatisfiesExpression);
//         assert_needs_parentheses!("(x satisfies any)!", TsSatisfiesExpression);

//         assert_needs_parentheses!("(x satisfies any).member", TsSatisfiesExpression);
//         assert_needs_parentheses!("(x satisfies any)[member]", TsSatisfiesExpression);
//         assert_not_needs_parentheses!("object[x satisfies any]", TsSatisfiesExpression);

//         assert_needs_parentheses!(
//             "(x satisfies any) + (y satisfies any)",
//             TsSatisfiesExpression[0]
//         );
//         assert_needs_parentheses!(
//             "(x satisfies any) + (y satisfies any)",
//             TsSatisfiesExpression[1]
//         );

//         assert_needs_parentheses!(
//             "(x satisfies any) && (y satisfies any)",
//             TsSatisfiesExpression[0]
//         );
//         assert_needs_parentheses!(
//             "(x satisfies any) && (y satisfies any)",
//             TsSatisfiesExpression[1]
//         );

//         assert_needs_parentheses!(
//             "(x satisfies any) in (y satisfies any)",
//             TsSatisfiesExpression[0]
//         );
//         assert_needs_parentheses!(
//             "(x satisfies any) in (y satisfies any)",
//             TsSatisfiesExpression[1]
//         );

//         assert_needs_parentheses!(
//             "(x satisfies any) instanceof (y satisfies any)",
//             TsSatisfiesExpression[0]
//         );
//         assert_needs_parentheses!(
//             "(x satisfies any) instanceof (y satisfies any)",
//             TsSatisfiesExpression[1]
//         );

//         assert_not_needs_parentheses!(
//             "x satisfies number satisfies string",
//             TsSatisfiesExpression[1]
//         );
//     }

//     #[test]
//     fn ts_type_assertion_expression() {
//         assert_needs_parentheses!("(<number> x) as any", TsTypeAssertionExpression);

//         assert_needs_parentheses!("class X extends (<number>B) {}", TsTypeAssertionExpression);

//         assert_needs_parentheses!("(<Function>x)()", TsTypeAssertionExpression);
//         assert_needs_parentheses!("(<Function>x)?.()", TsTypeAssertionExpression);
//         assert_needs_parentheses!("new (<Function>x)()", TsTypeAssertionExpression);

//         assert_needs_parentheses!("<number>(<any>x)", TsTypeAssertionExpression[1]);
//         assert_needs_parentheses!("<number>(<any>x)", TsTypeAssertionExpression[1]);
//         assert_needs_parentheses!("(<any>x)`template`", TsTypeAssertionExpression);
//         assert_needs_parentheses!("!(<any>x)", TsTypeAssertionExpression);
//         assert_needs_parentheses!("[...(<any>x)]", TsTypeAssertionExpression);
//         assert_needs_parentheses!("({...(<any>x)})", TsTypeAssertionExpression);

//         assert_needs_parentheses!("await (<any>x)", TsTypeAssertionExpression);
//         assert_needs_parentheses!("(<any>x)!", TsTypeAssertionExpression);

//         assert_needs_parentheses!("(<any>x).member", TsTypeAssertionExpression);
//         assert_needs_parentheses!("(<any>x)[member]", TsTypeAssertionExpression);
//         assert_not_needs_parentheses!("object[<any>x]", TsTypeAssertionExpression);
//     }
// }
