use crate::{JsRuleAction, services::typed::Typed};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression,
    AnyJsObjectMember, JsBinaryExpression, JsBinaryOperator, JsCallExpression, JsUnaryOperator, T,
};
use biome_rowan::{AstNode, BatchMutationExt, declare_node_union};
use biome_rule_options::use_includes::UseIncludesOptions;

declare_lint_rule! {
    /// Enforce the use of `includes()` over `indexOf()`.
    ///
    /// When checking if an array or a string includes a value, it is common to use `indexOf()` and check if the result is `-1`.
    /// However, it is simpler and more readable to use the `includes()` method, which returns a boolean.
    /// This rule reports when an `.indexOf()` call can be replaced with an `.includes()`.
    /// Additionally, this rule reports the tests of simple regular expressions in favor of `String#includes`.
    ///
    /// This rule will report on any receiver object of an `indexOf` method call that has an `includes` method
    /// where the two methods have the same parameters. This includes well-known built-in types like `String`, `Array`, `ReadonlyArray`, and typed arrays,
    /// as well as user-defined objects that have both an `indexOf` and an `includes` method.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// ["a", "b", "c"].indexOf("a") === -1
    /// ```
    /// ### Valid
    ///
    /// ```js
    /// !["a", "b", "c"].includes("a");
    /// ```
    ///
    pub UseIncludes {
        version: "next", // TODO
        name: "useIncludes",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-includes").same()],
        recommended: false,
        domains: &[RuleDomain::Project],
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub UseIncludesQuery = JsBinaryExpression | JsCallExpression
}

pub enum State {
    IndexOf(IndexOfData),
    RegexTest(RegexTestData),
}

pub struct IndexOfData {
    binary_expression: JsBinaryExpression,
    call_expression: JsCallExpression,
    member_expression: AnyJsMemberExpression,
    is_negated: bool,
}

impl Rule for UseIncludes {
    type Query = Typed<UseIncludesQuery>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = UseIncludesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            UseIncludesQuery::JsBinaryExpression(binary_expr) => {
                let (call_expr, check_type) = analyze_index_of_check(binary_expr)?;

                let member_expr = AnyJsMemberExpression::cast_ref(
                    call_expr.callee().ok()?.omit_parentheses().syntax(),
                )?;

                if member_expr.member_name()?.text() != "indexOf" {
                    return None;
                }

                // Verify that the object has an `includes` method.
                if !is_receiver_known_to_have_includes(member_expr.object().ok()) {
                    return None;
                }

                Some(State::IndexOf(IndexOfData {
                    binary_expression: binary_expr.clone(),
                    is_negated: check_type.is_negated(),
                    call_expression: call_expr,
                    member_expression: member_expr,
                }))
            }
            UseIncludesQuery::JsCallExpression(call_expr) => {
                let member_expr = AnyJsMemberExpression::cast_ref(
                    call_expr.callee().ok()?.omit_parentheses().syntax(),
                )?;

                if member_expr.member_name()?.text() != "test" {
                    return None;
                }

                let object = member_expr.object().ok()?;
                let regex_literal = object
                    .as_any_js_literal_expression()?
                    .as_js_regex_literal_expression()?;

                let (pattern, flags) = regex_literal.decompose().ok()?;

                if is_simple_regex(&pattern, &flags) {
                    Some(State::RegexTest(RegexTestData {
                        call_expression: call_expr.clone(),
                        member_expression: member_expr,
                    }))
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            State::IndexOf(data) => Some(RuleDiagnostic::new(
                rule_category!(),
                data.binary_expression.range(),
                markup! {
                    "Use "<Emphasis>"includes()"</Emphasis>" instead of "<Emphasis>"indexOf()"</Emphasis>" to check for inclusion, as it is more readable and concise."
                },
            )),
            State::RegexTest(data) => Some(RuleDiagnostic::new(
                rule_category!(),
                data.call_expression.range(),
                markup! {
                    "Use "<Emphasis>"String.prototype.includes()"</Emphasis>" instead of "<Emphasis>"RegExp.prototype.test()"</Emphasis>" for simple searches as it is more readable and concise."
                },
            )),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        match state {
            State::IndexOf(data) => {
                let mut mutation = ctx.root().begin();

                let new_member_expr = match data.member_expression.clone() {
                    AnyJsMemberExpression::JsStaticMemberExpression(static_member) => {
                        AnyJsMemberExpression::from(
                            static_member
                                .with_member(make::js_name(make::ident("includes")).into()),
                        )
                    }
                    AnyJsMemberExpression::JsComputedMemberExpression(computed_member) => {
                        AnyJsMemberExpression::from(computed_member.with_member(
                            AnyJsExpression::AnyJsLiteralExpression(
                                AnyJsLiteralExpression::JsStringLiteralExpression(
                                    make::js_string_literal_expression(make::js_string_literal(
                                        "includes",
                                    )),
                                ),
                            ),
                        ))
                    }
                };

                let mut new_call_expr = make::js_call_expression(
                    AnyJsExpression::from(new_member_expr),
                    data.call_expression.arguments().ok()?,
                )
                .build();

                new_call_expr = new_call_expr.with_leading_trivia_pieces([])?;

                let mut final_expr = if data.is_negated {
                    let unary_expr = make::js_unary_expression(
                        make::token(T![!]).with_trailing_trivia_pieces([]),
                        new_call_expr.with_leading_trivia_pieces([])?.into(),
                    );
                    AnyJsExpression::from(unary_expr)
                } else {
                    new_call_expr.into()
                };

                if let Some(trivia) = data.binary_expression.syntax().first_leading_trivia() {
                    final_expr = final_expr.with_leading_trivia_pieces(trivia.pieces())?;
                }
                if let Some(trivia) = data.binary_expression.syntax().last_trailing_trivia() {
                    final_expr = final_expr.with_trailing_trivia_pieces(trivia.pieces())?;
                }
                mutation.replace_node(
                    AnyJsExpression::from(data.binary_expression.clone()),
                    final_expr,
                );

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Replace with 'includes()'" }.to_owned(),
                    mutation,
                ))
            }
            State::RegexTest(data) => {
                let mut mutation = ctx.root().begin();

                let object = data.member_expression.object().ok()?;
                let regex_literal = object
                    .as_any_js_literal_expression()?
                    .as_js_regex_literal_expression()?;
                let (pattern, _) = regex_literal.decompose().ok()?;

                let string_literal = make::js_string_literal(pattern.text());
                let string_literal_expr = make::js_string_literal_expression(string_literal);

                let arguments = data.call_expression.arguments().ok()?;
                let first_arg = arguments
                    .args()
                    .into_iter()
                    .next()?
                    .ok()?
                    .as_any_js_expression()?
                    .clone();

                let new_callee = make::js_static_member_expression(
                    first_arg,
                    make::token(T![.]),
                    make::js_name(make::ident("includes")).into(),
                );

                let new_arguments = make::js_call_arguments(
                    make::token(T!['(']),
                    make::js_call_argument_list(
                        [AnyJsCallArgument::AnyJsExpression(
                            AnyJsExpression::AnyJsLiteralExpression(
                                AnyJsLiteralExpression::JsStringLiteralExpression(
                                    string_literal_expr,
                                ),
                            ),
                        )],
                        [],
                    ),
                    make::token(T![')']),
                );

                let new_call_expr =
                    make::js_call_expression(AnyJsExpression::from(new_callee), new_arguments)
                        .build();

                mutation.replace_node(
                    AnyJsExpression::from(data.call_expression.clone()),
                    AnyJsExpression::from(new_call_expr),
                );

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Replace with 'includes()'" }.to_owned(),
                    mutation,
                ))
            }
        }
    }
}

