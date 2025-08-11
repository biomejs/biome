use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsCallArgument, AnyJsExpression,
    AnyJsLiteralExpression, JsAssignmentExpression, JsAssignmentOperator, JsBinaryExpression,
    JsBinaryOperator, JsLanguage, JsUnaryExpression, JsUnaryOperator, T,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPieceKind, declare_node_union};
use biome_rule_options::no_implicit_coercions::NoImplicitCoercionsOptions;

declare_lint_rule! {
    /// Disallow shorthand type conversions.
    ///
    /// JavaScript allows shorthand type conversions by using operators like `!!`, `+`, `~`, etc.
    /// These shortcuts can make the code harder to read and understand, especially for developers
    /// who are not familiar with these patterns. Using explicit type conversion functions like
    /// `Boolean()`, `Number()`, and `String()` makes the intent clearer and more readable.
    ///
    /// This rule reports when values are converted to:
    /// - Boolean using double negation `!!value`
    /// - Number using unary plus `+value`, subtraction from zero `value - 0`, multiplication by one `value * 1`, division by one `value / 1`, or double negation with minus `-(-value)`
    /// - String using concatenation with empty string `value + ""` or empty template literal `` value + `` ``
    /// - Check index using bitwise NOT with indexOf `~value.indexOf(item)` instead of comparing with -1
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// !!foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// +foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// -(-foo);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo - 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo * 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo / 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// "" + foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo + "";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// `` + foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo += "";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// ~foo.indexOf(1);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Boolean(foo);
    /// ```
    ///
    /// ```js
    /// Number(foo);
    /// ```
    ///
    /// ```js
    /// String(foo);
    /// ```
    ///
    /// ```js
    /// foo.indexOf(1) !== -1;
    /// ```
    ///
    /// These are not flagged because they don't perform type coercion:
    /// ```js
    /// !foo;
    /// ~foo;
    /// -foo;
    /// +1234;
    /// 2 * foo;
    /// foo + 'bar';
    /// ```
    ///
    pub NoImplicitCoercions {
        version: "2.1.0",
        name: "noImplicitCoercions",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::Eslint("no-implicit-coercion").same(),
        ],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoImplicitCoercions {
    type Query = Ast<PotentialImplicitCoercion>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoImplicitCoercionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            PotentialImplicitCoercion::JsUnaryExpression(unary_expression) => {
                match unary_expression.operator().ok()? {
                    // +arg
                    JsUnaryOperator::Plus => {
                        let argument = unary_expression.argument().ok()?;
                        if !argument.is_number() {
                            Some(RuleState::ExpressionToTypeCall(ExpressionToTypeCall {
                                expression: unary_expression.clone().into(),
                                argument,
                                type_name: "Number",
                            }))
                        } else {
                            None
                        }
                    }
                    // -(-arg)
                    JsUnaryOperator::Minus => {
                        let argument = unary_expression.get_arg_for_double_operation()?;
                        if !argument.is_number() {
                            Some(RuleState::ExpressionToTypeCall(ExpressionToTypeCall {
                                expression: unary_expression.clone().into(),
                                argument,
                                type_name: "Number",
                            }))
                        } else {
                            None
                        }
                    }
                    // ~collection.indexOf(item)
                    JsUnaryOperator::BitwiseNot => {
                        let argument = unary_expression.argument().ok()?;
                        if argument.is_index_of_call() {
                            Some(RuleState::ExpressionToMinusOneComparison(
                                ExpressionToMinusOneComparison {
                                    expression: unary_expression.clone().into(),
                                    argument,
                                },
                            ))
                        } else {
                            None
                        }
                    }
                    // !!arg
                    JsUnaryOperator::LogicalNot => {
                        let argument = unary_expression.get_arg_for_double_operation()?;
                        Some(RuleState::ExpressionToTypeCall(ExpressionToTypeCall {
                            expression: unary_expression.clone().into(),
                            argument,
                            type_name: "Boolean",
                        }))
                    }
                    _ => None,
                }
            }
            PotentialImplicitCoercion::JsBinaryExpression(binary_expression) => {
                let operator = binary_expression.operator().ok()?;
                match operator {
                    // arg + "" | arg + ``
                    JsBinaryOperator::Plus => {
                        let argument = binary_expression
                            .get_another_arg_if_one_matches(|arg| arg.is_empty_string())?;

                        Some(RuleState::ExpressionToTypeCall(ExpressionToTypeCall {
                            expression: binary_expression.clone().into(),
                            argument,
                            type_name: "String",
                        }))
                    }
                    // arg - 0
                    JsBinaryOperator::Minus => {
                        let argument = binary_expression
                            .get_another_arg_if_one_matches(|arg| arg.is_zero())?;

                        if !argument.is_number() {
                            Some(RuleState::ExpressionToTypeCall(ExpressionToTypeCall {
                                expression: binary_expression.clone().into(),
                                argument,
                                type_name: "Number",
                            }))
                        } else {
                            None
                        }
                    }
                    // arg * 1
                    JsBinaryOperator::Times => {
                        let argument =
                            binary_expression.get_another_arg_if_one_matches(|arg| arg.is_one())?;

                        if !argument.is_number() {
                            Some(RuleState::ExpressionToTypeCall(ExpressionToTypeCall {
                                expression: binary_expression.clone().into(),
                                argument,
                                type_name: "Number",
                            }))
                        } else {
                            None
                        }
                    }
                    // arg / 1
                    JsBinaryOperator::Divide => {
                        let (left, right) = (
                            binary_expression.left().ok()?,
                            binary_expression.right().ok()?,
                        );

                        if !left.is_number() && right.is_one() {
                            Some(RuleState::ExpressionToTypeCall(ExpressionToTypeCall {
                                expression: binary_expression.clone().into(),
                                argument: left,
                                type_name: "Number",
                            }))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            PotentialImplicitCoercion::JsAssignmentExpression(assignment_expression) => {
                let assignment = assignment_expression.left().ok()?;
                let assignment = assignment.as_any_js_assignment()?;
                let expression = assignment_expression.right().ok()?;
                match assignment_expression.operator().ok()? {
                    // arg += "" | arg += ``
                    JsAssignmentOperator::AddAssign => {
                        if expression.is_empty_string() {
                            Some(RuleState::AssignmentToTypeCall(AssignmentToTypeCall {
                                assignment_expression: assignment_expression.clone(),
                                assignment: assignment.clone(),
                                type_name: "String",
                            }))
                        } else {
                            None
                        }
                    }

                    // arg -= 0
                    JsAssignmentOperator::SubtractAssign => {
                        if expression.is_zero() {
                            Some(RuleState::AssignmentToTypeCall(AssignmentToTypeCall {
                                assignment_expression: assignment_expression.clone(),
                                assignment: assignment.clone(),
                                type_name: "Number",
                            }))
                        } else {
                            None
                        }
                    }

                    // arg *= 1 | arg /= 1
                    JsAssignmentOperator::TimesAssign | JsAssignmentOperator::SlashAssign => {
                        if expression.is_one() {
                            Some(RuleState::AssignmentToTypeCall(AssignmentToTypeCall {
                                assignment_expression: assignment_expression.clone(),
                                assignment: assignment.clone(),
                                type_name: "Number",
                            }))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                {
                    if matches!(state, RuleState::ExpressionToMinusOneComparison(_)) {
                        "Using binary operations instead of comparisons is harder to read and understand."
                    } else {
                        "Implicit type conversion is hard to read and understand."
                    }
                }
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleAction<JsLanguage>> {
        let mut mutation = ctx.root().begin();
        match state {
            RuleState::ExpressionToTypeCall(expression_info) => {
                // This is a special case when replacing an expression which is a typeof argument.
                // We need to add a space after typeof, otherwise `typeof+x` -> `typeofNumber(x)`.
                if let Some(parent) = expression_info.expression.parent::<JsUnaryExpression>()
                    && let Ok(operator_token) = parent.operator_token()
                    && operator_token.trailing_trivia().is_empty()
                {
                    mutation.replace_token_discard_trivia(
                        operator_token.clone(),
                        operator_token.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    )
                }

                let new_expression = expression_info
                    .argument
                    .wrap_in_type_converter(expression_info.type_name);

                mutation.replace_node(expression_info.expression.clone(), new_expression);
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! {
                        "Use "<Emphasis>{expression_info.type_name}"()"</Emphasis>" call instead."
                    }
                    .to_owned(),
                    mutation,
                ))
            }
            RuleState::AssignmentToTypeCall(assignment_info) => {
                mutation.replace_node(
                    assignment_info.assignment_expression.clone(),
                    make::js_assignment_expression(
                        AnyJsAssignmentPattern::AnyJsAssignment(assignment_info.assignment.clone()),
                        make::token(T![=])
                            .with_trailing_trivia(Some((TriviaPieceKind::Whitespace, " "))),
                        assignment_info
                            .assignment
                            .as_expression()?
                            .wrap_in_type_converter(assignment_info.type_name),
                    ),
                );

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! {
                        "Use "<Emphasis>{assignment_info.type_name}"()"</Emphasis>" call instead."
                    }
                    .to_owned(),
                    mutation,
                ))
            }
            RuleState::ExpressionToMinusOneComparison(expression_info) => {
                mutation.replace_node(
                    expression_info.expression.clone(),
                    AnyJsExpression::JsParenthesizedExpression(make::js_parenthesized_expression(
                        make::token(T!['(']),
                        make::js_binary_expression(
                            expression_info.argument.clone(),
                            make::token(T![!==])
                                .with_leading_trivia(Some((TriviaPieceKind::Whitespace, " ")))
                                .with_trailing_trivia(Some((TriviaPieceKind::Whitespace, " "))),
                            AnyJsExpression::AnyJsLiteralExpression(
                                AnyJsLiteralExpression::JsNumberLiteralExpression(
                                    make::js_number_literal_expression(make::js_number_literal(-1)),
                                ),
                            ),
                        )
                        .into(),
                        make::token(T![')']),
                    )),
                );

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! {
                        "Compare with "<Emphasis>"-1"</Emphasis>" instead."
                    }
                    .to_owned(),
                    mutation,
                ))
            }
        }
    }
}

#[derive(Debug)]
pub enum RuleState {
    /// I.e. +arg -> Number(arg)
    ExpressionToTypeCall(ExpressionToTypeCall),
    /// I.e. arg += "" -> arg = String(arg)
    AssignmentToTypeCall(AssignmentToTypeCall),
    /// ~collection.indexOf(item) -> collection.indexOf(item) !== -1
    ExpressionToMinusOneComparison(ExpressionToMinusOneComparison),
}

#[derive(Debug)]
pub struct ExpressionToTypeCall {
    pub expression: AnyJsExpression,
    pub argument: AnyJsExpression,
    pub type_name: &'static str,
}

#[derive(Debug)]
pub struct ExpressionToMinusOneComparison {
    pub expression: AnyJsExpression,
    pub argument: AnyJsExpression,
}

#[derive(Debug)]
pub struct AssignmentToTypeCall {
    pub assignment_expression: JsAssignmentExpression,
    pub assignment: AnyJsAssignment,
    pub type_name: &'static str,
}

declare_node_union! {
    pub PotentialImplicitCoercion =
        JsUnaryExpression
        | JsBinaryExpression
        | JsAssignmentExpression
}

const TO_NUMBER_METHODS: &[&str] = &["Number", "parseInt", "parseFloat"];

trait ExpressionExt {
    /// Returns the actual expression, i.e. in case of parenthesized expressions
    /// it returns the inner expression.
    fn inner_expression(&self) -> Option<AnyJsExpression>;

    /// Returns true if the expression is an empty string literal.
    fn is_empty_string(&self) -> bool {
        let Some(expression) = self.inner_expression() else {
            return false;
        };
        match expression {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(string_literal),
            ) => string_literal
                .inner_string_text()
                .ok()
                .filter(|text| text.is_empty())
                .is_some(),
            AnyJsExpression::JsTemplateExpression(template_expression) => {
                template_expression.elements().len() == 0
            }
            _ => false,
        }
    }

    fn is_index_of_call(&self) -> bool {
        let Some(expression) = self.inner_expression() else {
            return false;
        };
        if let AnyJsExpression::JsCallExpression(call_expression) = expression
            && let Ok(callee) = call_expression.callee()
        {
            let Some(callee) = callee.inner_expression() else {
                return false;
            };
            return callee
                .get_callee_member_name()
                .filter(|name| name.text_trimmed() == "indexOf")
                .is_some();
        }
        false
    }

    /// Checks if expressions is an explicit number. We don't need to check the exact type of
    /// the expression for this rule, we just want to avoid obvious problems.
    fn is_number(&self) -> bool {
        let Some(expression) = self.inner_expression() else {
            return false;
        };
        match expression {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(_),
            ) => {
                return true;
            }
            AnyJsExpression::JsCallExpression(call) => {
                if let Ok(callee) = call.callee()
                    && let Some(AnyJsExpression::JsIdentifierExpression(ident)) =
                        callee.inner_expression()
                    && let Ok(name) = ident.name()
                    && let Ok(token) = name.value_token()
                {
                    return TO_NUMBER_METHODS.contains(&token.text_trimmed());
                }
            }
            AnyJsExpression::JsUnaryExpression(expr) => {
                if let Ok(operator) = expr.operator() {
                    return matches!(
                        operator,
                        JsUnaryOperator::Plus
                            | JsUnaryOperator::Minus
                            | JsUnaryOperator::BitwiseNot
                    );
                }
            }
            AnyJsExpression::JsBinaryExpression(expr) => {
                if let Ok(operator) = expr.operator() {
                    if matches!(
                        operator,
                        JsBinaryOperator::Minus
                            | JsBinaryOperator::Times
                            | JsBinaryOperator::Divide
                            | JsBinaryOperator::Exponent
                            | JsBinaryOperator::Remainder
                            | JsBinaryOperator::LeftShift
                            | JsBinaryOperator::RightShift
                            | JsBinaryOperator::UnsignedRightShift
                            | JsBinaryOperator::BitwiseAnd
                            | JsBinaryOperator::BitwiseOr
                            | JsBinaryOperator::BitwiseXor
                    ) {
                        return true;
                    }
                    if matches!(operator, JsBinaryOperator::Plus)
                        && let (Ok(left), Ok(right)) = (expr.left(), expr.right())
                    {
                        return left.is_number() && right.is_number();
                    }
                }
            }
            _ => {}
        }
        false
    }

    fn is_one(&self) -> bool {
        let Some(expression) = self.inner_expression() else {
            return false;
        };
        if let AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(number_literal),
        ) = expression
            && let Ok(token) = number_literal.value_token()
        {
            return token.text_trimmed() == "1";
        }
        false
    }

    fn is_zero(&self) -> bool {
        let Some(expression) = self.inner_expression() else {
            return false;
        };
        if let AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(number_literal),
        ) = expression
            && let Ok(token) = number_literal.value_token()
        {
            return token.text_trimmed() == "0";
        }
        false
    }

    fn wrap_in_type_converter(&self, name: &'static str) -> Self;
}

