use crate::{JsRuleAction, services::typed::Typed, utils::is_node_equal};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, AnyJsStatement, JsAssignmentExpression,
    JsAssignmentOperator,
    JsBinaryOperator,
    JsCallArgumentList, JsCallArguments, JsCallExpression, JsConditionalExpression,
    JsDoWhileStatement, JsForStatement, JsIfStatement, JsLogicalExpression, JsLogicalOperator,
    JsParenthesizedExpression, JsSyntaxKind, JsUnaryOperator, JsWhileStatement, OperatorPrecedence,
    T, static_value::StaticValue,
};
use biome_js_type_info::{IgnoredPrimitiveTypes, InferredType};
use biome_rowan::{
    AstNode, AstNodeList, BatchMutationExt, TextRange, declare_node_union,
    trim_leading_trivia_pieces,
};
use biome_rule_options::use_nullish_coalescing::UseNullishCoalescingOptions;

declare_lint_rule! {
    /// Enforce using the nullish coalescing operator (`??`) instead of logical or (`||`).
    ///
    /// `??` only checks for `null` and `undefined`, while `||` checks for any falsy value
    /// including `0`, `''`, and `false`. The rule reports `||` and `||=` when type analysis
    /// shows the left operand is possibly nullish, and ternary expressions that compare a
    /// value with `null` or `undefined` to pick a fallback. A strict comparison with only
    /// one of `null` and `undefined` is reported only when the value cannot hold the other
    /// one.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-or.ts
    /// declare const maybeString: string | null;
    /// const value = maybeString || 'default';
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-or-undefined.ts
    /// declare const maybeNumber: number | undefined;
    /// const value = maybeNumber || 0;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-or-assign.ts
    /// declare let x: string | null;
    /// x ||= 'default';
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-ternary-not-null.ts
    /// declare const x: string | null;
    /// const value = x !== null ? x : 'default';
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-ternary-loose-null.ts
    /// declare const x: string | null;
    /// const value = x == null ? 'default' : x;
    /// ```
    ///
    /// An `if` statement that only assigns to a nullish variable is also reported,
    /// since it can be rewritten as `??=`.
    ///
    /// ```ts,expect_diagnostic,file=invalid-if-assignment.ts
    /// declare let a: { x: string } | null;
    /// declare function makeA(): { x: string };
    /// if (!a) {
    ///     a = makeA();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid-nullish-coalescing.ts
    /// declare const maybeString: string | null;
    /// const value = maybeString ?? 'default';
    /// ```
    ///
    /// ```ts,file=valid-non-nullish.ts
    /// declare const definiteString: string;
    /// const value = definiteString || 'fallback';
    /// ```
    ///
    /// ```ts,file=valid-nullish-assign.ts
    /// declare let y: string | null;
    /// y ??= 'default';
    /// ```
    ///
    /// ## Options
    ///
    /// ### ignoreConditionalTests
    ///
    /// By default, Biome ignores `||` expressions inside the test of an `if`, `while`,
    /// `for`, or `do-while` statement or of a ternary expression. Set this to `false` to
    /// report them. Default: `true`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreConditionalTests": false
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-conditional-tests.ts
    /// declare const cond: string | null;
    /// if (cond || 'fallback') {}
    /// ```
    ///
    /// #### Valid
    ///
    /// With the default value, `||` in a conditional test is not reported.
    ///
    /// ```ts,file=valid-conditional-tests.ts
    /// declare const cond: string | null;
    /// while (cond || 'fallback') {}
    /// ```
    ///
    /// ### ignoreTernaryTests
    ///
    /// By default, Biome reports ternary expressions that compare a value with `null` or
    /// `undefined` and can be replaced with `??`. Set this to `true` to ignore them.
    /// Default: `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreTernaryTests": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// `||` is still reported when ternary expressions are ignored.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-ternary-tests.ts
    /// declare const maybeString: string | null;
    /// const value = maybeString || 'default';
    /// ```
    ///
    /// #### Valid
    ///
    /// ```ts,use_options,file=valid-ternary-tests.ts
    /// declare const x: string | null;
    /// const value = x !== null ? x : 'default';
    /// ```
    ///
    /// ### ignoreMixedLogicalExpressions
    ///
    /// By default, Biome reports `||` and `||=` even when they are combined with `&&`, as in
    /// `(a || b) && c` or `a || (b && c)`. Set this to `true` to ignore `||` and `||=` whose
    /// connected logical tree also contains a `&&`. Default: `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreMixedLogicalExpressions": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// `||` and `||=` are still reported when the surrounding logical tree does not contain `&&`.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-mixed-or.ts
    /// declare const maybeString: string | null;
    /// const value = maybeString || 'default';
    /// ```
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-mixed-or-assign.ts
    /// declare let assigned: string | null;
    /// assigned ||= 'default';
    /// ```
    ///
    /// #### Valid
    ///
    /// `||` and `||=` mixed with `&&` in the same logical tree are not reported.
    ///
    /// ```ts,use_options,file=valid-mixed-or.ts
    /// declare const a: string | null;
    /// declare const b: string;
    /// const r = (a || 'default') && b;
    /// ```
    ///
    /// ```ts,use_options,file=valid-mixed-or-assign.ts
    /// declare const b: string;
    /// declare let assigned: string | null;
    /// assigned ||= b && 'fallback';
    /// ```
    ///
    /// ### ignoreBooleanCoercion
    ///
    /// By default, Biome reports `||` and `||=` even when they are the argument of a call to
    /// the global `Boolean()` function. Set this to `true` to ignore them there, where
    /// coalescing on falsy values is intentional. Default: `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreBooleanCoercion": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// `||` and `||=` that are not the argument of a `Boolean()` call are still reported.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-boolean-coercion.ts
    /// declare const maybeString: string | null;
    /// const value = maybeString || 'default';
    /// ```
    ///
    /// #### Valid
    ///
    /// `||` and `||=` that are the argument of a `Boolean()` call are not reported. The
    /// argument may be parenthesized or combined with other `||` and `&&` operators.
    ///
    /// ```ts,use_options,file=valid-boolean-coercion.ts
    /// declare const a: string | null;
    /// declare const b: string;
    /// const r = Boolean(a || b);
    /// ```
    ///
    /// ### ignorePrimitives
    ///
    /// By default, Biome reports `||`, `||=`, ternary expressions, and `if` statements
    /// whatever primitive types their operand includes. Set this to `true` to ignore them
    /// when the operand is a nullable union whose non-nullish variants are all `string`,
    /// `number`, `boolean`, or `bigint`, or to an object with any of the keys `string`,
    /// `number`, `boolean`, and `bigint` set to `true` to ignore only unions made of those
    /// kinds. Other types, such as a `symbol | null` union or a non-union `string`, are
    /// still reported. Default: none.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignorePrimitives": { "string": true }
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// Primitive kinds that are not opted out of are still reported.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-primitives.ts
    /// declare const count: number | null;
    /// const value = count || 0;
    /// ```
    ///
    /// #### Valid
    ///
    /// A `string` operand is not reported when `string` is ignored.
    ///
    /// ```ts,use_options,file=valid-primitives.ts
    /// declare const name: string | null;
    /// const value = name || 'default';
    /// ```
    ///
    /// ### ignoreIfStatements
    ///
    /// By default, Biome reports an `if` statement that only assigns to a nullish variable,
    /// since it can be rewritten as `??=`. Set this to `true` to ignore those statements.
    /// Default: `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreIfStatements": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// `||` and `||=` are still reported when only `if` statements are ignored.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-if-statements.ts
    /// declare const maybeString: string | null;
    /// const value = maybeString || 'default';
    /// ```
    ///
    /// #### Valid
    ///
    /// An `if` statement performing a nullish assignment is not reported.
    ///
    /// ```ts,use_options,file=valid-if-statements.ts
    /// declare let a: { x: string } | null;
    /// declare function makeA(): { x: string };
    /// if (!a) {
    ///     a = makeA();
    /// }
    /// ```
    ///
    pub UseNullishCoalescing {
        version: "2.4.5",
        name: "useNullishCoalescing",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("prefer-nullish-coalescing").inspired()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Types],
        issue_number: Some("8043"),
    }
}

