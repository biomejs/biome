use crate::{
    JsRuleAction,
    services::typed::Typed,
    utils::same_reference::{is_equivalent_assignment, is_equivalent_reference, is_same_reference},
};
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
    JsDoWhileStatement, JsExpressionStatement, JsForStatement, JsIfStatement, JsLogicalExpression,
    JsLanguage, JsLogicalOperator, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, JsUnaryOperator, JsWhileStatement, OperatorPrecedence, T,
    static_value::StaticValue,
};
use biome_js_type_info::{IgnoredPrimitiveTypes, InferredType};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, SyntaxTriviaPiece, TextRange,
    TriviaPieceKind, declare_node_union,
    trim_leading_trivia_pieces,
};
use biome_rule_options::use_nullish_coalescing::UseNullishCoalescingOptions;

declare_lint_rule! {
    /// Enforce using the nullish coalescing operator (`??`) instead of logical or (`||`).
    ///
    /// `??` only checks for `null` and `undefined`, while `||` checks for any falsy value
    /// including `0`, `''`, and `false`. The rule reports `||` and `||=`, along with ternary
    /// expressions and `if` statements that pick a fallback for a missing value in a longer
    /// way than `??` and `??=`.
    ///
    /// A check for a falsy value (`||`, `||=`, `x ? x : y`, or `if (!x)`) is reported when
    /// type analysis shows that the value is possibly nullish or typed `any` or `unknown`.
    /// A comparison that catches both `null` and `undefined`, such as `x == null`, is
    /// reported whatever the type of the value. A strict comparison with only one of them
    /// is reported only when the value cannot hold the other one.
    ///
    /// For a check for a falsy value, Biome offers the fix only when `??` picks the same
    /// operand as `||` would, which rules out values typed `any` or `unknown`, and values
    /// that can be `0`, `''`, or `false`.
    ///
    /// A ternary expression or `if` statement is reported when the value it tests and the
    /// value it uses are the same variable or property. They match whichever way they are
    /// written, so `a?.b != null ? a.b : c` and `a['b'] != null ? a.b : c` are reported. A
    /// value read through a function call, such as `f() != null ? f() : c`, is not reported,
    /// because each call can return a different value.
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
    /// ```ts,expect_diagnostic,file=invalid-ternary-truthiness.ts
    /// declare const user: { name: string } | undefined;
    /// const value = user ? user : { name: 'guest' };
    /// ```
    ///
    /// An `if` statement that only assigns a fallback to the value its test checks is also
    /// reported, since it can be rewritten as `??=`.
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
    /// By default, Biome ignores `||`, `||=`, and ternary expressions such as `x ? x : y`
    /// whose value decides the test of an `if`, `while`, `for`, or `do-while` statement or of
    /// a ternary expression. The value may reach the test through `!`, a logical operator
    /// (`&&`, `||`, or `??`), the branches of a ternary expression, or the last expression of
    /// a comma-separated sequence. A value that reaches the test through a function call, a
    /// function body, or another operator is reported. Set this to `false` to also report the
    /// values that decide a test. Default: `true`.
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
    /// The same goes for a value that reaches the test through `!`.
    ///
    /// ```ts,file=valid-conditional-tests-negated.ts
    /// declare const cond: string | null;
    /// if (!(cond || 'fallback')) {}
    /// ```
    ///
    /// ### ignoreTernaryTests
    ///
    /// By default, Biome reports ternary expressions that `??` can replace, both those that
    /// compare a value with `null` or `undefined` and those that check whether the value is
    /// truthy, such as `x ? x : y`. Set this to `true` to ignore those ternary expressions.
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
    /// ```ts,use_options,file=valid-ternary-tests-truthiness.ts
    /// declare const user: { name: string } | undefined;
    /// const value = user ? user : { name: 'guest' };
    /// ```
    ///
    /// ### ignoreMixedLogicalExpressions
    ///
    /// By default, Biome reports `||` and `||=` even when they are combined with `&&`, as in
    /// `(a || b) && c` or `a || (b && c)`. Set this to `true` to ignore them. A `||` chain
    /// counts as combined with `&&` when one of its operands is a `&&` or the whole chain is
    /// an operand of `&&`. Default: `false`.
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
    /// `||` and `||=` that are not combined with `&&` are still reported.
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
    /// Only the inner `||=` below is combined with `&&`, so the outer one is reported.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-mixed-nested-assign.ts
    /// declare let outer: string | null;
    /// declare let inner: string | null;
    /// declare const a: string;
    /// declare const b: string;
    /// outer ||= (inner ||= a && b);
    /// ```
    ///
    /// #### Valid
    ///
    /// `||` and `||=` combined with `&&` are not reported.
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
    /// By default, Biome reports `||`, `||=`, and ternary expressions such as `x ? x : y`
    /// even when their value is the argument of a call to the global `Boolean()` function.
    /// Set this to `true` to ignore them there, where coalescing on falsy values is
    /// intentional. The value may reach the call through a logical operator (`&&`, `||`, or
    /// `??`), the branches of a ternary expression, or the last expression of a
    /// comma-separated sequence. A ternary expression that is itself the argument is still
    /// reported. Default: `false`.
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
    /// A `!` between the `||` and the call stops the value from counting as the argument.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-boolean-coercion-negated.ts
    /// declare const a: string | null;
    /// const r = Boolean(!(a || 'default'));
    /// ```
    ///
    /// A ternary expression that is the argument of `Boolean()` is still reported.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-boolean-coercion-ternary.ts
    /// declare const a: { name: string } | null;
    /// declare const b: { name: string };
    /// const r = Boolean(a ? a : b);
    /// ```
    ///
    /// #### Valid
    ///
    /// `||` and `||=` that are the argument of a `Boolean()` call are not reported.
    ///
    /// ```ts,use_options,file=valid-boolean-coercion.ts
    /// declare const a: string | null;
    /// declare const b: string;
    /// const r = Boolean(a || b);
    /// ```
    ///
    /// ### ignorePrimitives
    ///
    /// By default, Biome reports checks for a falsy value (`||`, `||=`, `x ? x : y`, and
    /// `if (!x)`) whatever primitive types the checked value includes. Set this to `true` to
    /// ignore those checks when the type includes a `string`, `number`, `boolean`, or
    /// `bigint`, or to an object that sets some of the keys `string`, `number`, `boolean`,
    /// and `bigint` to `true` to ignore only those kinds. A type is ignored when any part of
    /// it is an ignored kind, so `string | number | null` is ignored when `string` is
    /// ignored. When at least one kind is ignored, values typed `any` or `unknown` are
    /// ignored too. Comparisons with `null` or `undefined` are not affected by this option.
    /// Default: none.
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
    /// A comparison with `null` is reported even when its value is a `string`.
    ///
    /// ```ts,expect_diagnostic,use_options,file=invalid-primitives-comparison.ts
    /// declare const name: string | null;
    /// const value = name !== null ? name : 'default';
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
    /// A union that includes a `string` is not reported either.
    ///
    /// ```ts,use_options,file=valid-primitives-union.ts
    /// declare const id: string | number | null;
    /// const value = id || 'default';
    /// ```
    ///
    /// ### ignoreIfStatements
    ///
    /// By default, Biome reports an `if` statement without an `else` branch whose only
    /// statement assigns a fallback to the value its test checks, since it can be rewritten
    /// as `??=`. The test may compare the value with `null` or `undefined`, or check `!x`.
    /// Set this to `true` to ignore those statements. Default: `false`.
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
    /// An `if` statement that assigns a fallback to a missing value is not reported.
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
    }
}