impl ExpressionExt for AnyJsExpression {
    fn inner_expression(&self) -> Option<Self> {
        Self::inner_expression(self)
    }

    fn wrap_in_type_converter(&self, name: &'static str) -> Self {
        let clean_expression = self
            .clone()
            .with_leading_trivia_pieces([])
            .and_then(|expression| {
                if let Some(last_token) = expression.syntax().last_token()
                    && last_token.has_trailing_comments()
                {
                    return expression.trim_trailing_trivia();
                }
                expression.with_trailing_trivia_pieces([])
            })
            .unwrap_or_else(|| self.clone());

        Self::JsCallExpression(
            make::js_call_expression(
                make::js_identifier_expression(make::js_reference_identifier(make::ident(name)))
                    .into(),
                make::js_call_arguments(
                    make::token(T!['(']),
                    make::js_call_argument_list(
                        [AnyJsCallArgument::AnyJsExpression(clean_expression)],
                        [],
                    ),
                    make::token(T![')']),
                ),
            )
            .build(),
        )
    }
}

trait AssignmentExt {
    fn as_expression(&self) -> Option<AnyJsExpression>;
}

impl AssignmentExt for AnyJsAssignment {
    fn as_expression(&self) -> Option<AnyJsExpression> {
        match self {
            Self::JsComputedMemberAssignment(member_assignment) => Some(
                make::js_computed_member_expression(
                    member_assignment.object().ok()?,
                    member_assignment.l_brack_token().ok()?,
                    member_assignment.member().ok()?,
                    member_assignment.r_brack_token().ok()?,
                )
                .build()
                .into(),
            ),
            Self::JsIdentifierAssignment(identifier) => Some(
                make::js_identifier_expression(make::js_reference_identifier(
                    identifier.name_token().ok()?,
                ))
                .into(),
            ),
            Self::JsParenthesizedAssignment(parenthesized_assignment) => {
                parenthesized_assignment.assignment().ok()?.as_expression()
            }

            Self::JsStaticMemberAssignment(static_member_assignment) => Some(
                make::js_static_member_expression(
                    static_member_assignment.object().ok()?,
                    static_member_assignment.dot_token().ok()?,
                    static_member_assignment.member().ok()?,
                )
                .into(),
            ),
            Self::TsAsAssignment(as_assignment) => Some(
                make::ts_as_expression(
                    as_assignment.assignment().ok()?.as_expression()?,
                    as_assignment.as_token().ok()?,
                    as_assignment.ty().ok()?,
                )
                .into(),
            ),
            Self::TsNonNullAssertionAssignment(non_null_assertion_assignment) => Some(
                make::ts_non_null_assertion_expression(
                    non_null_assertion_assignment
                        .assignment()
                        .ok()?
                        .as_expression()?,
                    non_null_assertion_assignment.excl_token().ok()?,
                )
                .into(),
            ),
            Self::TsSatisfiesAssignment(satisfies_assignment) => Some(
                make::ts_satisfies_expression(
                    satisfies_assignment.assignment().ok()?.as_expression()?,
                    satisfies_assignment.satisfies_token().ok()?,
                    satisfies_assignment.ty().ok()?,
                )
                .into(),
            ),
            Self::TsTypeAssertionAssignment(type_assertion_assignment) => Some(
                make::ts_type_assertion_expression(
                    type_assertion_assignment.l_angle_token().ok()?,
                    type_assertion_assignment.ty().ok()?,
                    type_assertion_assignment.r_angle_token().ok()?,
                    type_assertion_assignment
                        .assignment()
                        .ok()?
                        .as_expression()?,
                )
                .into(),
            ),
            Self::JsBogusAssignment(_) => None,
        }
    }
}