declare_node_union! {
    pub UseNullishCoalescingQuery = JsLogicalExpression | JsAssignmentExpression | JsConditionalExpression | JsIfStatement
}

declare_node_union! {
    pub AnyJsLogicalOrLikeExpression = JsLogicalExpression | JsAssignmentExpression
}

impl AnyJsLogicalOrLikeExpression {
    fn is_logical_or_form(&self) -> bool {
        match self {
            Self::JsLogicalExpression(logical) => {
                logical.operator().ok() == Some(JsLogicalOperator::LogicalOr)
            }
            Self::JsAssignmentExpression(assignment) => {
                assignment.operator().ok() == Some(JsAssignmentOperator::LogicalOrAssign)
            }
        }
    }

    fn is_logical_and(&self) -> bool {
        matches!(
            self,
            Self::JsLogicalExpression(logical)
                if logical.operator().ok() == Some(JsLogicalOperator::LogicalAnd)
        )
    }
}

pub enum UseNullishCoalescingState {
    LogicalOr {
        operator_range: TextRange,
        can_fix: bool,
    },
    LogicalOrAssignment {
        operator_range: TextRange,
        can_fix: bool,
    },
    Ternary {
        test_range: TextRange,
        /// The branch taken when the tested value is present, which becomes the left
        /// operand of `??`.
        subject: AnyJsExpression,
        /// The branch taken when the tested value is missing, which becomes the right
        /// operand of `??`.
        fallback: AnyJsExpression,
        /// Whether the consequent is the branch taken when the tested value is present.
        is_positive: bool,
        can_fix: bool,
    },
    IfStatement {
        if_range: TextRange,
        assignment_target: AnyJsAssignmentPattern,
        assignment_value: AnyJsExpression,
        can_fix: bool,
    },
}

