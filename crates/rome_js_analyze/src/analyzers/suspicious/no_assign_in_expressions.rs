use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsArrayElement, AnyJsAssignment, AnyJsAssignmentPattern,
    AnyJsExpression, AnyJsObjectAssignmentPatternMember, AnyJsObjectMember,
    JsArrayAssignmentPattern, JsArrayAssignmentPatternRestElement,
    JsArrayAssignmentPatternRestElementFields, JsArrayExpression, JsAssignmentExpression,
    JsAssignmentOperator, JsAssignmentWithDefault, JsAssignmentWithDefaultFields,
    JsBogusAssignment, JsBogusExpression, JsComputedMemberAssignment,
    JsComputedMemberAssignmentFields, JsComputedMemberExpression, JsExpressionStatement,
    JsForStatement, JsIdentifierAssignment, JsIdentifierExpression, JsObjectAssignmentPattern,
    JsObjectAssignmentPatternFields, JsObjectAssignmentPatternPropertyFields, JsObjectExpression,
    JsParenthesizedAssignment, JsParenthesizedAssignmentFields, JsParenthesizedExpression,
    JsSequenceExpression, JsSpread, JsStaticMemberAssignment, JsStaticMemberAssignmentFields,
    JsStaticMemberExpression, JsSyntaxElement, JsSyntaxKind, TsAsAssignment, TsAsAssignmentFields,
    TsAsExpression, TsNonNullAssertionAssignment, TsNonNullAssertionAssignmentFields,
    TsNonNullAssertionExpression, TsSatisfiesAssignment, TsSatisfiesAssignmentFields,
    TsSatisfiesExpression, TsTypeAssertionAssignment, TsTypeAssertionAssignmentFields,
    TsTypeAssertionExpression,
};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxError, SyntaxResult};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow assignments in expressions.
    ///
    /// In expressions, it is common to mistype a comparison operator (such as `==`) as an assignment operator (such as `=`).
    /// Moreover, the use of assignments in expressions is confusing.
    /// Indeed, expressions are often considered as side-effect free.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// let a, b;
    /// a = (b = 1) + 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let a;
    /// if (a = 1) {
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function f(a) {
    ///     return a = 1;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// let a;
    /// a = 1;
    /// ```
    pub(crate) NoAssignInExpressions {
        version: "1.0.0",
        name: "noAssignInExpressions",
        recommended: true,
    }
}

impl Rule for NoAssignInExpressions {
    type Query = Ast<JsAssignmentExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let assign = ctx.query();
        let mut ancestor = assign
            .syntax()
            .ancestors()
            .take_while(|x| {
                // Allow parens and multiple assign such as `a = b = (c = d)`
                JsAssignmentExpression::can_cast(x.kind())
                    || JsParenthesizedExpression::can_cast(x.kind())
            })
            .last()?;
        let mut prev_ancestor = ancestor;
        ancestor = prev_ancestor.parent()?;
        while JsSequenceExpression::can_cast(ancestor.kind()) {
            // Allow statements separated by sequences such as `a = 1, b = 2`
            prev_ancestor = ancestor;
            ancestor = prev_ancestor.parent()?;
        }
        if JsExpressionStatement::can_cast(ancestor.kind()) {
            None
        } else if let Some(for_stmt) = JsForStatement::cast(ancestor) {
            if let Some(for_test) = for_stmt.test() {
                // Disallow assignment in test part of a `for`
                (for_test.syntax() == &prev_ancestor).then_some(())
            } else {
                // Allow assignment in initializer and update parts of a `for`
                None
            }
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let assign = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            assign.range(),
            markup! {
                "The "<Emphasis>"assignment"</Emphasis>" should not be in an "<Emphasis>"expression"</Emphasis>"."
            },
        ).note(
            "The use of assignments in expressions is confusing.\nExpressions are often considered as side-effect free."
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let assign = ctx.query();
        let op = assign.operator().ok()?;
        if let JsAssignmentOperator::Assign = op {
            let mut mutation = ctx.root().begin();
            let token = assign.operator_token().ok()?;
            let binary_expression = make::js_binary_expression(
                assign.left().ok()?.try_into_expression().ok()?,
                make::token(JsSyntaxKind::EQ3)
                    .with_leading_trivia_pieces(token.leading_trivia().pieces())
                    .with_trailing_trivia_pieces(token.trailing_trivia().pieces()),
                assign.right().ok()?,
            );
            mutation.replace_element(
                JsSyntaxElement::Node(assign.syntax().clone()),
                JsSyntaxElement::Node(binary_expression.into_syntax()),
            );
            Some(JsRuleAction {
                mutation,
                applicability: Applicability::MaybeIncorrect,
                category: ActionCategory::QuickFix,
                message: markup!("Did you mean '==='?").to_owned(),
            })
        } else {
            None
        }
    }
}