declare_node_union! {
    pub UseNullishCoalescingQuery = JsLogicalExpression | JsAssignmentExpression | JsConditionalExpression | JsIfStatement
}

declare_node_union! {
    pub AnyJsLogicalOrLikeExpression = JsLogicalExpression | JsAssignmentExpression
}

declare_node_union! {
    pub AnyJsConditionalTestLike = JsConditionalExpression | JsDoWhileStatement | JsForStatement | JsIfStatement | JsWhileStatement
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
        /// The tested value as written in the test, which becomes the left operand of `??`.
        subject: AnyJsExpression,
        /// The branch taken when `subject` is missing, which becomes the right operand of `??`.
        fallback: AnyJsExpression,
        /// Whether the consequent is the branch taken when `subject` is present.
        is_positive: bool,
        is_truthiness_check: bool,
        can_fix: bool,
    },
    IfStatement {
        if_range: TextRange,
        /// The assigned variable or property, without parentheses.
        assignment_target: AnyJsAssignment,
        assignment_value: AnyJsExpression,
        is_truthiness_check: bool,
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
            UseNullishCoalescingState::Ternary {
                test_range,
                is_truthiness_check: true,
                ..
            } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *test_range,
                    markup! {
                        "Prefer "<Emphasis>"??"</Emphasis>" over a ternary expression checking for a falsy value."
                    },
                )
                .note(markup! {
                    <Emphasis>"??"</Emphasis>" only uses the fallback when the value is null or undefined."
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
            UseNullishCoalescingState::IfStatement {
                if_range,
                is_truthiness_check: true,
                ..
            } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *if_range,
                    markup! {
                        "Prefer "<Emphasis>"??="</Emphasis>" over an "<Emphasis>"if"</Emphasis>" statement that assigns when the value is falsy."
                    },
                )
                .note(markup! {
                    <Emphasis>"??="</Emphasis>" only assigns when the value is null or undefined."
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
                // The subject's own trivia belongs to the test, which the rewrite drops.
                // Comments before the present-value branch move to the subject instead.
                let question = ternary.question_mark_token().ok()?;
                let colon = ternary.colon_token().ok()?;
                let (present_branch, present_separator, fallback_separator) = if *is_positive {
                    (ternary.consequent().ok()?, question, colon)
                } else {
                    (ternary.alternate().ok()?, colon, question)
                };
                let subject = subject
                    .clone()
                    .trim_comments_and_trivia()?
                    .prepend_trivia_pieces(comments_before_branch(
                        &present_separator,
                        &present_branch,
                    )?)?;
                // The ternary's trailing trivia is re-attached to the whole expression below.
                let fallback = AnyJsExpression::cast(
                    fallback
                        .syntax()
                        .clone()
                        .with_trailing_trivia_pieces([])?,
                )?
                .prepend_trivia_pieces(trim_leading_trivia_pieces(
                        fallback_separator.trailing_trivia().pieces(),
                    ))?;

                let operator = if starts_with_whitespace(&fallback) {
                    make::token(T![??]).with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
                } else {
                    make::token_decorated_with_space(T![??])
                };
                let new_expr = make::js_logical_expression(
                    maybe_parenthesize_for_nullish(subject),
                    operator,
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
                    AnyJsAssignmentPattern::AnyJsAssignment(target),
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

/// Whether a value passes through an enclosing `!` unchanged for truthiness.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Negation {
    PassesThrough,
    Stops,
}

/// A ternary or `if` test that decides whether a value is missing.
enum NullishTest {
    /// `x`, where `x` is a variable or a property.
    Truthy(AnyJsExpression),
    /// `!x`, where `x` is a variable or a property.
    Falsy(AnyJsExpression),
    /// A comparison with `==`, `!=`, `===`, or `!==`, or two of them joined by `||` or `&&`.
    Comparison {
        /// One of `==`, `!=`, `===`, and `!==`. A compound mixing loose and strict
        /// comparisons uses the loose operator.
        operator: JsBinaryOperator,
        first: (AnyJsExpression, AnyJsExpression),
        second: Option<(AnyJsExpression, AnyJsExpression)>,
    },
}

impl NullishTest {
    /// Reads how `test` checks for a missing value, ignoring parentheses, or returns
    /// `None` for any other test.
    fn from_test(test: AnyJsExpression) -> Option<Self> {
        match test.omit_parentheses() {
            test if is_variable_or_property(&test) => Some(Self::Truthy(test)),
            AnyJsExpression::JsUnaryExpression(unary) => {
                if unary.operator().ok()? != JsUnaryOperator::LogicalNot {
                    return None;
                }
                let argument = unary.argument().ok()?.omit_parentheses();
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
                ) = (
                    logical.left().ok()?.omit_parentheses(),
                    logical.right().ok()?.omit_parentheses(),
                )
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
            Self::Truthy(_) => false,
            Self::Falsy(_) => true,
            Self::Comparison { operator, .. } => matches!(
                operator,
                JsBinaryOperator::Equality | JsBinaryOperator::StrictEquality
            ),
        }
    }

    /// Returns the tested value that `is_subject` accepts and how the test checks it, or
    /// `None` unless every comparison compares `null` or `undefined` with such a value.
    fn into_subject(
        self,
        ctx: &RuleContext<UseNullishCoalescing>,
        is_subject: impl Fn(&AnyJsExpression) -> bool,
    ) -> Option<NullishSubject> {
        let (operator, first, second) = match self {
            Self::Truthy(expression) | Self::Falsy(expression) => {
                return is_subject(&expression).then_some(NullishSubject {
                    expression,
                    check: NullishCheck::Truthiness,
                    operands_match: true,
                });
            }
            Self::Comparison {
                operator,
                first,
                second,
            } => (operator, first, second),
        };

        let (first_literal, expression) = nullish_comparison(ctx, first, &is_subject)?;
        let (second_literal, operands_match) = match second {
            Some(second) => {
                let (literal, operand) = nullish_comparison(ctx, second, &is_subject)?;
                let operands_match =
                    is_same_reference(expression.clone(), operand) == Some(true);
                (Some(literal), operands_match)
            }
            None => (None, true),
        };

        if matches!(
            operator,
            JsBinaryOperator::Equality | JsBinaryOperator::Inequality
        ) {
            return Some(NullishSubject {
                expression,
                check: NullishCheck::Nullish,
                operands_match,
            });
        }

        let check = match (first_literal, second_literal) {
            (NullishLiteral::Null, Some(NullishLiteral::Undefined))
            | (NullishLiteral::Undefined, Some(NullishLiteral::Null)) => NullishCheck::Nullish,
            (literal, _) => NullishCheck::Strict(literal),
        };
        Some(NullishSubject {
            expression,
            check,
            operands_match,
        })
    }
}

/// The value that a [`NullishTest`] checks, and how it checks it.
struct NullishSubject {
    expression: AnyJsExpression,
    check: NullishCheck,
    /// Whether both comparisons of a compound read the value with optional chaining in the
    /// same places, which is always the case for a single check. Optional chaining in only
    /// one of them means that the other comparison can throw where `??` takes the fallback.
    operands_match: bool,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum NullishCheck {
    /// `x` or `!x`, which also treat `0`, `''`, and `false` as missing.
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
    /// A local `undefined` variable is not the global one, so it is not a nullish literal.
    fn from_expression(
        ctx: &RuleContext<UseNullishCoalescing>,
        expression: &AnyJsExpression,
    ) -> Option<Self> {
        let expression = expression.clone().omit_parentheses();
        match expression.as_static_value()? {
            StaticValue::Null(_) => Some(Self::Null),
            StaticValue::Undefined(_) => expression
                .as_js_identifier_expression()
                .and_then(|identifier| identifier.name().ok())
                .is_none_or(|reference| !ctx.has_binding(&reference))
                .then_some(Self::Undefined),
            _ => None,
        }
    }

    /// Whether a strict comparison with this literal catches every nullish variant of
    /// `ty`, or `None` when the type cannot be resolved.
    fn covers_nullish_variants_of(self, ty: InferredType) -> Option<bool> {
        if ty.includes_any_or_unknown_keyword() {
            return Some(false);
        }
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

    if is_ignored_truthiness_context(ctx, &AnyJsExpression::from(logical.clone())) {
        return None;
    }

    let options = ctx.options();
    if options.ignore_mixed_logical_expressions()
        && is_in_mixed_logical_tree(&AnyJsLogicalOrLikeExpression::from(logical.clone()))
    {
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

    if is_ignored_truthiness_context(ctx, &AnyJsExpression::from(assignment.clone())) {
        return None;
    }

    let options = ctx.options();
    if options.ignore_mixed_logical_expressions()
        && is_in_mixed_logical_tree(&AnyJsLogicalOrLikeExpression::from(assignment.clone()))
    {
        return None;
    }

    let left_ty = ctx.type_of_assignment(assignment.left().ok()?.as_any_js_assignment()?)?;
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
    let consequent = ternary.consequent().ok()?;
    let alternate = ternary.alternate().ok()?;
    let nullish_test = NullishTest::from_test(test.clone())?;
    let is_positive = !nullish_test.is_true_when_missing();
    let (present_branch, fallback) = if is_positive {
        (consequent.clone(), alternate)
    } else {
        (alternate, consequent.clone())
    };

    // Calls and updates never match, so evaluating the subject once in `??` only drops
    // a repeated read.
    let subject = nullish_test.into_subject(ctx, |operand| {
        is_equivalent_reference(operand.clone(), present_branch.clone()) == Some(true)
    })?;

    let is_truthiness_check = subject.check == NullishCheck::Truthiness;
    if is_truthiness_check
        && is_ignored_truthiness_context(ctx, &AnyJsExpression::from(ternary.clone()))
    {
        return None;
    }

    // The fix keeps comments before either branch and around the ternary; comments
    // elsewhere have no place in `subject ?? fallback`.
    let drops_comments = test.syntax().has_inner_comments()
        || test.syntax().has_trailing_comments()
        || present_branch.syntax().has_inner_comments()
        || consequent.syntax().has_trailing_comments()
        || ternary.question_mark_token().ok()?.has_leading_comments()
        || ternary.colon_token().ok()?.has_leading_comments();
    let present_separator = if is_positive {
        ternary.question_mark_token().ok()?
    } else {
        ternary.colon_token().ok()?
    };
    let moves_line_break = comments_before_branch(&present_separator, &present_branch).is_none();
    // The subject and the present branch match even when only one of them has a type
    // wrapper, but the fix keeps only the subject, which could change the type of the
    // expression.
    let crosses_type_wrapper =
        contains_type_wrapper(&subject.expression) || contains_type_wrapper(&present_branch);
    let joins_previous_statement = starts_expression_statement(ternary)
        && joins_previous_statement(ternary.syntax(), subject.expression.syntax());
    let can_fix = report_check(ctx, &subject)?
        && !drops_comments
        && !crosses_type_wrapper
        && !moves_line_break
        && !joins_previous_statement;

    Some(UseNullishCoalescingState::Ternary {
        test_range: test.syntax().text_trimmed_range(),
        subject: subject.expression,
        fallback,
        is_positive,
        is_truthiness_check,
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
    let left = assignment.left().ok()?;
    let target = left.as_any_js_assignment()?;

    let nullish_test = NullishTest::from_test(if_stmt.test().ok()?)?;
    if !nullish_test.is_true_when_missing() {
        return None;
    }
    let subject = nullish_test.into_subject(ctx, |operand| {
        is_equivalent_assignment(target.clone(), operand.clone()) == Some(true)
    })?;

    let target = target.clone().omit_parentheses();
    // Inner comments would be discarded by the rewrite.
    let can_fix = report_check(ctx, &subject)?
        && !if_stmt.syntax().has_inner_comments()
        && !joins_previous_statement(if_stmt.syntax(), target.syntax());

    Some(UseNullishCoalescingState::IfStatement {
        if_range: if_stmt.syntax().text_trimmed_range(),
        assignment_target: target,
        assignment_value: assignment.right().ok()?,
        is_truthiness_check: subject.check == NullishCheck::Truthiness,
        can_fix,
    })
}

/// Returns the assignment with `=`, `??=`, or `||=` that is the only statement of an
/// `if` body, either `if (test) { target = value; }` or `if (test) target = value;`.
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
        .omit_parentheses()
        .as_js_assignment_expression()?
        .clone();

    // Other compound assignments, such as `+=`, combine the old value instead of
    // replacing a missing one.
    matches!(
        assignment.operator().ok()?,
        JsAssignmentOperator::Assign
            | JsAssignmentOperator::NullishCoalescingAssign
            | JsAssignmentOperator::LogicalOrAssign
    )
    .then_some(assignment)
}

/// Returns `None` when the check of `subject` is not reported, otherwise whether the
/// `??` or `??=` rewrite keeps its behavior.
fn report_check(ctx: &RuleContext<UseNullishCoalescing>, subject: &NullishSubject) -> Option<bool> {
    match subject.check {
        NullishCheck::Nullish => Some(subject.operands_match),
        NullishCheck::Strict(literal) => literal
            .covers_nullish_variants_of(ctx.type_of_expression(&subject.expression)?)?
            .then_some(subject.operands_match),
        NullishCheck::Truthiness => {
            report_truthiness(ctx.options(), ctx.type_of_expression(&subject.expression)?)
        }
    }
}

/// Returns `None` when a truthiness check of a value of type `ty` is not reported,
/// otherwise whether `??` keeps its behavior.
fn report_truthiness(options: &UseNullishCoalescingOptions, ty: InferredType) -> Option<bool> {
    let ignores_primitives = options.ignores_any_primitive();
    if ty.includes_any_or_unknown_keyword() {
        return (!ignores_primitives).then_some(false);
    }
    if !ty.has_nullish_variant()? {
        return None;
    }
    if ignores_primitives
        && ty.has_ignored_primitive_variant(IgnoredPrimitiveTypes {
            string: options.should_ignore_primitive_string(),
            number: options.should_ignore_primitive_number(),
            boolean: options.should_ignore_primitive_boolean(),
            bigint: options.should_ignore_primitive_bigint(),
        })?
    {
        return None;
    }
    ty.is_safe_for_nullish_coalescing()
}

/// Whether the options ignore a truthiness check by `expression` because of how its
/// result is used: to decide a conditional test, or as the argument of `Boolean()`.
fn is_ignored_truthiness_context(
    ctx: &RuleContext<UseNullishCoalescing>,
    expression: &AnyJsExpression,
) -> bool {
    let options = ctx.options();
    (options.ignore_conditional_tests() && is_conditional_test(expression))
        || (options.ignore_boolean_coercion() && is_boolean_coercion(ctx, expression))
}

/// Returns the outermost expression with the truthiness of `expression`, reached through
/// parentheses, logical operators, ternary branches, the last expression of a sequence,
/// and `!` when `negation` lets it pass.
fn truthiness_root(expression: &AnyJsExpression, negation: Negation) -> AnyJsExpression {
    let mut current = expression.clone();
    while let Some(parent) = current.parent::<AnyJsExpression>() {
        let passes_truthiness = match &parent {
            AnyJsExpression::JsParenthesizedExpression(_)
            | AnyJsExpression::JsLogicalExpression(_) => true,
            AnyJsExpression::JsConditionalExpression(ternary) => ternary
                .test()
                .ok()
                .is_none_or(|test| test.syntax() != current.syntax()),
            AnyJsExpression::JsSequenceExpression(sequence) => sequence
                .right()
                .is_ok_and(|right| right.syntax() == current.syntax()),
            AnyJsExpression::JsUnaryExpression(unary) => {
                negation == Negation::PassesThrough
                    && unary.operator().ok() == Some(JsUnaryOperator::LogicalNot)
            }
            _ => false,
        };
        if !passes_truthiness {
            break;
        }
        current = parent;
    }
    current
}

/// Whether the truthiness of `expression` decides the test of an `if`, `while`,
/// `do-while`, `for`, or ternary expression.
fn is_conditional_test(expression: &AnyJsExpression) -> bool {
    let value = truthiness_root(expression, Negation::PassesThrough);
    value
        .parent::<AnyJsConditionalTestLike>()
        .and_then(|consumer| consumer.test())
        .is_some_and(|test| test.syntax() == value.syntax())
}

/// Whether the truthiness of `expression` is the first argument of a call to the global
/// `Boolean` function, the only one it converts, unless `expression` is a ternary passed as
/// the argument itself. A `Boolean` bound in scope is a different function.
fn is_boolean_coercion(
    ctx: &RuleContext<UseNullishCoalescing>,
    expression: &AnyJsExpression,
) -> bool {
    let argument = truthiness_root(expression, Negation::Stops);
    if matches!(expression, AnyJsExpression::JsConditionalExpression(_))
        && argument.clone().omit_parentheses().syntax() == expression.syntax()
    {
        return false;
    }
    argument
        .parent::<JsCallArgumentList>()
        .filter(|arguments| {
            arguments
                .iter()
                .next()
                .is_some_and(|first| first.is_ok_and(|first| first.syntax() == argument.syntax()))
        })
        .and_then(|arguments| arguments.parent::<JsCallArguments>())
        .and_then(|arguments| arguments.parent::<JsCallExpression>())
        .and_then(|call| call.callee().ok())
        .and_then(|callee| callee.as_js_reference_identifier())
        .is_some_and(|reference| reference.has_name("Boolean") && !ctx.has_binding(&reference))
}

impl AnyJsConditionalTestLike {
    fn test(&self) -> Option<AnyJsExpression> {
        match self {
            Self::JsConditionalExpression(node) => node.test().ok(),
            Self::JsDoWhileStatement(node) => node.test().ok(),
            Self::JsForStatement(node) => node.test(),
            Self::JsIfStatement(node) => node.test().ok(),
            Self::JsWhileStatement(node) => node.test().ok(),
        }
    }
}

/// Whether `node` is combined with `&&`: a `&&` is an operand of `node` or of a `||`
/// chained with it, or the outermost `||` of the chain is an operand of `&&`.
fn is_in_mixed_logical_tree(node: &AnyJsLogicalOrLikeExpression) -> bool {
    let mut root = node.clone();
    while let Some(parent) = parent_logical(&root)
        .filter(|parent| parent.operator().ok() == Some(JsLogicalOperator::LogicalOr))
    {
        root = parent.into();
    }
    parent_logical(&root)
        .is_some_and(|parent| parent.operator().ok() == Some(JsLogicalOperator::LogicalAnd))
        || chain_contains_logical_and(&root)
        || (root != *node && chain_contains_logical_and(node))
}

/// Returns the logical expression that `node` is an operand of, ignoring parentheses.
fn parent_logical(node: &AnyJsLogicalOrLikeExpression) -> Option<JsLogicalExpression> {
    let mut parent = node.parent::<AnyJsExpression>()?;
    while let AnyJsExpression::JsParenthesizedExpression(parenthesized) = parent {
        parent = parenthesized.parent::<AnyJsExpression>()?;
    }
    parent.as_js_logical_expression().cloned()
}

/// Whether a `&&` is an operand of `node` or of a `||` reached from `node` through `||`
/// operands, where only the right-hand side of a `||=` counts as an operand.
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
        let Ok(AnyJsExpression::JsLogicalExpression(logical)) =
            operand.map(AnyJsExpression::omit_parentheses)
        else {
            continue;
        };
        match logical.operator() {
            Ok(JsLogicalOperator::LogicalAnd) => return true,
            Ok(JsLogicalOperator::LogicalOr) => pending.extend([logical.left(), logical.right()]),
            _ => {}
        }
    }
    false
}

/// Splits a comparison into the nullish literal it compares with and the operand that
/// `is_subject` accepts.
fn nullish_comparison(
    ctx: &RuleContext<UseNullishCoalescing>,
    (left, right): (AnyJsExpression, AnyJsExpression),
    is_subject: &impl Fn(&AnyJsExpression) -> bool,
) -> Option<(NullishLiteral, AnyJsExpression)> {
    let (literal, operand) = match (
        NullishLiteral::from_expression(ctx, &left),
        NullishLiteral::from_expression(ctx, &right),
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
            JsLogicalOperator::LogicalOr,
            JsBinaryOperator::StrictEquality | JsBinaryOperator::Equality,
            JsBinaryOperator::StrictEquality | JsBinaryOperator::Equality,
        ) => Some(JsBinaryOperator::Equality),
        (
            JsLogicalOperator::LogicalAnd,
            JsBinaryOperator::StrictInequality,
            JsBinaryOperator::StrictInequality,
        ) => Some(JsBinaryOperator::StrictInequality),
        (
            JsLogicalOperator::LogicalAnd,
            JsBinaryOperator::StrictInequality | JsBinaryOperator::Inequality,
            JsBinaryOperator::StrictInequality | JsBinaryOperator::Inequality,
        ) => Some(JsBinaryOperator::Inequality),
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

/// Whether `ternary` is the first thing in an expression statement.
fn starts_expression_statement(ternary: &JsConditionalExpression) -> bool {
    let start = ternary.syntax().text_trimmed_range().start();
    ternary
        .syntax()
        .ancestors()
        .take_while(|ancestor| ancestor.text_trimmed_range().start() == start)
        .any(|ancestor| JsExpressionStatement::can_cast(ancestor.kind()))
}

/// Whether `replacement`, put where the statement starting with `original` begins,
/// would continue the previous statement: a line starting with `(` continues one that
/// doesn't end with `;` or `{`. The tested and assigned values never start with `[` or a
/// template literal, which would need the same care.
fn joins_previous_statement(original: &JsSyntaxNode, replacement: &JsSyntaxNode) -> bool {
    replacement
        .first_token()
        .is_some_and(|token| token.kind() == T!['('])
        && original
            .first_token()
            .and_then(|token| token.prev_token())
            .is_some_and(|previous| !matches!(previous.kind(), T![;] | T!['{']))
}

/// Whether `expression` contains TypeScript syntax that only affects types, such as
/// `x as T` or `x!`.
fn contains_type_wrapper(expression: &AnyJsExpression) -> bool {
    expression.syntax().descendants().any(|node| {
        matches!(
            node.kind(),
            JsSyntaxKind::TS_AS_EXPRESSION
                | JsSyntaxKind::TS_SATISFIES_EXPRESSION
                | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
                | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
                | JsSyntaxKind::TS_INSTANTIATION_EXPRESSION
        )
    })
}

/// Returns the comments between `separator`, the `?` or `:` before `branch`, and `branch`,
/// each followed by a space when the original has one. The line breaks around them are
/// dropped, since a line break after `return` or `throw` changes the meaning of the
/// statement.
///
/// Returns `None` for a comment that needs a line break after it: a `//` comment or a
/// comment that spans several lines.
fn comments_before_branch(
    separator: &JsSyntaxToken,
    branch: &AnyJsExpression,
) -> Option<Vec<SyntaxTriviaPiece<JsLanguage>>> {
    let pieces: Vec<_> = separator
        .trailing_trivia()
        .pieces()
        .chain(branch.syntax().first_leading_trivia()?.pieces())
        .collect();
    let space = pieces.iter().find(|piece| piece.is_whitespace());
    let mut kept = Vec::new();
    for comment in pieces.iter().filter(|piece| piece.is_comments()) {
        if comment.text().starts_with("//") || comment.text().contains(['\n', '\r']) {
            return None;
        }
        kept.push(comment.clone());
        kept.extend(space.cloned());
    }
    Some(kept)
}

/// Whether `expression` starts with whitespace or a line break, which already separates it
/// from a preceding operator.
fn starts_with_whitespace(expression: &AnyJsExpression) -> bool {
    expression
        .syntax()
        .first_leading_trivia()
        .and_then(|trivia| trivia.pieces().next())
        .is_some_and(|piece| piece.is_whitespace() || piece.is_newline())
}

/// Parenthesizes `expr` when it would be invalid or change meaning as an operand of `??`:
/// lower precedence, `||` or `&&`, or an arrow function, whose body would absorb the rest
/// of the `??` expression.
fn maybe_parenthesize_for_nullish(expr: AnyJsExpression) -> AnyJsExpression {
    if expr
        .precedence()
        .is_ok_and(|p| p < OperatorPrecedence::Coalesce)
    {
        return make::parenthesized(expr).into();
    }
    let is_or_and = expr.as_js_logical_expression().is_some_and(|logical| {
        logical.operator().is_ok_and(|op| {
            matches!(
                op,
                JsLogicalOperator::LogicalOr | JsLogicalOperator::LogicalAnd
            )
        })
    });
    if is_or_and || expr.as_js_arrow_function_expression().is_some() {
        return make::parenthesized(expr).into();
    }
    expr
}