impl Rule for UseNullishCoalescing {
    type Query = Typed<UseNullishCoalescingQuery>;
    type State = UseNullishCoalescingState;
    type Signals = Option<Self::State>;
    type Options = UseNullishCoalescingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            UseNullishCoalescingQuery::JsLogicalExpression(logical) => {
                run_logical_or(ctx, logical)
            }
            UseNullishCoalescingQuery::JsAssignmentExpression(assignment) => {
                run_logical_or_assignment(ctx, assignment)
            }
            UseNullishCoalescingQuery::JsConditionalExpression(ternary) => {
                run_ternary(ctx, ternary)
            }
            UseNullishCoalescingQuery::JsIfStatement(if_stmt) => run_if_statement(ctx, if_stmt),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            UseNullishCoalescingState::LogicalOr { operator_range, .. } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *operator_range,
                    markup! {
                        "Use "<Emphasis>"??"</Emphasis>" instead of "<Emphasis>"||"</Emphasis>"."
                    },
                )
                .note(markup! {
                    "The "<Emphasis>"||"</Emphasis>" operator checks for all falsy values (including 0, '', and false), while "<Emphasis>"??"</Emphasis>" only checks for null and undefined."
                }),
            ),
            UseNullishCoalescingState::LogicalOrAssignment { operator_range, .. } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *operator_range,
                    markup! {
                        "Use "<Emphasis>"??="</Emphasis>" instead of "<Emphasis>"||="</Emphasis>"."
                    },
                )
                .note(markup! {
                    "The "<Emphasis>"||="</Emphasis>" operator assigns when the left side is falsy, while "<Emphasis>"??="</Emphasis>" only assigns when it is null or undefined."
                }),
            ),
            UseNullishCoalescingState::Ternary { test_range, .. } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *test_range,
                    markup! {
                        "Prefer "<Emphasis>"??"</Emphasis>" over a ternary expression checking for nullish."
                    },
                )
                .note(markup! {
                    "A ternary expression that checks for "<Emphasis>"null"</Emphasis>" or "<Emphasis>"undefined"</Emphasis>" does what "<Emphasis>"??"</Emphasis>" does, but "<Emphasis>"??"</Emphasis>" is shorter and easier to read."
                }),
            ),
            UseNullishCoalescingState::IfStatement { if_range, .. } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *if_range,
                    markup! {
                        "Use "<Emphasis>"??="</Emphasis>" instead of an "<Emphasis>"if"</Emphasis>" statement for nullish assignment."
                    },
                )
                .note(markup! {
                    "This "<Emphasis>"if"</Emphasis>" statement only assigns when the variable is nullish, which "<Emphasis>"??="</Emphasis>" expresses directly."
                }),
            ),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let query = ctx.query();
        let mut mutation = ctx.root().begin();

        let message = match (state, query) {
            (
                UseNullishCoalescingState::LogicalOr { can_fix: true, .. },
                UseNullishCoalescingQuery::JsLogicalExpression(logical),
            ) => {
                let old_token = logical.operator_token().ok()?;

                let new_token = make::token(T![??])
                    .with_leading_trivia_pieces(old_token.leading_trivia().pieces())
                    .with_trailing_trivia_pieces(old_token.trailing_trivia().pieces());

                mutation.replace_token(old_token, new_token);
                markup! { "Replace "<Emphasis>"||"</Emphasis>" with "<Emphasis>"??"</Emphasis>"." }.to_owned()
            }
            (
                UseNullishCoalescingState::LogicalOrAssignment { can_fix: true, .. },
                UseNullishCoalescingQuery::JsAssignmentExpression(assignment),
            ) => {
                let old_token = assignment.operator_token().ok()?;

                let new_token = make::token(T![??=])
                    .with_leading_trivia_pieces(old_token.leading_trivia().pieces())
                    .with_trailing_trivia_pieces(old_token.trailing_trivia().pieces());

                mutation.replace_token(old_token, new_token);
                markup! { "Replace "<Emphasis>"||="</Emphasis>" with "<Emphasis>"??="</Emphasis>"." }.to_owned()
            }
            (
                UseNullishCoalescingState::Ternary {
                    subject,
                    fallback,
                    is_positive,
                    can_fix: true,
                    ..
                },
                UseNullishCoalescingQuery::JsConditionalExpression(ternary),
            ) => {
                // Strip trailing whitespace. Ternary layout trivia is not appropriate for `??`.
                let subject = subject.clone().trim_trailing_trivia()?;
                let fallback = fallback.clone().trim_trailing_trivia()?;

                // Transfer trivia from the ? and : tokens to the branch expressions they precede.
                let question = ternary.question_mark_token().ok()?;
                let colon = ternary.colon_token().ok()?;
                let question_trivia =
                    trim_leading_trivia_pieces(question.trailing_trivia().pieces());
                let colon_trivia =
                    trim_leading_trivia_pieces(colon.trailing_trivia().pieces());

                let (subject, fallback) = if *is_positive {
                    (
                        subject.prepend_trivia_pieces(question_trivia)?,
                        fallback.prepend_trivia_pieces(colon_trivia)?,
                    )
                } else {
                    (
                        subject.prepend_trivia_pieces(colon_trivia)?,
                        fallback.prepend_trivia_pieces(question_trivia)?,
                    )
                };

                let new_expr = make::js_logical_expression(
                    maybe_parenthesize_for_nullish(subject),
                    make::token_decorated_with_space(T![??]),
                    maybe_parenthesize_for_nullish(fallback),
                );

                // Transfer leading/trailing trivia from the original ternary
                let new_expr = AnyJsExpression::from(new_expr)
                    .prepend_trivia_pieces(
                        ternary.syntax().first_leading_trivia()?.pieces(),
                    )?
                    .append_trivia_pieces(
                        ternary.syntax().last_trailing_trivia()?.pieces(),
                    )?;

                mutation.replace_node_discard_trivia(
                    AnyJsExpression::from(ternary.clone()),
                    new_expr,
                );
                markup! { "Replace the ternary with "<Emphasis>"??"</Emphasis>"." }.to_owned()
            }
            (
                UseNullishCoalescingState::IfStatement {
                    assignment_target,
                    assignment_value,
                    can_fix: true,
                    ..
                },
                UseNullishCoalescingQuery::JsIfStatement(if_stmt),
            ) => {
                let target = assignment_target.clone().trim_trivia()?;
                let value = assignment_value.clone().trim_trivia()?;

                let new_assignment = make::js_assignment_expression(
                    target,
                    make::token_decorated_with_space(T![??=]),
                    value,
                );

                let new_stmt = make::js_expression_statement(AnyJsExpression::from(new_assignment))
                    .with_semicolon_token(make::token(T![;]))
                    .build();

                // Transfer leading/trailing trivia from the original if statement.
                let new_stmt = AnyJsStatement::from(new_stmt)
                    .prepend_trivia_pieces(if_stmt.syntax().first_leading_trivia()?.pieces())?
                    .append_trivia_pieces(if_stmt.syntax().last_trailing_trivia()?.pieces())?;

                mutation.replace_node_discard_trivia(
                    AnyJsStatement::from(if_stmt.clone()),
                    new_stmt,
                );
                markup! { "Replace the "<Emphasis>"if"</Emphasis>" statement with "<Emphasis>"??="</Emphasis>"." }.to_owned()
            }
            _ => return None,
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}