trait TryFromAssignment<T>: Sized {
    fn try_from_assignment(value: T) -> SyntaxResult<Self>;
}

pub trait TryIntoExpression<T>: Sized {
    fn try_into_expression(self) -> SyntaxResult<T>;
}

impl<T, U> TryIntoExpression<U> for T
where
    U: TryFromAssignment<T>,
{
    fn try_into_expression(self) -> SyntaxResult<U> {
        U::try_from_assignment(self)
    }
}

impl TryFromAssignment<AnyJsAssignment> for AnyJsExpression {
    fn try_from_assignment(value: AnyJsAssignment) -> SyntaxResult<AnyJsExpression> {
        let expression = match value {
            AnyJsAssignment::JsBogusAssignment(assigment) => {
                AnyJsExpression::JsBogusExpression(assigment.try_into_expression()?)
            }
            AnyJsAssignment::JsComputedMemberAssignment(assigment) => {
                AnyJsExpression::JsComputedMemberExpression(assigment.try_into_expression()?)
            }
            AnyJsAssignment::JsIdentifierAssignment(assigment) => {
                AnyJsExpression::JsIdentifierExpression(assigment.try_into_expression()?)
            }
            AnyJsAssignment::JsParenthesizedAssignment(assigment) => {
                AnyJsExpression::JsParenthesizedExpression(assigment.try_into_expression()?)
            }
            AnyJsAssignment::JsStaticMemberAssignment(assigment) => {
                AnyJsExpression::JsStaticMemberExpression(assigment.try_into_expression()?)
            }
            AnyJsAssignment::TsAsAssignment(assigment) => {
                AnyJsExpression::TsAsExpression(assigment.try_into_expression()?)
            }
            AnyJsAssignment::TsNonNullAssertionAssignment(assigment) => {
                AnyJsExpression::TsNonNullAssertionExpression(assigment.try_into_expression()?)
            }
            AnyJsAssignment::TsSatisfiesAssignment(assigment) => {
                AnyJsExpression::TsSatisfiesExpression(assigment.try_into_expression()?)
            }
            AnyJsAssignment::TsTypeAssertionAssignment(assigment) => {
                AnyJsExpression::TsTypeAssertionExpression(assigment.try_into_expression()?)
            }
        };

        Ok(expression)
    }
}

impl TryFromAssignment<JsArrayAssignmentPattern> for JsArrayExpression {
    fn try_from_assignment(value: JsArrayAssignmentPattern) -> SyntaxResult<Self> {
        let mut elements = Vec::new();
        let mut separators = Vec::new();

        for element in value.elements().elements() {
            if let Ok(Some(separator)) = element.trailing_separator {
                separators.push(separator);
            }

            let element = match element.node? {
                AnyJsArrayAssignmentPatternElement::AnyJsAssignmentPattern(assignment) => {
                    AnyJsArrayElement::AnyJsExpression(assignment.try_into_expression()?)
                }
                AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(
                    assignment,
                ) => AnyJsArrayElement::JsSpread(assignment.try_into_expression()?),
                AnyJsArrayAssignmentPatternElement::JsArrayHole(hole) => {
                    AnyJsArrayElement::JsArrayHole(hole)
                }
                AnyJsArrayAssignmentPatternElement::JsAssignmentWithDefault(assignment) => {
                    AnyJsArrayElement::AnyJsExpression(AnyJsExpression::JsAssignmentExpression(
                        assignment.try_into_expression()?,
                    ))
                }
            };
            elements.push(element);
        }

        let elements = make::js_array_element_list(elements.into_iter(), separators.into_iter());

        let expression =
            make::js_array_expression(value.l_brack_token()?, elements, value.r_brack_token()?);

        Ok(expression)
    }
}

