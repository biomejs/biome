use crate::{JsRuleAction, services::semantic::Semantic, utils::same_reference::is_same_reference};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFormalParameter, AnyJsLiteralExpression, AnyTsType,
    JsBinaryOperator, JsCallExpression, JsConditionalExpression, JsFormalParameter,
    JsInitializerClause, JsLogicalExpression, JsLogicalOperator, JsUnaryOperator, T,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind, declare_node_union,
};
use biome_rule_options::use_math_abs::UseMathAbsOptions;

declare_lint_rule! {
    /// Prefer `Math.abs()` over manual absolute value calculations and range checks.
    ///
    /// The absolute value of a number is its distance from zero, so it is never negative.
    /// `Math.abs()` states that intent directly.
    /// Writing the calculation manually repeats the value for the positive case and for the negative case, which is longer and easier to get wrong.
    ///
    /// This rule reports two patterns:
    ///
    /// - A conditional expression that flips the sign of a value when the value is below zero, such as `value < 0 ? -value : value`.
    /// - A condition that compares the same value with a limit and with the negative of that limit, such as `value > limit || value < -limit`.
    ///
    /// `Math.abs()` throws an error when its argument is a `bigint`.
    /// The rule ignores a value when its literal, its type annotation, or a `BigInt()` call shows that it is a `bigint`.
    ///
    /// :::caution
    /// The fix can change the behavior of the code, so it is unsafe:
    ///
    /// - `Math.abs()` always returns a number, while the manual calculation can return a value of another type, such as a string, unchanged.
    /// - `Math.abs(-0)` returns `0`, while `value < 0 ? -value : value` returns `-0` when `value` is `-0`.
    /// - `Math.abs()` throws an error for a `bigint` that the rule did not recognize, such as one returned from a function call.
    /// - The fix reads the value once, while the original code reads it more than once. The difference is visible when reading a property runs code, as a getter does.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const distance = value < 0 ? -value : value;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const distance = value >= 0 ? value : 0 - value;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (offset > limit || offset < -limit) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const distance = Math.abs(value);
    /// if (Math.abs(offset) > limit) {}
    /// const size = count < 0n ? -count : count;
    /// ```
    ///
    pub UseMathAbs {
        version: "2.6.3",
        name: "useMathAbs",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-math-abs").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyUseMathAbsQuery = JsConditionalExpression | JsLogicalExpression
}

impl Rule for UseMathAbs {
    type Query = Semantic<AnyUseMathAbsQuery>;
    type State = UseMathAbsState;
    type Signals = Option<Self::State>;
    type Options = UseMathAbsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        let state = match node {
            AnyUseMathAbsQuery::JsConditionalExpression(conditional) => {
                manual_absolute_value(conditional, model)?
            }
            AnyUseMathAbsQuery::JsLogicalExpression(logical) => manual_range_check(logical, model)?,
        };

        (!is_math_shadowed(node, model)).then_some(state)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let message = match state {
            UseMathAbsState::AbsoluteValue { .. } => {
                "This conditional expression manually computes an absolute value."
            }
            UseMathAbsState::RangeCheck { .. } => {
                "This condition manually compares an absolute value with a limit."
            }
        };

        Some(
            RuleDiagnostic::new(rule_category!(), ctx.query().range(), markup! { {message} }).note(
                markup! {
                    <Emphasis>"Math.abs()"</Emphasis>" expresses this directly and avoids repeating the value for the positive and negative cases."
                },
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if node.syntax().has_inner_comments() {
            return None;
        }

        // Parentheses can separate the expression from a keyword without whitespace, as in
        // `return(value) < 0 ? -value : value`. The replacement must not produce `returnMath`.
        let first_token = node.syntax().first_token()?;
        let follows_keyword = first_token.prev_token().is_some_and(|previous| {
            previous.kind().is_keyword()
                && previous.text_trimmed_range().end() == first_token.text_trimmed_range().start()
        });
        let math = if follows_keyword {
            make::ident("Math").with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
        } else {
            make::ident("Math")
        };

        let (UseMathAbsState::AbsoluteValue { value } | UseMathAbsState::RangeCheck { value, .. }) =
            state;
        let call = AnyJsExpression::from(
            make::js_call_expression(
                make::js_static_member_expression(
                    make::js_identifier_expression(make::js_reference_identifier(math)).into(),
                    make::token(T![.]),
                    make::js_name(make::ident("abs")).into(),
                )
                .into(),
                make::js_call_arguments(
                    make::token(T!['(']),
                    make::js_call_argument_list(
                        [AnyJsCallArgument::AnyJsExpression(without_trivia(
                            value.clone().omit_parentheses(),
                        )?)],
                        [],
                    ),
                    make::token(T![')']),
                ),
            )
            .build(),
        );
        let replacement = match state {
            UseMathAbsState::AbsoluteValue { .. } => call,
            UseMathAbsState::RangeCheck {
                limit,
                is_inclusive,
                ..
            } => make::js_binary_expression(
                call,
                make::token_decorated_with_space(if *is_inclusive { T![>=] } else { T![>] }),
                // `as` and `satisfies` bind as loosely as `>`. On the right side of the
                // comparison, `limit as number` without parentheses would take the whole
                // comparison as its operand.
                match without_trivia(limit.clone())? {
                    limit @ (AnyJsExpression::TsAsExpression(_)
                    | AnyJsExpression::TsSatisfiesExpression(_)) => {
                        make::parenthesized(limit).into()
                    }
                    limit => limit,
                },
            )
            .into(),
        };

        let mut mutation = ctx.root().begin();
        mutation.replace_node_transfer_trivia(
            match node {
                AnyUseMathAbsQuery::JsConditionalExpression(conditional) => {
                    AnyJsExpression::from(conditional.clone())
                }
                AnyUseMathAbsQuery::JsLogicalExpression(logical) => {
                    AnyJsExpression::from(logical.clone())
                }
            },
            replacement,
        )?;

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"Math.abs()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug, Clone)]
pub enum UseMathAbsState {
    /// A conditional expression such as `value < 0 ? -value : value`.
    AbsoluteValue {
        /// The occurrence of the value in the condition.
        value: AnyJsExpression,
    },
    /// Two comparisons such as `value > limit || value < -limit`.
    RangeCheck {
        /// The occurrence of the value that is compared with `limit`.
        value: AnyJsExpression,
        /// The occurrence of the limit that is not negated.
        limit: AnyJsExpression,
        /// Whether the comparisons use `>=` and `<=` instead of `>` and `<`.
        is_inclusive: bool,
    },
}

/// A `<`, `<=`, `>`, or `>=` comparison whose operands are stored by the side of the operator
/// they are on, so that `a > b` and `b < a` are the same comparison.
struct Comparison {
    greater: AnyJsExpression,
    lesser: AnyJsExpression,
    /// Whether the operator is `>=` or `<=`.
    is_inclusive: bool,
}

impl Comparison {
    /// Returns `None` if `expression` is not one of the four comparisons, ignoring parentheses.
    fn from_expression(expression: AnyJsExpression) -> Option<Self> {
        let expression = expression.omit_parentheses();
        let binary = expression.as_js_binary_expression()?;
        let left = binary.left().ok()?;
        let right = binary.right().ok()?;
        let (greater, lesser, is_inclusive) = match binary.operator().ok()? {
            JsBinaryOperator::GreaterThan => (left, right, false),
            JsBinaryOperator::GreaterThanOrEqual => (left, right, true),
            JsBinaryOperator::LessThan => (right, left, false),
            JsBinaryOperator::LessThanOrEqual => (right, left, true),
            _ => return None,
        };
        Some(Self {
            greater,
            lesser,
            is_inclusive,
        })
    }
}

/// Matches a conditional expression that returns a value unchanged when it is above zero and
/// with its sign flipped when it is below zero.
///
/// All of these match:
///
/// ```js
/// value < 0 ? -value : value;
/// value >= 0 ? value : 0 - value;
/// 0 > value ? value * -1 : value;
/// ```
fn manual_absolute_value(
    conditional: &JsConditionalExpression,
    model: &SemanticModel,
) -> Option<UseMathAbsState> {
    let Comparison {
        greater, lesser, ..
    } = Comparison::from_expression(conditional.test().ok()?)?;
    let consequent = conditional.consequent().ok()?;
    let alternate = conditional.alternate().ok()?;

    let (value, unchanged, negated) = if is_zero(&lesser) {
        (greater, consequent, alternate)
    } else if is_zero(&greater) {
        (lesser, alternate, consequent)
    } else {
        return None;
    };

    if !is_same_reference(unchanged, value.clone())?
        || !is_negation_of(negated, &value)?
        || is_bigint(&value, model)
    {
        return None;
    }

    Some(UseMathAbsState::AbsoluteValue { value })
}

/// Matches two comparisons joined by `||` that check whether a value is above a limit or below
/// the negative of that limit. The comparisons can be in either order.
///
/// All of these match:
///
/// ```js
/// value > limit || value < -limit;
/// value <= -limit || value >= limit;
/// limit < value || 0 - limit > value;
/// ```
fn manual_range_check(
    logical: &JsLogicalExpression,
    model: &SemanticModel,
) -> Option<UseMathAbsState> {
    if logical.operator().ok()? != JsLogicalOperator::LogicalOr {
        return None;
    }
    let left = Comparison::from_expression(logical.left().ok()?)?;
    let right = Comparison::from_expression(logical.right().ok()?)?;

    let (above, below) = if is_mirrored_comparison(&left, &right)? {
        (left, right)
    } else if is_mirrored_comparison(&right, &left)? {
        (right, left)
    } else {
        return None;
    };

    if [&above.greater, &above.lesser, &below.greater, &below.lesser]
        .into_iter()
        .any(|operand| is_bigint(operand, model))
    {
        return None;
    }

    Some(UseMathAbsState::RangeCheck {
        value: above.greater,
        limit: above.lesser,
        is_inclusive: above.is_inclusive,
    })
}

/// Returns whether `above` is `value > limit` and `below` is `value < -limit` for the same
/// value and limit, or the same pair with `>=` and `<=`.
///
/// Returns `None` if the syntax tree is incomplete.
fn is_mirrored_comparison(above: &Comparison, below: &Comparison) -> Option<bool> {
    Some(
        above.is_inclusive == below.is_inclusive
            && is_same_reference(above.greater.clone(), below.lesser.clone())?
            && is_negation_of(below.greater.clone(), &above.lesser)?,
    )
}

/// Returns whether `expression` is `value` with its sign flipped, written as `-value`,
/// `0 - value`, `value * -1`, or `-1 * value`.
///
/// Returns `None` if the syntax tree is incomplete.
fn is_negation_of(expression: AnyJsExpression, value: &AnyJsExpression) -> Option<bool> {
    let operand = match expression.omit_parentheses() {
        AnyJsExpression::JsUnaryExpression(unary)
            if unary.operator().ok()? == JsUnaryOperator::Minus =>
        {
            unary.argument().ok()?
        }
        AnyJsExpression::JsBinaryExpression(binary) => {
            let left = binary.left().ok()?;
            let right = binary.right().ok()?;
            match binary.operator().ok()? {
                JsBinaryOperator::Minus if is_zero(&left) => right,
                JsBinaryOperator::Times if is_negative_one(&right) => left,
                JsBinaryOperator::Times if is_negative_one(&left) => right,
                _ => return Some(false),
            }
        }
        _ => return Some(false),
    };
    is_same_reference(operand, value.clone())
}

/// Returns the value of `expression` if it is a number literal, ignoring parentheses.
fn number_literal_value(expression: &AnyJsExpression) -> Option<f64> {
    match expression.clone().omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(number),
        ) => number.as_number(),
        _ => None,
    }
}

fn is_zero(expression: &AnyJsExpression) -> bool {
    number_literal_value(expression) == Some(0.0)
}

/// Returns whether `expression` is the number `-1`, ignoring parentheses. The bigint `-1n`
/// doesn't match.
fn is_negative_one(expression: &AnyJsExpression) -> bool {
    let AnyJsExpression::JsUnaryExpression(unary) = expression.clone().omit_parentheses() else {
        return false;
    };
    matches!(unary.operator(), Ok(JsUnaryOperator::Minus))
        && unary
            .argument()
            .is_ok_and(|argument| number_literal_value(&argument) == Some(1.0))
}

/// Returns whether the code shows that `expression` is a `bigint`, either in the expression
/// itself or in the declaration of the variable or parameter that it reads.
///
/// This doesn't use type inference, so a `bigint` that is only known from the return type of a
/// function, for example, is not detected.
fn is_bigint(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    is_bigint_expression(expression) || has_bigint_declaration(expression, model).unwrap_or(false)
}

/// Returns whether `expression` is a bigint literal, a `BigInt()` call, or a TypeScript type
/// assertion to a type that includes `bigint`. A minus sign in front and other type assertions
/// around one of those are ignored, so `-1n` and `1n as const` also match.
fn is_bigint_expression(expression: &AnyJsExpression) -> bool {
    let mut expression = expression.clone();
    loop {
        let (inner, ty) = match expression.omit_parentheses() {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsBigintLiteralExpression(_),
            ) => return true,
            AnyJsExpression::JsCallExpression(call) => return is_bigint_call(&call),
            AnyJsExpression::JsUnaryExpression(unary)
                if matches!(unary.operator(), Ok(JsUnaryOperator::Minus)) =>
            {
                (unary.argument(), None)
            }
            AnyJsExpression::TsNonNullAssertionExpression(assertion) => {
                (assertion.expression(), None)
            }
            AnyJsExpression::TsAsExpression(assertion) => {
                (assertion.expression(), assertion.ty().ok())
            }
            AnyJsExpression::TsSatisfiesExpression(assertion) => {
                (assertion.expression(), assertion.ty().ok())
            }
            AnyJsExpression::TsTypeAssertionExpression(assertion) => {
                (assertion.expression(), assertion.ty().ok())
            }
            _ => return false,
        };
        if ty.is_some_and(is_bigint_type) {
            return true;
        }
        let Ok(inner) = inner else {
            return false;
        };
        expression = inner;
    }
}

/// Returns whether `call` is `BigInt(value)`.
fn is_bigint_call(call: &JsCallExpression) -> bool {
    !call.is_optional()
        && call
            .arguments()
            .is_ok_and(|arguments| arguments.args().len() == 1)
        && call.callee().is_ok_and(|callee| {
            callee
                .omit_parentheses()
                .as_js_identifier_expression()
                .and_then(|identifier| identifier.name().ok())
                .is_some_and(|name| name.has_name("BigInt"))
        })
}

/// Returns whether `ty` is `bigint`, a bigint literal type such as `1n`, or a union or
/// intersection that contains one of them.
fn is_bigint_type(ty: AnyTsType) -> bool {
    let mut types = vec![ty];
    while let Some(ty) = types.pop() {
        match ty {
            AnyTsType::TsBigintType(_) | AnyTsType::TsBigintLiteralType(_) => return true,
            AnyTsType::TsParenthesizedType(parenthesized) => types.extend(parenthesized.ty().ok()),
            AnyTsType::TsUnionType(union) => types.extend(union.types().iter().flatten()),
            AnyTsType::TsIntersectionType(intersection) => {
                types.extend(intersection.types().iter().flatten());
            }
            _ => {}
        }
    }
    false
}

/// Returns whether `expression` reads a variable or parameter whose type annotation, initial
/// value, or default value is a `bigint`. TypeScript type assertions around the name are ignored.
///
/// Returns `None` if `expression` is not a name that is declared in the file.
fn has_bigint_declaration(expression: &AnyJsExpression, model: &SemanticModel) -> Option<bool> {
    let expression = expression.clone().omit_type_wrappers()?;
    let reference = expression.as_js_identifier_expression()?.name().ok()?;
    let declaration = model.binding(&reference)?.tree().declaration()?;

    let (ty, initializer) = match declaration {
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => parameter_metadata(&parameter),
        AnyJsBindingDeclaration::TsPropertyParameter(parameter) => {
            match parameter.formal_parameter().ok()? {
                AnyJsFormalParameter::JsFormalParameter(parameter) => {
                    parameter_metadata(&parameter)
                }
                _ => return Some(false),
            }
        }
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => (
            declarator
                .variable_annotation()
                .and_then(|annotation| annotation.type_annotation().ok()??.ty().ok()),
            declarator.initializer(),
        ),
        AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(property) => {
            (None, property.init())
        }
        AnyJsBindingDeclaration::JsObjectBindingPatternProperty(property) => {
            (None, property.init())
        }
        AnyJsBindingDeclaration::JsArrayBindingPatternElement(element) => (None, element.init()),
        _ => return Some(false),
    };

    Some(
        ty.is_some_and(is_bigint_type)
            || initializer
                .and_then(|initializer| initializer.expression().ok())
                .is_some_and(|expression| is_bigint_expression(&expression)),
    )
}

/// Returns the type annotation and the default value of `parameter`.
fn parameter_metadata(
    parameter: &JsFormalParameter,
) -> (Option<AnyTsType>, Option<JsInitializerClause>) {
    (
        parameter
            .type_annotation()
            .and_then(|annotation| annotation.ty().ok()),
        parameter.initializer(),
    )
}

/// Returns whether a declaration named `Math` is visible from `node`, which means that `Math`
/// doesn't refer to the built-in object there.
fn is_math_shadowed(node: &AnyUseMathAbsQuery, model: &SemanticModel) -> bool {
    model
        .scope(node.syntax())
        .ancestors()
        .any(|scope| scope.get_binding("Math").is_some())
}

/// Removes all leading and trailing trivia from `expression`, including comments.
fn without_trivia(expression: AnyJsExpression) -> Option<AnyJsExpression> {
    expression
        .with_leading_trivia_pieces([])?
        .with_trailing_trivia_pieces([])
}