/// A ternary or `if` test that decides whether a value is missing.
enum NullishTest {
    /// `!x`, where `x` is a variable or a property.
    Falsy(AnyJsExpression),
    /// A comparison with `==`, `!=`, `===`, or `!==`, or two strict ones joined by `||` or `&&`.
    Comparison {
        /// The operator of the comparison, or of both comparisons of a compound.
        operator: JsBinaryOperator,
        first: (AnyJsExpression, AnyJsExpression),
        second: Option<(AnyJsExpression, AnyJsExpression)>,
    },
}

impl NullishTest {
    /// Reads how `test` checks for a missing value, or returns `None` for any other test.
    fn from_test(test: AnyJsExpression) -> Option<Self> {
        match test {
            AnyJsExpression::JsUnaryExpression(unary) => {
                if unary.operator().ok()? != JsUnaryOperator::LogicalNot {
                    return None;
                }
                let argument = unary.argument().ok()?;
                is_variable_or_property(&argument).then_some(Self::Falsy(argument))
            }
            AnyJsExpression::JsBinaryExpression(binary) => {
                let operator = binary.operator().ok()?;
                if !matches!(
                    operator,
                    JsBinaryOperator::Equality
                        | JsBinaryOperator::Inequality
                        | JsBinaryOperator::StrictEquality
                        | JsBinaryOperator::StrictInequality
                ) {
                    return None;
                }
                Some(Self::Comparison {
                    operator,
                    first: (binary.left().ok()?, binary.right().ok()?),
                    second: None,
                })
            }
            AnyJsExpression::JsLogicalExpression(logical) => {
                let (
                    AnyJsExpression::JsBinaryExpression(left),
                    AnyJsExpression::JsBinaryExpression(right),
                ) = (logical.left().ok()?, logical.right().ok()?)
                else {
                    return None;
                };
                let operator = compound_operator(
                    logical.operator().ok()?,
                    left.operator().ok()?,
                    right.operator().ok()?,
                )?;
                Some(Self::Comparison {
                    operator,
                    first: (left.left().ok()?, left.right().ok()?),
                    second: Some((right.left().ok()?, right.right().ok()?)),
                })
            }
            _ => None,
        }
    }

    /// Whether the test is true when the value is missing, so the ternary's consequent
    /// or the `if` body provides the fallback.
    fn is_true_when_missing(&self) -> bool {
        match self {
            Self::Falsy(_) => true,
            Self::Comparison { operator, .. } => matches!(
                operator,
                JsBinaryOperator::Equality | JsBinaryOperator::StrictEquality
            ),
        }
    }

    /// Returns the tested value that `is_subject` accepts and how the test checks it, or
    /// `None` unless every comparison compares `null` or `undefined` with that value.
    /// The operand of a second comparison must equal the one of the first.
    fn into_subject(self, is_subject: impl Fn(&AnyJsExpression) -> bool) -> Option<NullishSubject> {
        let (operator, first, second) = match self {
            Self::Falsy(expression) => {
                return is_subject(&expression).then_some(NullishSubject {
                    expression,
                    check: NullishCheck::Truthiness,
                });
            }
            Self::Comparison {
                operator,
                first,
                second,
            } => (operator, first, second),
        };

        let (first_literal, expression) = nullish_comparison(first, is_subject)?;
        let second_literal = match second {
            Some(second) => {
                let (literal, _) = nullish_comparison(second, |operand| {
                    expressions_equivalent(operand, &expression)
                })?;
                Some(literal)
            }
            None => None,
        };

        if matches!(
            operator,
            JsBinaryOperator::Equality | JsBinaryOperator::Inequality
        ) {
            return Some(NullishSubject {
                expression,
                check: NullishCheck::Nullish,
            });
        }

        let check = match (first_literal, second_literal) {
            (NullishLiteral::Null, Some(NullishLiteral::Undefined))
            | (NullishLiteral::Undefined, Some(NullishLiteral::Null)) => NullishCheck::Nullish,
            (literal, _) => NullishCheck::Strict(literal),
        };
        Some(NullishSubject { expression, check })
    }
}