impl TryFromAssignment<JsArrayAssignmentPatternRestElement> for JsSpread {
    fn try_from_assignment(value: JsArrayAssignmentPatternRestElement) -> SyntaxResult<JsSpread> {
        let JsArrayAssignmentPatternRestElementFields {
            dotdotdot_token,
            pattern,
        } = value.as_fields();

        Ok(make::js_spread(
            dotdotdot_token?,
            pattern?.try_into_expression()?,
        ))
    }
}

impl TryFromAssignment<JsObjectAssignmentPattern> for JsObjectExpression {
    fn try_from_assignment(value: JsObjectAssignmentPattern) -> SyntaxResult<JsObjectExpression> {
        let JsObjectAssignmentPatternFields {
            l_curly_token,
            properties,
            r_curly_token,
        } = value.as_fields();

        let mut members = Vec::new();
        let mut separators = Vec::new();

        for property in properties.elements() {
            if let Ok(Some(separator)) = property.trailing_separator {
                separators.push(separator);
            }

            let member = match property.node? {
                AnyJsObjectAssignmentPatternMember::JsBogusAssignment(assigment) => {
                    AnyJsObjectMember::JsBogusMember(make::js_bogus_member([Some(
                        JsSyntaxElement::Node(assigment.into_syntax()),
                    )]))
                }
                AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(
                    assigment,
                ) => {
                    let JsObjectAssignmentPatternPropertyFields {
                        member,
                        colon_token,
                        pattern,
                        init,
                    } = assigment.as_fields();

                    if init.is_some() {
                        return Err(SyntaxError::MissingRequiredChild);
                    } else {
                        let member = make::js_property_object_member(
                            member?,
                            colon_token?,
                            pattern?.try_into_expression()?,
                        );
                        AnyJsObjectMember::JsPropertyObjectMember(member)
                    }
                }
                AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(assigment) => {
                    let spread = make::js_spread(
                        assigment.dotdotdot_token()?,
                        assigment.target()?.try_into_expression()?,
                    );
                    AnyJsObjectMember::JsSpread(spread)
                }
                AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                    assigment,
                ) => {
                    if assigment.init().is_some() {
                        return Err(SyntaxError::MissingRequiredChild);
                    } else {
                        let member = make::js_shorthand_property_object_member(
                            make::js_reference_identifier(assigment.identifier()?.name_token()?),
                        );
                        AnyJsObjectMember::JsShorthandPropertyObjectMember(member)
                    }
                }
            };

            members.push(member);
        }

        let member_list = make::js_object_member_list(members.into_iter(), separators.into_iter());

        let expression = make::js_object_expression(l_curly_token?, member_list, r_curly_token?);

        Ok(expression)
    }
}

impl TryFromAssignment<JsAssignmentWithDefault> for JsAssignmentExpression {
    fn try_from_assignment(value: JsAssignmentWithDefault) -> SyntaxResult<JsAssignmentExpression> {
        let JsAssignmentWithDefaultFields {
            pattern,
            eq_token,
            default,
        } = value.as_fields();

        Ok(make::js_assignment_expression(
            pattern?, eq_token?, default?,
        ))
    }
}

impl TryFromAssignment<AnyJsAssignmentPattern> for AnyJsExpression {
    fn try_from_assignment(value: AnyJsAssignmentPattern) -> SyntaxResult<AnyJsExpression> {
        let expression = match value {
            AnyJsAssignmentPattern::AnyJsAssignment(assigment) => {
                assigment.try_into_expression()?
            }
            AnyJsAssignmentPattern::JsArrayAssignmentPattern(assigment) => {
                AnyJsExpression::JsArrayExpression(assigment.try_into_expression()?)
            }
            AnyJsAssignmentPattern::JsObjectAssignmentPattern(assigment) => {
                AnyJsExpression::JsObjectExpression(assigment.try_into_expression()?)
            }
        };

        Ok(expression)
    }
}

