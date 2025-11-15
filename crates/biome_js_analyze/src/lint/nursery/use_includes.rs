use crate::{JsRuleAction, services::typed::Typed};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression,
    JsBinaryExpression, JsBinaryOperator, JsCallExpression, JsUnaryOperator, T,
};
use biome_rowan::{AstNode, BatchMutationExt, declare_node_union};
use biome_rule_options::use_includes::UseIncludesOptions;

declare_lint_rule! {
    /// Enforce the use of `includes()` over `indexOf()`.
    ///
    /// Prior to ES2015, `Array#indexOf` and `String#indexOf` comparisons against `-1` were the standard ways to check whether a value exists in an array or string, respectively.
    /// ES2015 added `String#includes` and ES2016 added `Array#includes`, which are easier to read and write.
    ///
    /// This rule reports when an `.indexOf()` call can be replaced with an `.includes()`.
    /// Additionally, this rule reports the tests of simple regular expressions in favor of `String#includes`.
    ///
    /// This rule will report on any receiver object of an `indexOf` method call that has an `includes` method
    /// where the two methods have the same parameters. Matching types include: `String`, `Array`, `ReadonlyArray`, and typed arrays.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// "foo".indexOf("o") !== -1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// ["a", "b", "c"].indexOf("a") === -1
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /a/.test("abc")
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// "foo".includes("o");
    /// ```
    ///
    /// ```js
    /// !["a", "b", "c"].includes("a");
    /// ```
    ///
    /// ```js
    /// "abc".includes("a");
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
    IndexOf(IndexOfState),
    RegexTest(JsCallExpression),
}

pub struct IndexOfState {
    binary_expression: JsBinaryExpression,
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

                let member_expr = AnyJsMemberExpression::cast(
                    call_expr.callee().ok()?.omit_parentheses().into_syntax(),
                )?;

                if member_expr.member_name()?.text() != "indexOf" {
                    return None;
                }

                // Verify that the object has an `includes` method.
                if !is_receiver_known_to_have_includes(ctx, member_expr.object().ok()) {
                    return None;
                }

                Some(State::IndexOf(IndexOfState {
                    binary_expression: binary_expr.clone(),
                    is_negated: check_type.is_negated(),
                }))
            }
            UseIncludesQuery::JsCallExpression(call_expr) => {
                let member_expr = AnyJsMemberExpression::cast(
                    call_expr.callee().ok()?.omit_parentheses().into_syntax(),
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
                    Some(State::RegexTest(call_expr.clone()))
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            State::IndexOf(index_of_state) => Some(RuleDiagnostic::new(
                rule_category!(),
                index_of_state.binary_expression.range(),
                markup! {
                    "Use "<Emphasis>"includes()"</Emphasis>" instead of "<Emphasis>"indexOf()"</Emphasis>" to check for inclusion, as it is more readable and concise."
                },
            )),
            State::RegexTest(call_expr) => Some(RuleDiagnostic::new(
                rule_category!(),
                call_expr.range(),
                markup! {
                    "Use "<Emphasis>"String.prototype.includes()"</Emphasis>" instead of "<Emphasis>"RegExp.prototype.test()"</Emphasis>" for simple searches as it is more readable and concise."
                },
            )),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        match state {
            State::IndexOf(index_of_state) => {
                let mut mutation = ctx.root().begin();
                let (call_expr, _) = analyze_index_of_check(&index_of_state.binary_expression)?;

                let member_expr = AnyJsMemberExpression::cast(
                    call_expr.callee().ok()?.omit_parentheses().into_syntax(),
                )?;

                let new_member_expr = match member_expr {
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
                    call_expr.arguments().ok()?,
                )
                .build();

                new_call_expr = new_call_expr.with_leading_trivia_pieces([])?;

                let mut final_expr = if index_of_state.is_negated {
                    let unary_expr = make::js_unary_expression(
                        make::token(T![!]).with_trailing_trivia_pieces([]),
                        new_call_expr.with_leading_trivia_pieces([])?.into(),
                    );
                    AnyJsExpression::from(unary_expr)
                } else {
                    new_call_expr.into()
                };

                if let Some(trivia) = index_of_state
                    .binary_expression
                    .syntax()
                    .first_leading_trivia()
                {
                    final_expr = final_expr.with_leading_trivia_pieces(trivia.pieces())?;
                }
                if let Some(trivia) = index_of_state
                    .binary_expression
                    .syntax()
                    .last_trailing_trivia()
                {
                    final_expr = final_expr.with_trailing_trivia_pieces(trivia.pieces())?;
                }
                mutation.replace_node(
                    AnyJsExpression::from(index_of_state.binary_expression.clone()),
                    final_expr,
                );

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Replace with 'includes()'" }.to_owned(),
                    mutation,
                ))
            }
            State::RegexTest(call_expr) => {
                let mut mutation = ctx.root().begin();

                let member_expr =
                    AnyJsMemberExpression::cast(call_expr.callee().ok()?.into_syntax())?;
                let object = member_expr.object().ok()?;
                let regex_literal = object
                    .as_any_js_literal_expression()?
                    .as_js_regex_literal_expression()?;
                let (pattern, _) = regex_literal.decompose().ok()?;

                let string_literal = make::js_string_literal(pattern.text());
                let string_literal_expr = make::js_string_literal_expression(string_literal);

                let arguments = call_expr.arguments().ok()?;
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
                    AnyJsExpression::from(call_expr.clone()),
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

/// Checks if the receiver of a method call is known to have an `includes` method.
/// This is true for `string`, `array`, and `typed array` types.
fn is_receiver_known_to_have_includes(
    ctx: &RuleContext<UseIncludes>,
    receiver: Option<AnyJsExpression>,
) -> bool {
    let Some(receiver) = receiver else {
        return false;
    };

    // Fast path for literals
    if matches!(
        receiver,
        AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsStringLiteralExpression(
            _
        )) | AnyJsExpression::JsArrayExpression(_)
    ) {
        return true;
    }

    // Fallback to semantic model for other expressions
    let ty = ctx.type_of_expression(&receiver);
    ty.is_string() || ty.is_array_of(|_| true)
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
    !pattern.chars().any(|c| ".\\+*?[]^$(){}=!<>|:-".contains(c))
}