/// The value that a [`NullishTest`] checks, and how it checks it.
struct NullishSubject {
    expression: AnyJsExpression,
    check: NullishCheck,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum NullishCheck {
    /// `!x`, which also treats `0`, `''`, and `false` as missing.
    Truthiness,
    /// Covers both `null` and `undefined`: a loose comparison with either, or strict
    /// comparisons with both.
    Nullish,
    /// A strict comparison with only one of `null` and `undefined`.
    Strict(NullishLiteral),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum NullishLiteral {
    Null,
    Undefined,
}

impl NullishLiteral {
    fn from_expression(expression: &AnyJsExpression) -> Option<Self> {
        match expression.as_static_value()? {
            StaticValue::Null(_) => Some(Self::Null),
            StaticValue::Undefined(_) => Some(Self::Undefined),
            _ => None,
        }
    }

    /// Whether a strict comparison with this literal catches every nullish variant of
    /// `ty`, or `None` when the type cannot be resolved.
    fn covers_nullish_variants_of(self, ty: InferredType) -> Option<bool> {
        let misses_a_variant = match self {
            Self::Null => ty.has_undefined_variant()?,
            Self::Undefined => ty.has_null_variant()?,
        };
        Some(!misses_a_variant)
    }
}

fn run_logical_or(
    ctx: &RuleContext<UseNullishCoalescing>,
    logical: &JsLogicalExpression,
) -> Option<UseNullishCoalescingState> {
    let operator = logical.operator().ok()?;
    if operator != JsLogicalOperator::LogicalOr {
        return None;
    }

    let options = ctx.options();
    if options.ignore_conditional_tests() && is_in_test_position(logical) {
        return None;
    }

    let node = AnyJsLogicalOrLikeExpression::from(logical.clone());
    if options.ignore_mixed_logical_expressions() && is_in_mixed_logical_tree(&node) {
        return None;
    }

    if options.ignore_boolean_coercion() && is_in_boolean_call_context(ctx, &node) {
        return None;
    }

    let left_ty = ctx.type_of_expression(&logical.left().ok()?)?;
    let can_fix = report_truthiness(options, left_ty)? && can_replace_operator(logical);

    Some(UseNullishCoalescingState::LogicalOr {
        operator_range: logical.operator_token().ok()?.text_trimmed_range(),
        can_fix,
    })
}

fn run_logical_or_assignment(
    ctx: &RuleContext<UseNullishCoalescing>,
    assignment: &JsAssignmentExpression,
) -> Option<UseNullishCoalescingState> {
    let operator = assignment.operator().ok()?;
    if operator != JsAssignmentOperator::LogicalOrAssign {
        return None;
    }

    let options = ctx.options();
    let node = AnyJsLogicalOrLikeExpression::from(assignment.clone());
    if options.ignore_mixed_logical_expressions() && is_in_mixed_logical_tree(&node) {
        return None;
    }

    if options.ignore_boolean_coercion() && is_in_boolean_call_context(ctx, &node) {
        return None;
    }

    let left = assignment.left().ok()?;
    let left_ty = match &left {
        AnyJsAssignmentPattern::AnyJsAssignment(assign) => {
            let id = assign.as_js_identifier_assignment()?;
            let name = id.name_token().ok()?;
            ctx.type_of_named_value(assignment.range(), name.text_trimmed())?
        }
        _ => return None,
    };
    let can_fix = report_truthiness(options, left_ty)?;

    Some(UseNullishCoalescingState::LogicalOrAssignment {
        operator_range: assignment.operator_token().ok()?.text_trimmed_range(),
        can_fix,
    })
}

fn run_ternary(
    ctx: &RuleContext<UseNullishCoalescing>,
    ternary: &JsConditionalExpression,
) -> Option<UseNullishCoalescingState> {
    if ctx.options().ignore_ternary_tests() {
        return None;
    }

    let test = ternary.test().ok()?;
    let test_range = test.syntax().text_trimmed_range();
    let consequent = ternary.consequent().ok()?;
    let alternate = ternary.alternate().ok()?;

    let nullish_test = NullishTest::from_test(test)?;
    if matches!(nullish_test, NullishTest::Falsy(_)) {
        return None;
    }
    let is_positive = !nullish_test.is_true_when_missing();
    let (present_branch, fallback) = if is_positive {
        (consequent, alternate)
    } else {
        (alternate, consequent)
    };
    let check = nullish_test
        .into_subject(|operand| expressions_equivalent(operand, &present_branch))?
        .check;

    // The branch is what `??` keeps, so it is the expression typed and rewritten.
    let subject = NullishSubject {
        expression: present_branch,
        check,
    };
    // The ternary evaluates a subject containing a call or `new` twice (test + branch)
    // while `??` evaluates it once.
    let can_fix =
        report_check(ctx, &subject)? && !contains_call_or_new_expression(&subject.expression);

    Some(UseNullishCoalescingState::Ternary {
        test_range,
        subject: subject.expression,
        fallback,
        is_positive,
        can_fix,
    })
}

fn run_if_statement(
    ctx: &RuleContext<UseNullishCoalescing>,
    if_stmt: &JsIfStatement,
) -> Option<UseNullishCoalescingState> {
    if ctx.options().ignore_if_statements() {
        return None;
    }

    // An `else` branch means the statement does more than a nullish fallback.
    if if_stmt.else_clause().is_some() {
        return None;
    }

    let assignment = if_body_assignment(if_stmt)?;
    let target = assignment.left().ok()?;

    // The test must check the same reference that is assigned in the body.
    let nullish_test = NullishTest::from_test(if_stmt.test().ok()?)?;
    if !nullish_test.is_true_when_missing() {
        return None;
    }
    let subject = nullish_test
        .into_subject(|operand| subject_matches_assignment_target(operand, &target))?;

    // Unlike a ternary, an `if` is reported only for a subject that can be nullish.
    if !ctx
        .type_of_expression(&subject.expression)?
        .has_nullish_variant()?
    {
        return None;
    }

    // The original `if` evaluates the subject twice (test + assignment) while `??=`
    // evaluates it once, so the rewrite only preserves semantics when re-evaluating the
    // reference is side-effect-free and stable (e.g. `getObj().x` or `arr[i++]` are not).
    // Inner comments would be discarded by the rewrite, so skip the fix in that case too.
    let can_fix = report_check(ctx, &subject)?
        && is_stable_reference(&subject.expression)
        && !if_stmt.syntax().has_inner_comments();

    Some(UseNullishCoalescingState::IfStatement {
        if_range: if_stmt.syntax().text_trimmed_range(),
        assignment_target: target,
        assignment_value: assignment.right().ok()?,
        can_fix,
    })
}

/// Returns the single plain `=` assignment in an `if` body, either
/// `if (test) { target = value; }` or `if (test) target = value;`.
/// `||=` and `??=` bodies are left to the operator logic, avoiding a double report.
fn if_body_assignment(if_stmt: &JsIfStatement) -> Option<JsAssignmentExpression> {
    let statement = match if_stmt.consequent().ok()? {
        AnyJsStatement::JsBlockStatement(block) => {
            let mut statements = block.statements().iter();
            let first = statements.next()?;
            if statements.next().is_some() {
                return None;
            }
            first
        }
        statement => statement,
    };

    let assignment = statement
        .as_js_expression_statement()?
        .expression()
        .ok()?
        .as_js_assignment_expression()?
        .clone();
    (assignment.operator().ok()? == JsAssignmentOperator::Assign).then_some(assignment)
}

/// Returns `None` when the check of `subject` is not reported, otherwise whether the
/// `??` or `??=` rewrite keeps its behavior.
fn report_check(ctx: &RuleContext<UseNullishCoalescing>, subject: &NullishSubject) -> Option<bool> {
    let options = ctx.options();
    let ty = ctx.type_of_expression(&subject.expression);
    if subject.check == NullishCheck::Truthiness {
        return report_truthiness(options, ty?);
    }

    if options.has_any_ignore_primitives()
        && let Some(ty) = ty
        && should_ignore_for_primitives(options, ty)?
    {
        return None;
    }

    if let NullishCheck::Strict(literal) = subject.check {
        return literal.covers_nullish_variants_of(ty?)?.then_some(true);
    }
    Some(true)
}

/// Returns `None` when a truthiness check of a value of type `ty` is not reported,
/// otherwise whether `??` keeps its behavior.
fn report_truthiness(options: &UseNullishCoalescingOptions, ty: InferredType) -> Option<bool> {
    if !ty.has_nullish_variant()? {
        return None;
    }

    if options.has_any_ignore_primitives() && should_ignore_for_primitives(options, ty)? {
        return None;
    }

    ty.is_safe_for_nullish_coalescing()
}

/// Returns `true` when `ty` is a nullish union whose non-nullish variants are all
/// primitives that the configured `ignorePrimitives` option suppresses. A non-union
/// type is never ignored.
fn should_ignore_for_primitives(
    options: &UseNullishCoalescingOptions,
    ty: InferredType,
) -> Option<bool> {
    ty.nullish_union_matches_ignored_primitives(IgnoredPrimitiveTypes {
        string: options.should_ignore_primitive_string(),
        number: options.should_ignore_primitive_number(),
        boolean: options.should_ignore_primitive_boolean(),
        bigint: options.should_ignore_primitive_bigint(),
    })
}

fn is_in_test_position(logical: &JsLogicalExpression) -> bool {
    let logical_range = logical.syntax().text_trimmed_range();

    let test_contains_logical = |test: Option<AnyJsExpression>| -> bool {
        test.is_some_and(|t| t.syntax().text_trimmed_range().contains_range(logical_range))
    };

    for ancestor in logical.syntax().ancestors() {
        let is_in_test = match ancestor.kind() {
            JsSyntaxKind::JS_IF_STATEMENT => test_contains_logical(
                JsIfStatement::cast_ref(&ancestor).and_then(|s| s.test().ok()),
            ),
            JsSyntaxKind::JS_WHILE_STATEMENT => test_contains_logical(
                JsWhileStatement::cast_ref(&ancestor).and_then(|s| s.test().ok()),
            ),
            JsSyntaxKind::JS_FOR_STATEMENT => test_contains_logical(
                JsForStatement::cast_ref(&ancestor).and_then(|s| s.test()),
            ),
            JsSyntaxKind::JS_DO_WHILE_STATEMENT => test_contains_logical(
                JsDoWhileStatement::cast_ref(&ancestor).and_then(|s| s.test().ok()),
            ),
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => test_contains_logical(
                JsConditionalExpression::cast_ref(&ancestor).and_then(|s| s.test().ok()),
            ),
            _ => false,
        };
        if is_in_test {
            return true;
        }
    }
    false
}

/// Whether the `||` or `||=` `node` is an argument of a call to the global `Boolean`,
/// reached through parentheses and logical expressions. A `Boolean` bound in scope is a
/// different function.
fn is_in_boolean_call_context(
    ctx: &RuleContext<UseNullishCoalescing>,
    node: &AnyJsLogicalOrLikeExpression,
) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| {
            !(JsParenthesizedExpression::can_cast(ancestor.kind())
                || JsLogicalExpression::can_cast(ancestor.kind()))
        })
        .and_then(JsCallArgumentList::cast)
        .and_then(|list| list.parent::<JsCallArguments>())
        .and_then(|args| args.parent::<JsCallExpression>())
        .and_then(|call| call.callee().ok())
        .is_some_and(|callee| {
            callee
                .as_js_reference_identifier()
                .is_some_and(|reference| {
                    reference.has_name("Boolean") && !ctx.has_binding(&reference)
                })
        })
}

