use biome_analyze::{FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression,
    JsBinaryOperator, JsCallExpression, JsLanguage, JsSyntaxKind, JsUnaryOperator,
    JsVariableDeclarator, OperatorPrecedence, T, TsAsExpression, TsSatisfiesExpression,
    binary_like_expression::AnyJsBinaryLikeExpression,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxKindSet, TextRange};
use biome_rule_options::use_consistent_existence_index_check::UseConsistentExistenceIndexCheckOptions;

use crate::{
    JsRuleAction, ast_utils::starts_unterminated_statement, services::semantic::Semantic,
};

declare_lint_rule! {
    /// Enforce comparing with `-1` when checking whether `indexOf()`, `lastIndexOf()`, `findIndex()`, or `findLastIndex()` found an element.
    ///
    /// These methods return `-1` when they find nothing, so `index === -1` and `index !== -1`
    /// state the intent more clearly than `index < 0`, `index >= 0`, or `index > -1`.
    ///
    /// The rule checks `const` variables that hold the result of one of these calls.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const index = list.indexOf(item);
    /// if (index < 0) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = list.lastIndexOf(item);
    /// if (index >= 0) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = list.findIndex((element) => element > 10);
    /// if (index > -1) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const index = list.indexOf(item);
    /// if (index === -1) {}
    /// if (index !== -1) {}
    ///
    /// // `let` and `var` variables can be reassigned, so they aren't checked.
    /// let position = list.findLastIndex((element) => element > 10);
    /// if (position < 0) {}
    /// ```
    ///
    pub UseConsistentExistenceIndexCheck {
        version: "next",
        name: "useConsistentExistenceIndexCheck",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("consistent-existence-index-check").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseConsistentExistenceIndexCheck {
    type Query = Semantic<JsBinaryExpression>;
    type State = IndexCheck;
    type Signals = Option<Self::State>;
    type Options = UseConsistentExistenceIndexCheckOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let comparison = IndexComparison::from_binary(node)?;
        let left = node.left().ok()?.omit_parentheses();
        let reference = left.as_js_identifier_expression()?.name().ok()?;
        let binding = ctx.model().binding(&reference)?.tree();
        let declarator = JsVariableDeclarator::cast(binding.syntax().parent()?)?;
        if !declarator.declaration()?.is_const() {
            return None;
        }
        let initializer = declarator.initializer()?.expression().ok()?.omit_parentheses();
        let method = IndexMethod::from_call(initializer.as_js_call_expression()?)?;
        Some(IndexCheck { method, comparison })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let range = TextRange::new(
            node.operator_token().ok()?.text_trimmed_range().start(),
            node.range().end(),
        );
        let IndexCheck { method, comparison } = state;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This uses "<Emphasis>{comparison}</Emphasis>" to check whether "<Emphasis>{method}</Emphasis>" "{comparison.outcome()}"."
                },
            )
            .note(markup! {
                <Emphasis>{method}</Emphasis>" returns "<Emphasis>"-1"</Emphasis>" when it finds no element. Comparing the result with "<Emphasis>"-1"</Emphasis>" makes it clear that the code checks for this value."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let operator = node.operator_token().ok()?;
        let new_operator = make::token(state.comparison.replacement_operator())
            .with_leading_trivia_pieces(operator.leading_trivia().pieces())
            .with_trailing_trivia_pieces(operator.trailing_trivia().pieces());
        let mut new_node = node.clone().with_operator_token_token(new_operator);
        if state.comparison.compares_with_zero() {
            let right = node.right().ok()?;
            let zero = right.clone().omit_parentheses();
            let zero_token = zero.syntax().first_token()?;
            let negative_one = AnyJsExpression::from(make::js_unary_expression(
                make::token(T![-]).with_leading_trivia_pieces(zero_token.leading_trivia().pieces()),
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsNumberLiteralExpression(
                        make::js_number_literal_expression(
                            make::js_number_literal("1")
                                .with_trailing_trivia_pieces(zero_token.trailing_trivia().pieces()),
                        ),
                    ),
                ),
            ));
            let new_right = if right == zero {
                negative_one
            } else {
                AnyJsExpression::cast(
                    right
                        .into_syntax()
                        .replace_child(zero.into_syntax().into(), negative_one.into_syntax().into())?,
                )?
            };
            new_node = new_node.with_right(new_right);
        }

        let mut mutation = ctx.root().begin();
        if needs_parentheses(node) {
            let old_node = AnyJsExpression::from(node.clone());
            // A `(` at the start of a line would continue the previous statement.
            if starts_unterminated_statement(&old_node) {
                return None;
            }
            let new_node = new_node
                .with_leading_trivia_pieces([])?
                .with_trailing_trivia_pieces([])?;
            mutation.replace_node(old_node, make::parenthesized(new_node).into());
        } else {
            mutation.replace_node_discard_trivia(node.clone(), new_node);
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{state.comparison.replacement()}</Emphasis>" instead." },
            mutation,
        ))
    }
}

/// A comparison of an index against a sign boundary, and the method call that produced the index.
#[derive(Clone, Copy, Debug)]
pub struct IndexCheck {
    method: IndexMethod,
    comparison: IndexComparison,
}

#[derive(Clone, Copy, Debug)]
enum IndexMethod {
    IndexOf,
    LastIndexOf,
    FindIndex,
    FindLastIndex,
}