impl TryFromAssignment<JsBogusAssignment> for JsBogusExpression {
    fn try_from_assignment(value: JsBogusAssignment) -> SyntaxResult<JsBogusExpression> {
        Ok(make::js_bogus_expression([Some(JsSyntaxElement::Node(
            value.into_syntax(),
        ))]))
    }
}

impl TryFromAssignment<JsComputedMemberAssignment> for JsComputedMemberExpression {
    fn try_from_assignment(
        value: JsComputedMemberAssignment,
    ) -> SyntaxResult<JsComputedMemberExpression> {
        let JsComputedMemberAssignmentFields {
            object,
            l_brack_token,
            member,
            r_brack_token,
        } = value.as_fields();

        Ok(
            make::js_computed_member_expression(object?, l_brack_token?, member?, r_brack_token?)
                .build(),
        )
    }
}

impl TryFromAssignment<JsIdentifierAssignment> for JsIdentifierExpression {
    fn try_from_assignment(value: JsIdentifierAssignment) -> SyntaxResult<JsIdentifierExpression> {
        Ok(make::js_identifier_expression(
            make::js_reference_identifier(value.name_token()?),
        ))
    }
}

impl TryFromAssignment<JsParenthesizedAssignment> for JsParenthesizedExpression {
    fn try_from_assignment(
        value: JsParenthesizedAssignment,
    ) -> SyntaxResult<JsParenthesizedExpression> {
        let JsParenthesizedAssignmentFields {
            l_paren_token,
            assignment,
            r_paren_token,
        } = value.as_fields();

        Ok(make::js_parenthesized_expression(
            l_paren_token?,
            assignment?.try_into_expression()?,
            r_paren_token?,
        ))
    }
}

impl TryFromAssignment<JsStaticMemberAssignment> for JsStaticMemberExpression {
    fn try_from_assignment(
        value: JsStaticMemberAssignment,
    ) -> SyntaxResult<JsStaticMemberExpression> {
        let JsStaticMemberAssignmentFields {
            object,
            dot_token,
            member,
        } = value.as_fields();

        Ok(make::js_static_member_expression(
            object?, dot_token?, member?,
        ))
    }
}

impl TryFromAssignment<TsAsAssignment> for TsAsExpression {
    fn try_from_assignment(value: TsAsAssignment) -> SyntaxResult<TsAsExpression> {
        let TsAsAssignmentFields {
            assignment,
            as_token,
            ty,
        } = value.as_fields();

        Ok(make::ts_as_expression(
            assignment?.try_into_expression()?,
            as_token?,
            ty?,
        ))
    }
}

impl TryFromAssignment<TsNonNullAssertionAssignment> for TsNonNullAssertionExpression {
    fn try_from_assignment(
        value: TsNonNullAssertionAssignment,
    ) -> SyntaxResult<TsNonNullAssertionExpression> {
        let TsNonNullAssertionAssignmentFields {
            assignment,
            excl_token,
        } = value.as_fields();

        Ok(make::ts_non_null_assertion_expression(
            assignment?.try_into_expression()?,
            excl_token?,
        ))
    }
}

impl TryFromAssignment<TsSatisfiesAssignment> for TsSatisfiesExpression {
    fn try_from_assignment(value: TsSatisfiesAssignment) -> SyntaxResult<TsSatisfiesExpression> {
        let TsSatisfiesAssignmentFields {
            assignment,
            satisfies_token,
            ty,
        } = value.as_fields();

        Ok(make::ts_satisfies_expression(
            assignment?.try_into_expression()?,
            satisfies_token?,
            ty?,
        ))
    }
}

impl TryFromAssignment<TsTypeAssertionAssignment> for TsTypeAssertionExpression {
    fn try_from_assignment(
        value: TsTypeAssertionAssignment,
    ) -> SyntaxResult<TsTypeAssertionExpression> {
        let TsTypeAssertionAssignmentFields {
            l_angle_token,
            ty,
            r_angle_token,
            assignment,
        } = value.as_fields();

        Ok(make::ts_type_assertion_expression(
            l_angle_token?,
            ty?,
            r_angle_token?,
            assignment?.try_into_expression()?,
        ))
    }
}