/// Whether `node`, a `||` or `||=`, is combined with `&&`: a `&&` is an operand of the
/// chain of `||` and `||=` that `node` belongs to, or that chain is an operand of `&&`.
fn is_in_mixed_logical_tree(node: &AnyJsLogicalOrLikeExpression) -> bool {
    let mut root = node.clone();
    while let Some(parent) =
        parent_logical(&root).filter(AnyJsLogicalOrLikeExpression::is_logical_or_form)
    {
        root = parent;
    }
    parent_logical(&root).is_some_and(|parent| parent.is_logical_and())
        || chain_contains_logical_and(&root)
}

/// Returns the logical expression or assignment that `node` is an operand of, ignoring
/// parentheses.
fn parent_logical(node: &AnyJsLogicalOrLikeExpression) -> Option<AnyJsLogicalOrLikeExpression> {
    let mut parent = node.parent::<AnyJsExpression>()?;
    while let AnyJsExpression::JsParenthesizedExpression(parenthesized) = parent {
        parent = parenthesized.parent::<AnyJsExpression>()?;
    }
    AnyJsLogicalOrLikeExpression::cast(parent.into_syntax())
}

/// Whether a `&&` is an operand of `node` or of a `||` or `||=` reached from `node`
/// through operands, where only the right-hand side of a `||=` is an operand.
fn chain_contains_logical_and(node: &AnyJsLogicalOrLikeExpression) -> bool {
    let mut pending = match node {
        AnyJsLogicalOrLikeExpression::JsLogicalExpression(logical) => {
            vec![logical.left(), logical.right()]
        }
        AnyJsLogicalOrLikeExpression::JsAssignmentExpression(assignment) => {
            vec![assignment.right()]
        }
    };
    while let Some(operand) = pending.pop() {
        match operand.ok().map(AnyJsExpression::omit_parentheses) {
            Some(AnyJsExpression::JsLogicalExpression(logical)) => match logical.operator() {
                Ok(JsLogicalOperator::LogicalAnd) => return true,
                Ok(JsLogicalOperator::LogicalOr) => {
                    pending.extend([logical.left(), logical.right()]);
                }
                _ => {}
            },
            Some(AnyJsExpression::JsAssignmentExpression(assignment))
                if assignment.operator().ok() == Some(JsAssignmentOperator::LogicalOrAssign) =>
            {
                pending.push(assignment.right());
            }
            _ => {}
        }
    }
    false
}

