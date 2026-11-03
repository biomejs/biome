use crate::{JsRuleAction, services::semantic::Semantic};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsAssignment, AnyJsCallArgument, AnyJsExpression, AnyJsTemplateElement,
    JsAssignmentExpression, JsAssignmentOperator, JsBinaryExpression, JsBinaryOperator,
    JsCallArguments, JsCallExpression, JsLanguage, JsSyntaxKind, JsUnaryExpression,
    JsUnaryOperator, T, global_identifier,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutation, BatchMutationExt, Direction, TextRange,
    TriviaPieceKind, declare_node_union,
};
use biome_rule_options::use_math_trunc::UseMathTruncOptions;

declare_lint_rule! {
    /// Require `Math.trunc()` instead of tricks that remove the decimal part of a number.
    ///
    /// Some code removes the decimal part of a number with a bitwise operation, such as `x | 0` or `~~x`,
    /// or by turning the number into a string and parsing it back, as in `parseInt(String(x), 10)`.
    /// These patterns are hard to read, and they give wrong results for some numbers:
    ///
    /// - Bitwise operations turn the number into a 32-bit integer first.
    ///   Numbers above 2147483647 or below -2147483648 wrap around, so `~~(2 ** 32)` is `0`.
    ///   They also turn `NaN` into `0`.
    /// - Very large and very small numbers turn into strings such as `"1e+21"`,
    ///   which `parseInt()` reads as `1`.
    ///
    /// [`Math.trunc()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/trunc)
    /// removes the decimal part of any number and states the intent clearly.
    ///
    /// This rule reports these patterns, where the bitwise operations use the number `0`:
    ///
    /// - `x | 0`, `x ^ 0`, `x << 0`, and `x >> 0`
    /// - `x |= 0`, `x ^= 0`, `x <<= 0`, and `x >>= 0`
    /// - `~~x`
    /// - `parseInt(String(x), 10)` and `Number.parseInt(String(x), 10)`
    ///
    /// The fix is unsafe because the result can differ from the original code.
    /// `Math.trunc()` keeps numbers outside the 32-bit range and keeps `NaN`, unlike bitwise operations.
    /// It also handles values that aren't numbers differently from `parseInt()`:
    /// `parseInt(String("12px"), 10)` is `12`, while `Math.trunc("12px")` is `NaN`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const result = value | 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const result = ~~value;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const result = value >> 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// value |= 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const result = parseInt(String(value), 10);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const truncated = Math.trunc(value);
    /// const flags = value | 1;
    /// const inverted = ~value;
    /// const parsed = parseInt(input, 10);
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow bitwise operators entirely, see [`noBitwiseOperators`](https://biomejs.dev/linter/rules/no-bitwise-operators/).
    ///
    pub UseMathTrunc {
        version: "2.6.3",
        name: "useMathTrunc",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-math-trunc").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyJsTruncationCandidate =
        JsBinaryExpression
        | JsAssignmentExpression
        | JsUnaryExpression
        | JsCallExpression
}

impl Rule for UseMathTrunc {
    type Query = Semantic<AnyJsTruncationCandidate>;
    type State = TruncationPattern;
    type Signals = Option<Self::State>;
    type Options = UseMathTruncOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyJsTruncationCandidate::JsBinaryExpression(binary) => {
                let operator = binary.operator().ok()?;
                (TRUNCATING_BINARY_OPERATORS.contains(&operator) && is_zero(&binary.right().ok()?))
                    .then_some(TruncationPattern::BitwiseOperator)
            }
            AnyJsTruncationCandidate::JsAssignmentExpression(assignment) => {
                let operator = assignment.operator().ok()?;
                (TRUNCATING_ASSIGNMENT_OPERATORS.contains(&operator)
                    && is_zero(&assignment.right().ok()?))
                .then_some(TruncationPattern::BitwiseOperator)
            }
            AnyJsTruncationCandidate::JsUnaryExpression(unary) => {
                double_bitwise_not_operand(unary).map(TruncationPattern::DoubleBitwiseNot)
            }
            AnyJsTruncationCandidate::JsCallExpression(call) => {
                let function = parse_int_function(call, ctx.model())?;
                let arguments = string_call_arguments(call, ctx.model())?;
                Some(TruncationPattern::ParseInt(function, arguments))
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let diagnostic = match state {
            TruncationPattern::BitwiseOperator => {
                let (operator, right) = match node {
                    AnyJsTruncationCandidate::JsBinaryExpression(binary) => {
                        (binary.operator_token().ok()?, binary.right().ok()?)
                    }
                    AnyJsTruncationCandidate::JsAssignmentExpression(assignment) => {
                        (assignment.operator_token().ok()?, assignment.right().ok()?)
                    }
                    _ => return None,
                };
                let zero = right
                    .omit_parentheses()
                    .as_any_js_literal_expression()?
                    .as_js_number_literal_expression()?
                    .value_token()
                    .ok()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "Use "<Emphasis>"Math.trunc()"</Emphasis>" instead of "<Emphasis>{operator.text_trimmed()}" "{zero.text_trimmed()}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "Bitwise operations turn the number into a 32-bit integer first, so numbers above 2147483647 or below -2147483648 give wrong results."
                })
            }
            TruncationPattern::DoubleBitwiseNot(_) => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use "<Emphasis>"Math.trunc()"</Emphasis>" instead of "<Emphasis>"~~"</Emphasis>"."
                },
            )
            .note(markup! {
                "Bitwise operations turn the number into a 32-bit integer first, so numbers above 2147483647 or below -2147483648 give wrong results."
            }),
            TruncationPattern::ParseInt(function, _) => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use "<Emphasis>"Math.trunc()"</Emphasis>" instead of "<Emphasis>{function}"(String(...), 10)"</Emphasis>"."
                },
            )
            .note(markup! {
                "Very large and very small numbers turn into strings such as "<Emphasis>"\"1e+21\""</Emphasis>", which "<Emphasis>{function}"()"</Emphasis>" reads as "<Emphasis>"1"</Emphasis>"."
            }),
        };

        Some(diagnostic.note(markup! {
            <Emphasis>"Math.trunc()"</Emphasis>" removes the decimal part of any number and states the intent clearly."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match (node, state) {
            (AnyJsTruncationCandidate::JsBinaryExpression(binary), _) => {
                if binary.syntax().has_inner_comments() {
                    return None;
                }
                replace_with_math_trunc(
                    &mut mutation,
                    binary.clone().into(),
                    single_argument_list(single_argument(binary.left().ok()?)?),
                )?;
            }
            (
                AnyJsTruncationCandidate::JsUnaryExpression(unary),
                TruncationPattern::DoubleBitwiseNot(operand),
            ) => {
                if unary.syntax().has_inner_comments() {
                    return None;
                }
                replace_with_math_trunc(
                    &mut mutation,
                    unary.clone().into(),
                    single_argument_list(single_argument(operand.clone())?),
                )?;
            }
            (AnyJsTruncationCandidate::JsAssignmentExpression(assignment), _) => {
                if assignment.syntax().has_inner_comments() {
                    return None;
                }
                let target = assignment
                    .left()
                    .ok()?
                    .as_any_js_assignment()
                    .and_then(side_effect_free_target)?;
                mutation.replace_token(assignment.operator_token().ok()?, make::token(T![=]));
                mutation.replace_node(
                    assignment.right().ok()?,
                    math_trunc_call(single_argument_list(single_argument(target)?), false),
                );
            }
            (
                AnyJsTruncationCandidate::JsCallExpression(call),
                TruncationPattern::ParseInt(_, arguments),
            ) => {
                if has_comments_outside_arguments(call, arguments) {
                    return None;
                }
                let l_paren = arguments.l_paren_token().ok()?;
                let r_paren = arguments.r_paren_token().ok()?;
                let arguments = arguments
                    .clone()
                    .with_l_paren_token(l_paren.with_leading_trivia([]))
                    .with_r_paren_token(r_paren.with_trailing_trivia([]));
                replace_with_math_trunc(&mut mutation, call.clone().into(), arguments)?;
            }
            _ => return None,
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use "<Emphasis>"Math.trunc()"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug, Clone)]
pub enum TruncationPattern {
    /// `x | 0`, `x |= 0`, and the same forms with `^`, `<<`, and `>>`.
    BitwiseOperator,
    /// `~~x`, storing the truncated value `x`.
    DoubleBitwiseNot(AnyJsExpression),
    /// `parseInt(String(x), 10)` or `Number.parseInt(String(x), 10)`, storing the
    /// arguments of the `String` call.
    ParseInt(ParseIntFunction, JsCallArguments),
}

#[derive(Debug, Clone, Copy)]
pub enum ParseIntFunction {
    /// The global `parseInt`.
    Global,
    /// `Number.parseInt`.
    NumberMethod,
}

impl biome_console::fmt::Display for ParseIntFunction {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Global => "parseInt",
            Self::NumberMethod => "Number.parseInt",
        })
    }
}

const TRUNCATING_BINARY_OPERATORS: &[JsBinaryOperator] = &[
    JsBinaryOperator::BitwiseOr,
    JsBinaryOperator::BitwiseXor,
    JsBinaryOperator::LeftShift,
    JsBinaryOperator::RightShift,
];

const TRUNCATING_ASSIGNMENT_OPERATORS: &[JsAssignmentOperator] = &[
    JsAssignmentOperator::BitwiseOrAssign,
    JsAssignmentOperator::BitwiseXorAssign,
    JsAssignmentOperator::LeftShiftAssign,
    JsAssignmentOperator::RightShiftAssign,
];

/// Returns whether `expression` is a number literal equal to zero, such as `0`, `0.0`, or `0x0`.
fn is_zero(expression: &AnyJsExpression) -> bool {
    expression
        .clone()
        .omit_parentheses()
        .as_any_js_literal_expression()
        .and_then(|literal| literal.as_js_number_literal_expression()?.as_number())
        == Some(0.0)
}

/// Returns the value truncated by the innermost `~~` pair, such as `x` in `~~x`.
///
/// In `~~~x`, only the inner `~~x` is a match, so the outer `~` returns `None`.
fn double_bitwise_not_operand(unary: &JsUnaryExpression) -> Option<AnyJsExpression> {
    if unary.operator().ok()? != JsUnaryOperator::BitwiseNot {
        return None;
    }
    let inner = unary.argument().ok()?.omit_parentheses();
    let inner = inner.as_js_unary_expression()?;
    if inner.operator().ok()? != JsUnaryOperator::BitwiseNot {
        return None;
    }
    let operand = inner.argument().ok()?;
    let is_bitwise_not = operand
        .clone()
        .omit_parentheses()
        .as_js_unary_expression()
        .is_some_and(|unary| unary.operator() == Ok(JsUnaryOperator::BitwiseNot));
    (!is_bitwise_not).then_some(operand)
}

/// Returns which `parseInt` function `call` invokes, when its callee is the global
/// `parseInt` or the global `Number.parseInt` and the call is not optional.
fn parse_int_function(call: &JsCallExpression, model: &SemanticModel) -> Option<ParseIntFunction> {
    if call.optional_chain_token().is_some() {
        return None;
    }
    let callee = call.callee().ok()?.omit_parentheses();
    if is_global(&callee, "parseInt", model) {
        return Some(ParseIntFunction::Global);
    }
    let member = callee.as_js_static_member_expression()?;
    (member.operator_token().ok()?.kind() == T![.]
        && member.member().ok()?.value_token().ok()?.text_trimmed() == "parseInt"
        && is_global(&member.object().ok()?.omit_parentheses(), "Number", model))
    .then_some(ParseIntFunction::NumberMethod)
}

/// Returns the arguments of `String(...)` in `parseInt(String(...), 10)`.
///
/// Returns `None` unless `call` has exactly two arguments, the first is a call to the
/// global `String` with exactly one argument, the second is the number `10`, and none
/// of them is spread.
fn string_call_arguments(
    call: &JsCallExpression,
    model: &SemanticModel,
) -> Option<JsCallArguments> {
    let [
        Some(AnyJsCallArgument::AnyJsExpression(value)),
        Some(AnyJsCallArgument::AnyJsExpression(radix)),
        None,
    ] = call.arguments().ok()?.get_arguments_by_index([0, 1, 2])
    else {
        return None;
    };
    let radix = radix.omit_parentheses();
    if radix
        .as_any_js_literal_expression()
        .and_then(|literal| literal.as_js_number_literal_expression()?.as_number())
        != Some(10.0)
    {
        return None;
    }

    let value = value.omit_parentheses();
    let string_call = value.as_js_call_expression()?;
    if string_call.optional_chain_token().is_some()
        || !is_global(
            &string_call.callee().ok()?.omit_parentheses(),
            "String",
            model,
        )
    {
        return None;
    }
    let arguments = string_call.arguments().ok()?;
    let [Some(AnyJsCallArgument::AnyJsExpression(_)), None] =
        arguments.get_arguments_by_index([0, 1])
    else {
        return None;
    };
    Some(arguments)
}

/// Returns whether `expression` refers to the global variable `name` and isn't shadowed
/// by a local declaration.
fn is_global(expression: &AnyJsExpression, name: &str, model: &SemanticModel) -> bool {
    expression
        .as_any_global_identifier_expression()
        .and_then(|expression| global_identifier(&expression))
        .is_some_and(|(reference, global_name)| {
            global_name.text() == name && model.binding(&reference).is_none()
        })
}

/// Returns `target` as an expression when reading it has no side effects, such as
/// `foo`, `foo.bar`, or `foo[0]`. Returns `None` for targets such as `foo[i++]` or
/// `foo().bar`, which a fix can't repeat without running the side effect twice.
fn side_effect_free_target(target: &AnyJsAssignment) -> Option<AnyJsExpression> {
    match target {
        AnyJsAssignment::JsParenthesizedAssignment(parenthesized) => {
            side_effect_free_target(&parenthesized.assignment().ok()?)
        }
        AnyJsAssignment::TsNonNullAssertionAssignment(assertion) => Some(
            make::ts_non_null_assertion_expression(
                side_effect_free_target(&assertion.assignment().ok()?)?,
                assertion.excl_token().ok()?,
            )
            .into(),
        ),
        AnyJsAssignment::TsAsAssignment(assertion) => Some(
            make::ts_as_expression(
                side_effect_free_target(&assertion.assignment().ok()?)?,
                assertion.as_token().ok()?,
                assertion.ty().ok()?,
            )
            .into(),
        ),
        AnyJsAssignment::TsSatisfiesAssignment(assertion) => Some(
            make::ts_satisfies_expression(
                side_effect_free_target(&assertion.assignment().ok()?)?,
                assertion.satisfies_token().ok()?,
                assertion.ty().ok()?,
            )
            .into(),
        ),
        AnyJsAssignment::TsTypeAssertionAssignment(assertion) => Some(
            make::ts_type_assertion_expression(
                assertion.l_angle_token().ok()?,
                assertion.ty().ok()?,
                assertion.r_angle_token().ok()?,
                side_effect_free_target(&assertion.assignment().ok()?)?,
            )
            .into(),
        ),
        AnyJsAssignment::JsIdentifierAssignment(identifier) => Some(
            make::js_identifier_expression(make::js_reference_identifier(
                identifier.name_token().ok()?,
            ))
            .into(),
        ),
        AnyJsAssignment::JsStaticMemberAssignment(member) => {
            let object = member.object().ok()?;
            is_side_effect_free(&object).then_some(())?;
            Some(
                make::js_static_member_expression(
                    object,
                    member.dot_token().ok()?,
                    member.member().ok()?,
                )
                .into(),
            )
        }
        AnyJsAssignment::JsComputedMemberAssignment(member) => {
            let object = member.object().ok()?;
            let property = member.member().ok()?;
            (is_side_effect_free(&object) && is_side_effect_free(&property)).then_some(())?;
            Some(
                make::js_computed_member_expression(
                    object,
                    member.l_brack_token().ok()?,
                    property,
                    member.r_brack_token().ok()?,
                )
                .build()
                .into(),
            )
        }
        _ => None,
    }
}

/// Returns whether evaluating `expression` can't change program state.
///
/// Accepts variables, `this`, `super`, literals, property reads, operators other than
/// `delete`, conditionals, and arrays and untagged template literals built from such
/// expressions. Parentheses and TypeScript type assertions are looked through. Calls,
/// assignments, `++`, and `--` are rejected. Getters and conversions to primitive
/// values, such as `valueOf()` calls triggered by `+`, are assumed to have no side effects.
fn is_side_effect_free(expression: &AnyJsExpression) -> bool {
    let Some(expression) = expression.clone().omit_type_wrappers() else {
        return false;
    };
    match expression {
        AnyJsExpression::JsIdentifierExpression(_)
        | AnyJsExpression::JsThisExpression(_)
        | AnyJsExpression::JsSuperExpression(_)
        | AnyJsExpression::AnyJsLiteralExpression(_) => true,
        AnyJsExpression::JsBinaryExpression(binary) => {
            binary.left().is_ok_and(|left| is_side_effect_free(&left))
                && binary
                    .right()
                    .is_ok_and(|right| is_side_effect_free(&right))
        }
        AnyJsExpression::JsLogicalExpression(logical) => {
            logical.left().is_ok_and(|left| is_side_effect_free(&left))
                && logical
                    .right()
                    .is_ok_and(|right| is_side_effect_free(&right))
        }
        AnyJsExpression::JsUnaryExpression(unary) => {
            unary
                .operator()
                .is_ok_and(|operator| operator != JsUnaryOperator::Delete)
                && unary
                    .argument()
                    .is_ok_and(|argument| is_side_effect_free(&argument))
        }
        AnyJsExpression::JsConditionalExpression(conditional) => {
            conditional
                .test()
                .is_ok_and(|test| is_side_effect_free(&test))
                && conditional
                    .consequent()
                    .is_ok_and(|consequent| is_side_effect_free(&consequent))
                && conditional
                    .alternate()
                    .is_ok_and(|alternate| is_side_effect_free(&alternate))
        }
        AnyJsExpression::JsArrayExpression(array) => {
            array.elements().iter().all(|element| match element {
                Ok(AnyJsArrayElement::AnyJsExpression(element)) => is_side_effect_free(&element),
                Ok(AnyJsArrayElement::JsArrayHole(_)) => true,
                _ => false,
            })
        }
        AnyJsExpression::JsTemplateExpression(template) => {
            template.tag().is_none()
                && template
                    .elements()
                    .into_iter()
                    .all(|element| match element {
                        AnyJsTemplateElement::JsTemplateChunkElement(_) => true,
                        AnyJsTemplateElement::JsTemplateElement(element) => element
                            .expression()
                            .is_ok_and(|expression| is_side_effect_free(&expression)),
                    })
        }
        AnyJsExpression::JsStaticMemberExpression(member) => member
            .object()
            .is_ok_and(|object| is_side_effect_free(&object)),
        AnyJsExpression::JsComputedMemberExpression(member) => {
            member
                .object()
                .is_ok_and(|object| is_side_effect_free(&object))
                && member
                    .member()
                    .is_ok_and(|property| is_side_effect_free(&property))
        }
        _ => false,
    }
}

/// Returns `expression` in a form that can be the only argument of a call.
///
/// Parentheses are removed, except around a comma-separated sequence such as
/// `(a, b)`, where they keep the sequence a single argument. Whitespace and comments
/// before and after the expression are removed.
fn single_argument(expression: AnyJsExpression) -> Option<AnyJsExpression> {
    let expression = expression.omit_parentheses().trim_trivia()?;
    Some(if expression.as_js_sequence_expression().is_some() {
        make::js_parenthesized_expression(make::token(T!['(']), expression, make::token(T![')']))
            .into()
    } else {
        expression
    })
}

fn single_argument_list(argument: AnyJsExpression) -> JsCallArguments {
    make::js_call_arguments(
        make::token(T!['(']),
        make::js_call_argument_list([AnyJsCallArgument::AnyJsExpression(argument)], []),
        make::token(T![')']),
    )
}

/// Returns `Math.trunc` called with `arguments`, starting with a space when
/// `leading_space` is `true`.
fn math_trunc_call(arguments: JsCallArguments, leading_space: bool) -> AnyJsExpression {
    let math = make::ident("Math");
    let math = if leading_space {
        math.with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
    } else {
        math
    };
    make::js_call_expression(
        make::js_static_member_expression(
            make::js_identifier_expression(make::js_reference_identifier(math)).into(),
            make::token(T![.]),
            make::js_name(make::ident("trunc")).into(),
        )
        .into(),
        arguments,
    )
    .build()
    .into()
}

/// Replaces `expression` with a call to `Math.trunc` with `arguments`, keeping the
/// whitespace and comments around `expression`.
///
/// A space is inserted when `expression` touches a preceding keyword or name, as in
/// `return~~x`, so the replacement doesn't produce `returnMath.trunc(x)`.
fn replace_with_math_trunc(
    mutation: &mut BatchMutation<JsLanguage>,
    expression: AnyJsExpression,
    arguments: JsCallArguments,
) -> Option<()> {
    let first_token = expression.syntax().first_token()?;
    let leading_space = first_token.leading_trivia().pieces().next().is_none()
        && first_token.prev_token().is_some_and(|token| {
            (token.kind().is_keyword() || token.kind() == JsSyntaxKind::IDENT)
                && token.trailing_trivia().pieces().next().is_none()
        });
    mutation.replace_node_transfer_trivia(expression, math_trunc_call(arguments, leading_space))
}

/// Returns whether `call` has a comment that the fix would delete, meaning a comment
/// anywhere in `call` except inside the parentheses of `arguments`, before `call`, or
/// after `call`.
fn has_comments_outside_arguments(call: &JsCallExpression, arguments: &JsCallArguments) -> bool {
    let (Ok(l_paren), Ok(r_paren)) = (arguments.l_paren_token(), arguments.r_paren_token()) else {
        return true;
    };
    let kept_range = TextRange::new(
        l_paren.text_trimmed_range().end(),
        r_paren.text_trimmed_range().start(),
    );
    let (Some(first), Some(last)) = (call.syntax().first_token(), call.syntax().last_token())
    else {
        return true;
    };

    call.syntax()
        .descendants_tokens(Direction::Next)
        .flat_map(|token| {
            let leading = (token != first).then(|| token.leading_trivia().pieces());
            let trailing = (token != last).then(|| token.trailing_trivia().pieces());
            leading
                .into_iter()
                .flatten()
                .chain(trailing.into_iter().flatten())
        })
        .any(|piece| piece.is_comments() && !kept_range.contains_range(piece.text_range()))
}