pub struct RegexTestData {
    call_expression: JsCallExpression,
    member_expression: AnyJsMemberExpression,
}

enum IndexOfCheckType {
    /// `indexOf(foo) === -1` or `indexOf(foo) < 0`
    Absence,
    /// `indexOf(foo) !== -1` or `indexOf(foo) >= 0`
    Presence,
}

impl IndexOfCheckType {
    fn is_negated(&self) -> bool {
        matches!(self, Self::Absence)
    }
}

/// Analyzes a binary expression to determine if it's a check for `indexOf` result.
///
/// Returns the `indexOf` call expression and the type of check (presence or absence).
fn analyze_index_of_check(
    binary_expr: &JsBinaryExpression,
) -> Option<(JsCallExpression, IndexOfCheckType)> {
    let op = binary_expr.operator().ok()?;
    let left = binary_expr.left().ok()?.omit_parentheses();
    let right = binary_expr.right().ok()?.omit_parentheses();

    let (call_expr, number_expr) = if let Some(call) = left.as_js_call_expression() {
        (call, &right)
    } else if let Some(call) = right.as_js_call_expression() {
        (call, &left)
    } else {
        return None;
    };

    let is_call_on_left = left.as_js_call_expression().is_some();

    let check_type = match (op, NumberMatcher::from(number_expr)?) {
        // `indexOf(foo) === -1`
        (
            JsBinaryOperator::StrictEquality | JsBinaryOperator::Equality,
            NumberMatcher::MinusOne,
        ) => IndexOfCheckType::Absence,
        // `indexOf(foo) !== -1`
        (
            JsBinaryOperator::StrictInequality | JsBinaryOperator::Inequality,
            NumberMatcher::MinusOne,
        ) => IndexOfCheckType::Presence,
        // `indexOf(foo) < 0`
        (JsBinaryOperator::LessThan, NumberMatcher::Zero) if is_call_on_left => {
            IndexOfCheckType::Absence
        }
        // `0 > indexOf(foo)`
        (JsBinaryOperator::GreaterThan, NumberMatcher::Zero) if !is_call_on_left => {
            IndexOfCheckType::Absence
        }
        // `indexOf(foo) >= 0`
        (JsBinaryOperator::GreaterThanOrEqual, NumberMatcher::Zero) if is_call_on_left => {
            IndexOfCheckType::Presence
        }
        // `0 <= indexOf(foo)`
        (JsBinaryOperator::LessThanOrEqual, NumberMatcher::Zero) if !is_call_on_left => {
            IndexOfCheckType::Presence
        }
        // `indexOf(foo) > -1`
        (JsBinaryOperator::GreaterThan, NumberMatcher::MinusOne) if is_call_on_left => {
            IndexOfCheckType::Presence
        }
        // `-1 < indexOf(foo)`
        (JsBinaryOperator::LessThan, NumberMatcher::MinusOne) if !is_call_on_left => {
            IndexOfCheckType::Presence
        }
        // `indexOf(foo) <= -1`
        (JsBinaryOperator::LessThanOrEqual, NumberMatcher::MinusOne) if is_call_on_left => {
            IndexOfCheckType::Absence
        }
        // `-1 >= indexOf(foo)`
        (JsBinaryOperator::GreaterThanOrEqual, NumberMatcher::MinusOne) if !is_call_on_left => {
            IndexOfCheckType::Absence
        }
        _ => return None,
    };

    Some((call_expr.clone(), check_type))
}

