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
    JsInExpression, JsInstanceofExpression, JsLogicalExpression, JsLogicalOperator, JsMetavariable,
    JsNewExpression, JsNewTargetExpression, JsNullLiteralExpression, JsNumberLiteralExpression,
    JsObjectExpression, JsParenthesizedExpression, JsPostUpdateExpression, JsPreUpdateExpression,
    JsPreUpdateOperator, JsRegexLiteralExpression, JsSequenceExpression, JsStaticMemberExpression,
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
            Self::JsMetavariable(expr) => expr.needs_parentheses(),
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

impl NeedsParentheses for JsMetavariable {
    fn needs_parentheses(&self) -> bool {
        false
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
        } else if self
            .operator()
            .is_ok_and(|operator| operator == JsLogicalOperator::NullishCoalescing)
            && self
                .syntax()
                .parent()
                .is_some_and(|parent| parent.kind() == JsSyntaxKind::JS_CONDITIONAL_EXPRESSION)
        {
            true
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
            JsSyntaxKind::JS_BINARY_EXPRESSION => {
                JsBinaryExpression::unwrap_cast(parent).operator()
                    == Ok(JsBinaryOperator::LeftShift)
            }
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