trait BinaryExpressionExt {
    fn get_another_arg_if_one_matches(
        &self,
        cb: impl Fn(&AnyJsExpression) -> bool,
    ) -> Option<AnyJsExpression>;
}

impl BinaryExpressionExt for JsBinaryExpression {
    fn get_another_arg_if_one_matches(
        &self,
        cb: impl Fn(&AnyJsExpression) -> bool,
    ) -> Option<AnyJsExpression> {
        let left = self.left().ok()?;
        let right = self.right().ok()?;
        if cb(&left) {
            Some(right)
        } else if cb(&right) {
            Some(left)
        } else {
            None
        }
    }
}

trait UnaryExpressionExt {
    /// Returns the argument of a unary expression if it is a double unary operation.
    /// I.e. `-(-arg)` or `!!arg`.
    fn get_arg_for_double_operation(&self) -> Option<AnyJsExpression>;
}

impl UnaryExpressionExt for JsUnaryExpression {
    fn get_arg_for_double_operation(&self) -> Option<AnyJsExpression> {
        let argument = self.argument().ok()?;
        let nested_unary_expression = argument.inner_expression()?;
        let nested_unary_expression = nested_unary_expression.as_js_unary_expression()?;
        let operator = self.operator().ok()?;
        let nested_operator = nested_unary_expression.operator().ok()?;
        if operator == nested_operator {
            nested_unary_expression.argument().ok()
        } else {
            None
        }
    }
}