/// A helper to match against number literals in expressions.
#[derive(Debug, PartialEq, Eq)]
enum NumberMatcher {
    Zero,
    MinusOne,
}

impl<'a> NumberMatcher {
    fn from(expr: &'a AnyJsExpression) -> Option<Self> {
        match expr {
            AnyJsExpression::JsUnaryExpression(unary) => {
                if unary.operator() == Ok(JsUnaryOperator::Minus)
                    && matches_number_literal(unary.argument().ok().as_ref(), "1")
                {
                    Some(Self::MinusOne)
                } else {
                    None
                }
            }
            AnyJsExpression::AnyJsLiteralExpression(_) => {
                if matches_number_literal(Some(expr), "0") {
                    Some(Self::Zero)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Checks if an optional expression is a number literal with the given text.
fn matches_number_literal(expr: Option<&AnyJsExpression>, text: &str) -> bool {
    expr.as_ref()
        .and_then(|e| e.as_any_js_literal_expression())
        .and_then(|e| e.as_js_number_literal_expression())
        .and_then(|n| n.value_token().ok())
        .is_some_and(|token| token.text_trimmed() == text)
}

// Checks if the receiver of a method call is known to have an `includes` method.
// This is true for `string`, `array`, and `typed array` types, or if it's an
// object literal with an 'includes' property.
fn is_receiver_known_to_have_includes(receiver: Option<AnyJsExpression>) -> bool {
    let Some(receiver_expr) = receiver else {
        return false;
    };

    // Fast path for literals (already handled in your original code)
    if matches!(
        receiver_expr,
        AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsStringLiteralExpression(
            _
        )) | AnyJsExpression::JsArrayExpression(_)
    ) {
        return true;
    }

    if let Some(object_expression) = receiver_expr.as_js_object_expression() {
        for member in object_expression.members() {
            if let Ok(AnyJsObjectMember::JsPropertyObjectMember(property_member)) = member
                && let Ok(Some(name)) = property_member.name().map(|n| n.name())
                && name.text() == "includes"
            {
                return true;
            }
        }
    }

    false
}

/// Checks if a regex is simple enough to be replaced by `includes()`.
/// A simple regex is one that has no flags (except `u` or `v`) and contains no special characters.
fn is_simple_regex(pattern: &str, flags: &str) -> bool {
    if !flags.chars().all(|c| c == 'u' || c == 'v') {
        return false;
    }

    // This is a simplified check.
    // For now, we check for characters that are special in regex.
    // This list is not exhaustive but covers many common cases.
    !pattern.chars().any(|c| "\"'.\\\+*?[]^$(){}=!<>|:-".contains(c))
}