impl IndexMethod {
    /// Returns the method that `call` invokes, if it is a plain `object.method()` call to one of
    /// the methods that return `-1` when nothing is found.
    ///
    /// Optional calls such as `object?.indexOf()` and computed calls such as
    /// `object["indexOf"]()` return `None`.
    fn from_call(call: &JsCallExpression) -> Option<Self> {
        if call.is_optional_chain() {
            return None;
        }
        let callee = call.callee().ok()?.omit_parentheses();
        let member = callee.as_js_static_member_expression()?;
        let name = member.member().ok()?;
        match name.as_js_name()?.value_token().ok()?.text_trimmed() {
            "indexOf" => Some(Self::IndexOf),
            "lastIndexOf" => Some(Self::LastIndexOf),
            "findIndex" => Some(Self::FindIndex),
            "findLastIndex" => Some(Self::FindLastIndex),
            _ => None,
        }
    }
}

impl biome_console::fmt::Display for IndexMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::IndexOf => "indexOf()",
            Self::LastIndexOf => "lastIndexOf()",
            Self::FindIndex => "findIndex()",
            Self::FindLastIndex => "findLastIndex()",
        })
    }
}

/// A comparison that tests the sign of an index instead of comparing it with `-1`.
#[derive(Clone, Copy, Debug)]
enum IndexComparison {
    /// `index < 0`
    LessThanZero,
    /// `index >= 0`
    GreaterThanOrEqualZero,
    /// `index > -1`
    GreaterThanNegativeOne,
}

impl IndexComparison {
    fn from_binary(binary: &JsBinaryExpression) -> Option<Self> {
        let right = binary.right().ok()?.omit_parentheses();
        match binary.operator().ok()? {
            JsBinaryOperator::LessThan if is_number(&right, 0.0) => Some(Self::LessThanZero),
            JsBinaryOperator::GreaterThanOrEqual if is_number(&right, 0.0) => {
                Some(Self::GreaterThanOrEqualZero)
            }
            JsBinaryOperator::GreaterThan if is_negative_one(&right) => {
                Some(Self::GreaterThanNegativeOne)
            }
            _ => None,
        }
    }

    const fn checks_not_found(self) -> bool {
        matches!(self, Self::LessThanZero)
    }

    const fn compares_with_zero(self) -> bool {
        matches!(self, Self::LessThanZero | Self::GreaterThanOrEqualZero)
    }

    const fn replacement_operator(self) -> JsSyntaxKind {
        if self.checks_not_found() {
            T![===]
        } else {
            T![!==]
        }
    }

    /// Describes what the comparison checks for, as the end of a sentence such as
    /// "to check whether `indexOf()` found no element".
    const fn outcome(self) -> &'static str {
        if self.checks_not_found() {
            "found no element"
        } else {
            "found an element"
        }
    }

    const fn replacement(self) -> &'static str {
        if self.checks_not_found() {
            "=== -1"
        } else {
            "!== -1"
        }
    }
}

impl biome_console::fmt::Display for IndexComparison {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::LessThanZero => "< 0",
            Self::GreaterThanOrEqualZero => ">= 0",
            Self::GreaterThanNegativeOne => "> -1",
        })
    }
}

/// Returns `true` if `expression` is a number literal with the given value.
fn is_number(expression: &AnyJsExpression, value: f64) -> bool {
    expression
        .as_any_js_literal_expression()
        .and_then(|literal| literal.as_js_number_literal_expression())
        .and_then(|literal| literal.as_number())
        .is_some_and(|number| number == value)
}

/// Returns `true` if `expression` is `-1`, allowing parentheses around the `1`.
fn is_negative_one(expression: &AnyJsExpression) -> bool {
    let Some(unary) = expression.as_js_unary_expression() else {
        return false;
    };
    unary.operator() == Ok(JsUnaryOperator::Minus)
        && unary
            .argument()
            .is_ok_and(|argument| is_number(&argument.omit_parentheses(), 1.0))
}

/// Returns `true` if replacing the relational comparison `binary` with an equality comparison
/// requires parentheses to keep the same grouping.
///
/// Equality operators bind more loosely than relational operators. For example,
/// `a === index < 0` groups as `a === (index < 0)`, so its replacement must be written as
/// `a === (index === -1)`.
fn needs_parentheses(binary: &JsBinaryExpression) -> bool {
    let Some(parent) = binary.syntax().parent() else {
        return false;
    };
    if TYPE_WRAPPER_KINDS.matches(parent.kind()) {
        return true;
    }
    let Some(parent) = AnyJsBinaryLikeExpression::cast(parent) else {
        return false;
    };
    let Ok(precedence) = parent.operator().map(|operator| operator.precedence()) else {
        return false;
    };
    precedence > OperatorPrecedence::Equality
        || (precedence == OperatorPrecedence::Equality
            && parent
                .right()
                .is_ok_and(|right| right.syntax() == binary.syntax()))
}

/// The TypeScript expressions that bind more tightly than equality operators when they wrap a
/// comparison, such as `index < 0 as boolean`.
const TYPE_WRAPPER_KINDS: SyntaxKindSet<JsLanguage> =
    TsAsExpression::KIND_SET.union(TsSatisfiesExpression::KIND_SET);
