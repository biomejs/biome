use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsAssignmentExpression, JsAwaitExpression, JsBinaryOperator,
    JsCallExpression, JsComputedMemberExpression, JsConditionalExpression,
    JsExpressionStatement, JsForStatement, JsImportCallExpression, JsInExpression,
    JsInstanceofExpression, JsStaticMemberExpression,
    JsLanguage, JsLogicalOperator, JsNewExpression, JsPostUpdateExpression,
    JsPreUpdateExpression, JsSpread, JsSyntaxKind,
    JsTemplateExpression, JsUnaryExpression, JsUnaryOperator, JsYieldExpression,
    OperatorPrecedence, T, inner_string_text,
    numbers::{canonicalize_js_bigint_literal, parse_js_number_with_single_rounding},
    static_value::StaticValue,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxKindSet, Text, WalkEvent};
use biome_rule_options::no_useless_nested_ternary::NoUselessNestedTernaryOptions;
use smallvec::{SmallVec, smallvec};

use crate::{
    JsRuleAction,
    utils::{
        is_node_equal,
        same_reference::{is_same_reference, string_value},
    },
};

declare_lint_rule! {
    /// Disallow nested ternary expressions that can be merged into one.
    ///
    /// A ternary expression (`condition ? a : b`) placed inside another one is unnecessary
    /// when both of them share a result.
    /// The two conditions can then be combined with `&&` or `||`, so that one ternary
    /// expression is enough.
    /// This makes the code shorter and easier to read.
    ///
    /// The rule also checks chains of ternary expressions whose conditions compare the same
    /// value with `===` against different literals, such as `x === 1 ? a : x === 2 ? b : c`.
    /// At most one of these conditions is true at a time.
    /// A condition can then be combined with a later condition that has the same result,
    /// or removed when its result is also the last result of the chain.
    ///
    /// The rule doesn't report code where the simplification would change whether or when
    /// a function call or an assignment in a condition runs.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const result = a ? (b ? c : d) : d;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const result = a ? d : (b ? d : c);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const result = a ? (b ? d : c) : d;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const result = x === 1 ? a : (x === 2 ? b : (x === 3 ? a : c));
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const combined = a && b ? c : d;
    /// const threeResults = a ? (b ? c : d) : e;
    /// const chain = x === 1 || x === 3 ? a : (x === 2 ? b : c);
    /// ```
    ///
    /// ## See Also
    ///
    /// - To disallow all nested ternary expressions, see [`noNestedTernary`](https://biomejs.dev/linter/rules/no-nested-ternary/).
    ///
    pub NoUselessNestedTernary {
        version: "next",
        name: "noUselessNestedTernary",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-unnecessary-nested-ternary").inspired()],
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessNestedTernary {
    type Query = Ast<JsConditionalExpression>;
    type State = UselessNestedTernary;
    type Signals = Option<Self::State>;
    type Options = NoUselessNestedTernaryOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let consequent = node.consequent().ok()?.omit_parentheses();
        let alternate = node.alternate().ok()?.omit_parentheses();
        let nested_consequent = consequent.as_js_conditional_expression();
        let nested_alternate = alternate.as_js_conditional_expression();

        if let Some(nested) = nested_consequent
            && is_same_expression(&nested.alternate().ok()?, &alternate)
        {
            return Some(UselessNestedTernary::InConsequent { is_negated: false });
        }
        if let Some(nested) = nested_alternate
            && is_same_expression(&consequent, &nested.consequent().ok()?)
        {
            return Some(UselessNestedTernary::InAlternate { is_negated: false });
        }
        if let Some(state) = find_useless_branch(node, &consequent) {
            return Some(state);
        }
        if let Some(nested) = nested_consequent
            && is_same_expression(&nested.consequent().ok()?, &alternate)
        {
            return Some(UselessNestedTernary::InConsequent { is_negated: true });
        }
        if let Some(nested) = nested_alternate
            && is_same_expression(&consequent, &nested.alternate().ok()?)
        {
            return Some(UselessNestedTernary::InAlternate { is_negated: true });
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                match state {
                    UselessNestedTernary::InConsequent { .. }
                    | UselessNestedTernary::InAlternate { .. } => markup! {
                        "This nested ternary expression is unnecessary."
                    },
                    UselessNestedTernary::LaterBranch(_) => markup! {
                        "The first condition of this chain of ternary expressions has the same result as a later condition."
                    },
                    UselessNestedTernary::FirstBranch => markup! {
                        "The first condition of this chain of ternary expressions is unnecessary."
                    },
                },
            )
            .note(match state {
                UselessNestedTernary::InConsequent { .. } => markup! {
                    "The nested expression repeats a result of the expression around it. Combining both conditions with "<Emphasis>"&&"</Emphasis>" gives the same results without the nesting."
                },
                UselessNestedTernary::InAlternate { .. } => markup! {
                    "The nested expression repeats a result of the expression around it. Combining both conditions with "<Emphasis>"||"</Emphasis>" gives the same results without the nesting."
                },
                UselessNestedTernary::LaterBranch(_) => markup! {
                    "Combining both conditions with "<Emphasis>"||"</Emphasis>" gives the same results with one ternary expression less."
                },
                UselessNestedTernary::FirstBranch => markup! {
                    "The first condition leads to the same result as when no condition of the chain is true. Removing it gives the same results with one ternary expression less."
                },
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if is_in_for_initializer_with_in_operator(node) {
            return None;
        }
        let test = node.test().ok()?;
        let question_mark = make::token_decorated_with_space(T![?]);
        let colon = make::token_decorated_with_space(T![:]);
        // Each pair is an expression to replace and its replacement. The first pair starts at the
        // first token of `node`.
        let replacements: SmallVec<[(AnyJsExpression, AnyJsExpression); 2]> = match state {
            UselessNestedTernary::InConsequent { is_negated } => {
                let nested = node.consequent().ok()?.omit_parentheses();
                let nested = nested.as_js_conditional_expression()?;
                let result = if *is_negated {
                    nested.alternate()
                } else {
                    nested.consequent()
                };
                let merged = make::js_conditional_expression(
                    combine_conditions(
                        test,
                        JsLogicalOperator::LogicalAnd,
                        nested.test().ok()?,
                        *is_negated,
                    )?,
                    question_mark,
                    result.ok()?.trim_comments_and_trivia()?,
                    colon,
                    node.alternate().ok()?.trim_comments_and_trivia()?,
                );
                smallvec![(node.clone().into(), merged.into())]
            }
            UselessNestedTernary::InAlternate { is_negated } => {
                let nested = node.alternate().ok()?.omit_parentheses();
                let nested = nested.as_js_conditional_expression()?;
                let result = if *is_negated {
                    nested.consequent()
                } else {
                    nested.alternate()
                };
                let merged = make::js_conditional_expression(
                    combine_conditions(
                        test,
                        JsLogicalOperator::LogicalOr,
                        nested.test().ok()?,
                        *is_negated,
                    )?,
                    question_mark,
                    node.consequent().ok()?.trim_comments_and_trivia()?,
                    colon,
                    result.ok()?.trim_comments_and_trivia()?,
                );
                smallvec![(node.clone().into(), merged.into())]
            }
            UselessNestedTernary::LaterBranch(later_branch) => {
                let branch = later_branch.clone().omit_parentheses();
                let branch = branch.as_js_conditional_expression()?;
                let condition = combine_conditions(
                    test.clone(),
                    JsLogicalOperator::LogicalOr,
                    branch.test().ok()?,
                    false,
                )?;
                smallvec![
                    (test, condition),
                    (
                        later_branch.clone(),
                        branch.alternate().ok()?.trim_comments_and_trivia()?
                    ),
                ]
            }
            UselessNestedTernary::FirstBranch => smallvec![(
                node.clone().into(),
                node.alternate()
                    .ok()?
                    .omit_parentheses()
                    .trim_comments_and_trivia()?
            )],
        };

        let (first_original, first_replacement) = replacements.first()?;
        if may_join_previous_statement(first_original, first_replacement) {
            return None;
        }
        // The replacements are built without the comments around the parts they keep and without
        // the parts they drop, so a lower count means that the fix would delete a comment.
        let (original_comments, replacement_comments) = replacements.iter().fold(
            (0, 0),
            |(original_comments, replacement_comments), (original, replacement)| {
                (
                    original_comments + count_inner_comments(original),
                    replacement_comments + count_inner_comments(replacement),
                )
            },
        );
        if original_comments != replacement_comments {
            return None;
        }

        let mut mutation = ctx.root().begin();
        for (original, replacement) in replacements {
            mutation.replace_node(original, replacement);
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            match state {
                UselessNestedTernary::InConsequent { .. } => markup! {
                    "Combine the conditions with "<Emphasis>"&&"</Emphasis>"."
                },
                UselessNestedTernary::InAlternate { .. }
                | UselessNestedTernary::LaterBranch(_) => markup! {
                    "Combine the conditions with "<Emphasis>"||"</Emphasis>"."
                },
                UselessNestedTernary::FirstBranch => markup! {
                    "Remove the first condition."
                },
            },
            mutation,
        ))
    }
}

/// How a ternary expression that contains another one can be simplified.
pub enum UselessNestedTernary {
    /// The ternary nested in the consequent shares a result with the outer alternate.
    ///
    /// `a ? (b ? c : d) : d` becomes `a && b ? c : d`. When `is_negated` is set, the shared
    /// result is the consequent of the nested ternary, so `a ? (b ? d : c) : d` becomes
    /// `a && !b ? c : d`.
    ///
    /// In a chain such as `x === 1 ? (b ? c : d) : (x === 2 ? e : d)`, the shared result is the
    /// last alternate of the chain instead, and the outer alternate is kept as it is.
    InConsequent { is_negated: bool },
    /// The ternary nested in the alternate shares a result with the outer consequent.
    ///
    /// `a ? d : (b ? d : c)` becomes `a || b ? d : c`. When `is_negated` is set, the shared
    /// result is the alternate of the nested ternary, so `a ? c : (b ? d : c)` becomes
    /// `a || !b ? c : d`.
    InAlternate { is_negated: bool },
    /// A branch that isn't directly nested in the first one has the same result as the first
    /// branch, and its condition is never true together with the conditions before it.
    ///
    /// `x === 1 ? a : (x === 2 ? b : (x === 3 ? a : c))` becomes
    /// `x === 1 || x === 3 ? a : (x === 2 ? b : c)`. The value is the later branch with the
    /// parentheses around it, here `(x === 3 ? a : c)`.
    LaterBranch(AnyJsExpression),
    /// The first branch has the same result as the last alternate of the chain, and its
    /// condition is never true together with the other conditions.
    ///
    /// `x === 1 ? c : (x === 2 ? b : c)` becomes `x === 2 ? b : c`.
    FirstBranch,
}

/// Looks for a simplification of the chain of ternaries that starts at `node` and continues in
/// its alternate. The simplification relies on conditions of the chain that can't be true at
/// the same time.
///
/// `consequent` is the consequent of `node`, without parentheses.
fn find_useless_branch(
    node: &JsConditionalExpression,
    consequent: &AnyJsExpression,
) -> Option<UselessNestedTernary> {
    // The tests of the branches after the first one.
    let mut later_tests: SmallVec<[Option<LiteralTest>; 4]> = SmallVec::new();
    let mut alternate = node.alternate().ok()?;
    let last_alternate = loop {
        let AnyJsExpression::JsConditionalExpression(branch) = alternate.clone().omit_parentheses()
        else {
            break alternate.omit_parentheses();
        };
        let test = LiteralTest::from_condition(&branch.test().ok()?);
        // Moving the condition of a branch to the front of the chain gives the same results only
        // if the conditions that are skipped this way are false whenever this one is true.
        if !later_tests.is_empty()
            && let Some(test) = &test
            && later_tests.iter().all(|skipped| {
                skipped
                    .as_ref()
                    .is_some_and(|skipped| test.excludes(skipped))
            })
            && is_same_expression(consequent, &branch.consequent().ok()?)
        {
            return Some(UselessNestedTernary::LaterBranch(alternate));
        }
        later_tests.push(test);
        alternate = branch.alternate().ok()?;
    };
    if later_tests.is_empty() {
        return None;
    }

    // When the first condition is true, all the others are false. Evaluating the alternate of
    // `node` then gives the last alternate of the chain.
    let first_test = LiteralTest::from_condition(&node.test().ok()?)?;
    if !later_tests
        .iter()
        .all(|later| later.as_ref().is_some_and(|later| first_test.excludes(later)))
    {
        return None;
    }
    if is_same_expression(consequent, &last_alternate) {
        return Some(UselessNestedTernary::FirstBranch);
    }
    let nested = consequent.as_js_conditional_expression()?;
    // The nested test would run before the later conditions, which it must not influence.
    if may_have_side_effects(&nested.test().ok()?) {
        return None;
    }
    if is_same_expression(&nested.consequent().ok()?, &last_alternate) {
        Some(UselessNestedTernary::InConsequent { is_negated: true })
    } else if is_same_expression(&nested.alternate().ok()?, &last_alternate) {
        Some(UselessNestedTernary::InConsequent { is_negated: false })
    } else {
        None
    }
}

/// What a condition without side effects says about a value when the condition is true: the
/// value is one of the listed primitives.
///
/// For example, `x === 1 || x === 2` says that `x` is `1` or `2`.
struct LiteralTest {
    /// The expression that is compared, such as `x` in `x === 1` and in `typeof x === "string"`.
    subject: AnyJsExpression,
    /// Whether the comparison is about the result of `typeof subject` instead of `subject`.
    is_typeof: bool,
    values: SmallVec<[Primitive; 2]>,
}

impl LiteralTest {
    /// Returns what `condition` says when it is true, or `None` if it has side effects or isn't
    /// built from `===` comparisons against literals.
    ///
    /// All operands of `&&` are true when the condition is true, so the first operand that
    /// makes such comparisons is enough: `enabled && x === 1` says that `x` is `1`. The
    /// operands before it must not be able to throw, so `a.b && x === 1` returns `None`.
    fn from_condition(condition: &AnyJsExpression) -> Option<Self> {
        let mut operands: SmallVec<[_; 4]> = smallvec![condition.clone()];
        while let Some(operand) = operands.pop() {
            match operand.omit_parentheses() {
                AnyJsExpression::JsLogicalExpression(logical)
                    if logical.operator().ok()? == JsLogicalOperator::LogicalAnd =>
                {
                    operands.push(logical.right().ok()?);
                    operands.push(logical.left().ok()?);
                }
                operand => {
                    if let Some(test) = Self::from_comparisons(operand.clone()) {
                        return (!may_have_side_effects(condition)).then_some(test);
                    }
                    // The simplifications evaluate the condition in cases where another
                    // condition used to stop the evaluation before. This operand runs before the
                    // comparisons, so nothing guards it against values that make it throw.
                    if operand
                        .syntax()
                        .preorder_of_kinds(THROWING_KINDS)
                        .next()
                        .is_some()
                    {
                        return None;
                    }
                }
            }
        }
        None
    }

    /// Returns what `expression` says when it is true, if it is a `===` comparison against a
    /// literal, or several of them about the same value joined by `||`.
    fn from_comparisons(expression: AnyJsExpression) -> Option<Self> {
        let mut test: Option<Self> = None;
        let mut comparisons: SmallVec<[_; 4]> = smallvec![expression];
        while let Some(comparison) = comparisons.pop() {
            match comparison.omit_parentheses() {
                AnyJsExpression::JsLogicalExpression(logical)
                    if logical.operator().ok()? == JsLogicalOperator::LogicalOr =>
                {
                    comparisons.push(logical.right().ok()?);
                    comparisons.push(logical.left().ok()?);
                }
                AnyJsExpression::JsBinaryExpression(binary)
                    if binary.operator().ok()? == JsBinaryOperator::StrictEquality =>
                {
                    let left = binary.left().ok()?;
                    let right = binary.right().ok()?;
                    let (subject, value) = match (
                        Primitive::from_expression(&left),
                        Primitive::from_expression(&right),
                    ) {
                        (None, Some(value)) => (left, value),
                        (Some(value), None) => (right, value),
                        _ => return None,
                    };
                    let (subject, is_typeof) = match subject.omit_parentheses() {
                        AnyJsExpression::JsUnaryExpression(unary)
                            if unary.operator().ok()? == JsUnaryOperator::Typeof =>
                        {
                            (unary.argument().ok()?, true)
                        }
                        subject => (subject, false),
                    };
                    match &mut test {
                        Some(test) => {
                            if !test.has_subject(&subject, is_typeof) {
                                return None;
                            }
                            test.values.push(value);
                        }
                        None => {
                            test = Some(Self {
                                subject,
                                is_typeof,
                                values: smallvec![value],
                            });
                        }
                    }
                }
                _ => return None,
            }
        }
        test
    }

    fn has_subject(&self, subject: &AnyJsExpression, is_typeof: bool) -> bool {
        self.is_typeof == is_typeof
            && is_same_reference(self.subject.clone(), subject.clone()) == Some(true)
    }

    /// Returns `true` if `self` and `other` can't be true at the same time: they compare the
    /// same value against lists of primitives that have nothing in common.
    fn excludes(&self, other: &Self) -> bool {
        self.has_subject(&other.subject, other.is_typeof)
            && self.values.iter().all(|value| !other.values.contains(value))
    }
}

/// A value that `===` compares by content, written as a literal.
///
/// Two primitives are equal exactly when `===` says so. Numbers are compared by value, so `0`
/// equals `-0` and `1` equals `1.0`.
#[derive(PartialEq)]
enum Primitive {
    Null,
    Boolean(bool),
    Number(f64),
    /// The decimal notation of the value, such as `255n` for `0xFFn`.
    BigInt(Box<str>),
    String(Text),
}

impl Primitive {
    /// Returns the value of `expression` if it is a literal, a template literal without a tag
    /// and without `${}` parts, or a number literal with a `-` or `+` sign.
    ///
    /// Returns `None` for `undefined`, because a local variable can have that name. Also
    /// returns `None` for a string whose value can't be decoded, such as one that contains a
    /// legacy octal escape (`"\1"`).
    fn from_expression(expression: &AnyJsExpression) -> Option<Self> {
        let expression = expression.clone().omit_parentheses();
        if let AnyJsExpression::JsUnaryExpression(unary) = &expression {
            let is_negative = match unary.operator().ok()? {
                JsUnaryOperator::Minus => true,
                JsUnaryOperator::Plus => false,
                _ => return None,
            };
            let argument = unary.argument().ok()?.omit_parentheses();
            let StaticValue::Number(token) = argument.as_static_value()? else {
                return None;
            };
            let value = parse_js_number_with_single_rounding(token.text_trimmed())?;
            return Some(Self::Number(if is_negative { -value } else { value }));
        }
        if let AnyJsExpression::JsTemplateExpression(template) = &expression
            && template.tag().is_some()
        {
            return None;
        }
        Some(match expression.as_static_value()? {
            StaticValue::Null(_) => Self::Null,
            StaticValue::Undefined(_) => return None,
            StaticValue::Boolean(token) => Self::Boolean(token.kind() == T![true]),
            StaticValue::Number(token) => {
                Self::Number(parse_js_number_with_single_rounding(token.text_trimmed())?)
            }
            StaticValue::BigInt(token) => {
                Self::BigInt(canonicalize_js_bigint_literal(token.text_trimmed())?.into())
            }
            StaticValue::String(token) => Self::String(string_value(inner_string_text(&token))?),
            StaticValue::EmptyString(_) => Self::String(Text::new_static("")),
        })
    }
}

/// Returns whether `left` and `right` are the same code, ignoring comments, whitespace, and
/// the parentheses around them.
fn is_same_expression(left: &AnyJsExpression, right: &AnyJsExpression) -> bool {
    is_node_equal(
        left.clone().omit_parentheses().syntax(),
        right.clone().omit_parentheses().syntax(),
    )
}

/// Returns `true` if evaluating `expression` may do more than produce a value, such as calling
/// a function or assigning to a variable.
///
/// Reading a property is assumed to have no side effects, although a getter can have some.
fn may_have_side_effects(expression: &AnyJsExpression) -> bool {
    expression
        .syntax()
        .preorder_of_kinds(SIDE_EFFECT_KINDS)
        .any(|event| {
            let WalkEvent::Enter(node) = event else {
                return false;
            };
            if let Some(unary) = JsUnaryExpression::cast_ref(&node) {
                unary.operator() == Ok(JsUnaryOperator::Delete)
            } else if let Some(template) = JsTemplateExpression::cast_ref(&node) {
                // A tagged template calls its tag function.
                template.tag().is_some()
            } else {
                true
            }
        })
}

/// Expressions that throw a `TypeError` for some operands, such as reading a property of
/// `null`.
const THROWING_KINDS: SyntaxKindSet<JsLanguage> = JsStaticMemberExpression::KIND_SET
    .union(JsComputedMemberExpression::KIND_SET)
    .union(JsInExpression::KIND_SET)
    .union(JsInstanceofExpression::KIND_SET);

/// The nodes that [may_have_side_effects] inspects. Unary expressions and template literals
/// only have side effects in some of their forms.
const SIDE_EFFECT_KINDS: SyntaxKindSet<JsLanguage> = JsCallExpression::KIND_SET
    .union(JsNewExpression::KIND_SET)
    .union(JsImportCallExpression::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET)
    .union(JsPreUpdateExpression::KIND_SET)
    .union(JsPostUpdateExpression::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(JsYieldExpression::KIND_SET)
    .union(JsSpread::KIND_SET)
    .union(JsTemplateExpression::KIND_SET)
    .union(JsUnaryExpression::KIND_SET);

/// Builds `left operator right`, or `left operator !right` if `is_negated` is set. Operands
/// are wrapped in parentheses when they need them, and lose the comments and whitespace
/// around them.
fn combine_conditions(
    left: AnyJsExpression,
    operator: JsLogicalOperator,
    right: AnyJsExpression,
    is_negated: bool,
) -> Option<AnyJsExpression> {
    let precedence = operator.precedence();
    let operator_token = match operator {
        JsLogicalOperator::LogicalAnd => T![&&],
        JsLogicalOperator::LogicalOr => T![||],
        JsLogicalOperator::NullishCoalescing => T![??],
    };
    let mut right = right.trim_comments_and_trivia()?;
    if is_negated {
        right = match &right {
            // The combined condition is only checked for being truthy, so `!!x` can be `x`.
            AnyJsExpression::JsUnaryExpression(unary)
                if unary.operator().ok()? == JsUnaryOperator::LogicalNot =>
            {
                unary.argument().ok()?.trim_comments_and_trivia()?
            }
            _ => make::js_unary_expression(
                make::token(T![!]),
                parenthesize_below(right, OperatorPrecedence::Unary)?,
            )
            .into(),
        };
    }
    Some(
        make::js_logical_expression(
            parenthesize_below(left.trim_comments_and_trivia()?, precedence)?,
            make::token_decorated_with_space(operator_token),
            parenthesize_below(right, precedence)?,
        )
        .into(),
    )
}

/// Wraps `expression` in parentheses if its precedence is lower than `precedence`.
fn parenthesize_below(
    expression: AnyJsExpression,
    precedence: OperatorPrecedence,
) -> Option<AnyJsExpression> {
    Some(if expression.precedence().ok()? < precedence {
        make::parenthesized(expression).into()
    } else {
        expression
    })
}

/// Returns `true` if `node` contains the `in` operator and is part of the initializer of a `for`
/// loop.
///
/// The `in` operator must stay inside parentheses there, and the fix can remove them. Without
/// its parentheses, `for (let i = a ? (key in object ? 1 : 0) : 0; ;) {}` no longer parses.
fn is_in_for_initializer_with_in_operator(node: &JsConditionalExpression) -> bool {
    node.syntax()
        .preorder_of_kinds(JsInExpression::KIND_SET)
        .next()
        .is_some()
        && node
            .syntax()
            .ancestors()
            .filter_map(JsForStatement::cast)
            .any(|for_statement| {
                for_statement
                    .initializer()
                    .is_some_and(|initializer| initializer.range().contains_range(node.range()))
            })
}

/// Returns `true` if putting `replacement` in place of `original` could attach it to the
/// previous line: `original` starts a statement that follows a statement without a semicolon,
/// and `replacement` starts with another kind of token.
///
/// For example, replacing `a || b ? (c ? x : y) : y` with `(a || b) && c ? x : y` would turn a
/// previous line `foo` into the function call `foo(a || b)`.
fn may_join_previous_statement(original: &AnyJsExpression, replacement: &AnyJsExpression) -> bool {
    let (Some(first_token), Some(replacement_token)) = (
        original.syntax().first_token(),
        replacement.syntax().first_token(),
    ) else {
        return false;
    };
    if first_token.kind() == replacement_token.kind() {
        return false;
    }
    let is_statement_start = original
        .syntax()
        .ancestors()
        .find_map(JsExpressionStatement::cast)
        .and_then(|statement| statement.syntax().first_token())
        .is_some_and(|token| token == first_token);
    is_statement_start
        && first_token
            .prev_token()
            .is_some_and(|previous| !STATEMENT_TERMINATORS.contains(previous.kind()))
}

const STATEMENT_TERMINATORS: TokenSet<JsSyntaxKind> = token_set![T![;], T!['{']];

/// Counts the comments inside `expression`, without the comments before its first token and
/// after its last token.
fn count_inner_comments(expression: &AnyJsExpression) -> usize {
    let mut tokens = expression.syntax().borrowed_tokens().peekable();
    let mut count = 0;
    let mut is_first_token = true;
    while let Some(token) = tokens.next() {
        if !is_first_token {
            count += token
                .leading_trivia_pieces()
                .filter(|piece| piece.kind().is_comment())
                .count();
        }
        if tokens.peek().is_some() {
            count += token
                .trailing_trivia_pieces()
                .filter(|piece| piece.kind().is_comment())
                .count();
        }
        is_first_token = false;
    }
    count
}