/// Whether the `if` test's `subject` and the body's assignment `target` reference the
/// same value. Assignment targets and expressions have different node kinds, so each
/// shape is compared field by field instead of as a single subtree.
fn subject_matches_assignment_target(
    subject: &AnyJsExpression,
    target: &AnyJsAssignmentPattern,
) -> bool {
    let AnyJsAssignmentPattern::AnyJsAssignment(target) = target else {
        return false;
    };
    match (subject, target) {
        (
            AnyJsExpression::JsIdentifierExpression(subject),
            AnyJsAssignment::JsIdentifierAssignment(target),
        ) => subject.name().is_ok_and(|name| {
            name.value_token().is_ok_and(|token| {
                target
                    .name_token()
                    .is_ok_and(|target_token| token.text_trimmed() == target_token.text_trimmed())
            })
        }),
        (
            AnyJsExpression::JsStaticMemberExpression(subject),
            AnyJsAssignment::JsStaticMemberAssignment(target),
        ) => {
            let same_object = subject
                .object()
                .ok()
                .zip(target.object().ok())
                .is_some_and(|(a, b)| expressions_equivalent(&a, &b));
            let same_member = subject
                .member()
                .ok()
                .zip(target.member().ok())
                .is_some_and(|(a, b)| is_node_equal(a.syntax(), b.syntax()));
            same_object && same_member
        }
        (
            AnyJsExpression::JsComputedMemberExpression(subject),
            AnyJsAssignment::JsComputedMemberAssignment(target),
        ) => {
            let same_object = subject
                .object()
                .ok()
                .zip(target.object().ok())
                .is_some_and(|(a, b)| expressions_equivalent(&a, &b));
            let same_member = subject
                .member()
                .ok()
                .zip(target.member().ok())
                .is_some_and(|(a, b)| expressions_equivalent(&a, &b));
            same_object && same_member
        }
        _ => false,
    }
}

/// Splits a comparison into the nullish literal it compares with and the operand that
/// `is_subject` accepts.
fn nullish_comparison(
    (left, right): (AnyJsExpression, AnyJsExpression),
    is_subject: impl Fn(&AnyJsExpression) -> bool,
) -> Option<(NullishLiteral, AnyJsExpression)> {
    let (literal, operand) = match (
        NullishLiteral::from_expression(&left),
        NullishLiteral::from_expression(&right),
    ) {
        (Some(literal), None) => (literal, right),
        (None, Some(literal)) => (literal, left),
        _ => return None,
    };
    is_subject(&operand).then_some((literal, operand))
}

/// Returns the operator that a compound of two comparisons acts as, or `None` when
/// the compound does not check for a missing value.
fn compound_operator(
    logical: JsLogicalOperator,
    left: JsBinaryOperator,
    right: JsBinaryOperator,
) -> Option<JsBinaryOperator> {
    match (logical, left, right) {
        (
            JsLogicalOperator::LogicalOr,
            JsBinaryOperator::StrictEquality,
            JsBinaryOperator::StrictEquality,
        ) => Some(JsBinaryOperator::StrictEquality),
        (
            JsLogicalOperator::LogicalAnd,
            JsBinaryOperator::StrictInequality,
            JsBinaryOperator::StrictInequality,
        ) => Some(JsBinaryOperator::StrictInequality),
        _ => None,
    }
}

fn is_variable_or_property(expression: &AnyJsExpression) -> bool {
    matches!(
        expression,
        AnyJsExpression::JsIdentifierExpression(_)
            | AnyJsExpression::JsStaticMemberExpression(_)
            | AnyJsExpression::JsComputedMemberExpression(_)
    )
}

fn expressions_equivalent(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    is_node_equal(a.syntax(), b.syntax())
}

/// Whether re-evaluating `expr` is side-effect-free and yields the same reference.
/// Rewriting an `if` into `??=` collapses two evaluations of the subject into one, so
/// the rewrite only preserves semantics for stable references. Calls, updates
/// (`i++`), and other observable expressions anywhere in the reference are rejected.
fn is_stable_reference(expr: &AnyJsExpression) -> bool {
    let mut current = expr.clone();
    loop {
        current = match current {
            AnyJsExpression::JsIdentifierExpression(_) | AnyJsExpression::JsThisExpression(_) => {
                return true;
            }
            AnyJsExpression::JsParenthesizedExpression(paren) => match paren.expression() {
                Ok(inner) => inner,
                Err(_) => return false,
            },
            AnyJsExpression::JsStaticMemberExpression(member) => match member.object() {
                Ok(object) => object,
                Err(_) => return false,
            },
            AnyJsExpression::JsComputedMemberExpression(member) => {
                if !member.member().is_ok_and(|index| is_stable_index(&index)) {
                    return false;
                }
                match member.object() {
                    Ok(object) => object,
                    Err(_) => return false,
                }
            }
            _ => return false,
        };
    }
}

/// A computed-member index is stable when it is a literal or another stable reference,
/// so `obj['a']` and `obj[key]` are fixable but `arr[i++]` is not.
fn is_stable_index(expr: &AnyJsExpression) -> bool {
    matches!(expr, AnyJsExpression::AnyJsLiteralExpression(_)) || is_stable_reference(expr)
}

/// Returns true if the expression contains a call or `new` expression.
///
/// In a ternary like `foo() !== null ? foo() : y`, the subject is evaluated twice
/// (once in the test, once in the branch). Replacing with `foo() ?? y` evaluates it
/// once, which is a semantic change if the call has side effects.
fn contains_call_or_new_expression(expr: &AnyJsExpression) -> bool {
    expr.syntax().descendants().any(|node| {
        matches!(
            node.kind(),
            JsSyntaxKind::JS_CALL_EXPRESSION | JsSyntaxKind::JS_NEW_EXPRESSION
        )
    })
}

/// Whether `??` can replace the `||` of `logical` without adding parentheses next to
/// another `&&` or `||`.
fn can_replace_operator(logical: &JsLogicalExpression) -> bool {
    logical.parent::<JsLogicalExpression>().is_none()
        && [logical.left(), logical.right()].into_iter().all(|operand| {
            !operand.is_ok_and(|operand| {
                operand.as_js_logical_expression().is_some_and(|operand| {
                    matches!(
                        operand.operator(),
                        Ok(JsLogicalOperator::LogicalAnd | JsLogicalOperator::LogicalOr)
                    )
                })
            })
        })
}

/// Wraps the expression in parentheses if it would be invalid or change meaning
/// as an operand of `??`. This covers two cases:
/// - Expressions with lower precedence than `??` (conditional, assignment, yield, comma)
/// - `||` and `&&` expressions, which cannot be mixed with `??` without parentheses
fn maybe_parenthesize_for_nullish(expr: AnyJsExpression) -> AnyJsExpression {
    if expr
        .precedence()
        .is_ok_and(|p| p < OperatorPrecedence::Coalesce)
    {
        return make::parenthesized(expr).into();
    }
    if let Some(logical) = expr.as_js_logical_expression()
        && logical.operator().is_ok_and(|op| {
            matches!(
                op,
                JsLogicalOperator::LogicalOr | JsLogicalOperator::LogicalAnd
            )
        })
    {
        return make::parenthesized(expr).into();
    }
    expr
}
