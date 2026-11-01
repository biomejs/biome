use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsArrowFunctionExpression, JsBinaryExpression,
    JsBinaryOperator, JsExportDefaultExpressionClause, JsExpressionStatement, JsLanguage,
    JsSyntaxKind, JsSyntaxToken, T,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, BatchMutationExt, Direction, SyntaxKindSet};
use biome_rule_options::no_subtraction_comparison::NoSubtractionComparisonOptions;

declare_lint_rule! {
    /// Disallow subtracting two values only to compare the result with `0`.
    ///
    /// Code such as `a - b > 0` subtracts one value from another, then checks whether the
    /// difference is above, below, or equal to `0`. Comparing the two values directly, as in
    /// `a > b`, is shorter and states the intent more clearly.
    ///
    /// This pattern often remains after code is copied from a sort comparison function such as
    /// `(a, b) => a - b`.
    ///
    /// The rule checks the operators `>`, `>=`, `<`, `<=`, `===`, `!==`, `==`, and `!=`, with
    /// `0` on either side. When `0` is on the left, the fix flips the operator, so `0 < a - b`
    /// becomes `a > b`.
    ///
    /// :::caution
    /// The fix gives the same result only when both values are numbers, so it is unsafe:
    ///
    /// - Strings are compared character by character instead of as numbers.
    ///   `"10" - "5" > 0` is `true`, but `"10" > "5"` is `false`.
    /// - `===` and `!==` check whether two objects, such as two dates, are the same object.
    ///   `start - end === 0` checks whether two dates hold the same time, but `start === end`
    ///   checks whether they are the same date object.
    /// - `Infinity - Infinity` is `NaN` ("not a number"), and every comparison with `NaN` is
    ///   `false`. `Infinity - Infinity >= 0` is `false`, but `Infinity >= Infinity` is `true`.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (a - b > 0) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (a - b <= 0) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (0 < a - b) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (a - b === 0) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (a > b) {}
    /// if (a - b > 1) {}
    /// if (a + b > 0) {}
    /// if (a - b > 0n) {}
    /// ```
    ///
    pub NoSubtractionComparison {
        version: "next",
        name: "noSubtractionComparison",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-subtraction-comparison").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoSubtractionComparison {
    type Query = Ast<JsBinaryExpression>;
    type State = SubtractionComparison;
    type Signals = Option<Self::State>;
    type Options = NoSubtractionComparisonOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let operator = node.operator_token().ok()?.kind();
        if !COMPARISON_OPERATORS.contains(operator) {
            return None;
        }

        let left = node.left().ok()?;
        let right = node.right().ok()?;
        if is_zero(&right)
            && let Some(subtraction) = as_subtraction(&left)
        {
            Some(SubtractionComparison {
                subtraction,
                operator,
            })
        } else if is_zero(&left)
            && let Some(subtraction) = as_subtraction(&right)
        {
            Some(SubtractionComparison {
                subtraction,
                operator: mirrored_operator(operator),
            })
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This comparison subtracts two values and compares the difference with "<Emphasis>"0"</Emphasis>"."
                },
            )
            .note(markup! {
                "Compare the two values directly instead, which is shorter and states the intent more clearly."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let left = state.subtraction.left().ok()?;
        let right = state.subtraction.right().ok()?;
        if has_removed_comments(node, [&left, &right]) {
            return None;
        }

        let first = node.syntax().first_token()?;
        let last = node.syntax().last_token()?;
        let new_first = left.syntax().first_token()?;
        let new_last = right.syntax().last_token()?;

        // Some tokens at the start of a statement change how the statement is parsed. For
        // example, `{` starts a block, and `(` continues the previous line when it doesn't end
        // with a semicolon. The original code doesn't start with such a token, or it would have
        // been parsed that way already.
        if new_first != first
            && UNSAFE_STATEMENT_STARTS.contains(new_first.kind())
            && starts_statement(node)
        {
            return None;
        }

        let mut new_left = left.trim_comments_and_trivia()?;
        let mut new_right = right.trim_comments_and_trivia()?;
        // Parentheses can separate the comparison from a keyword without whitespace, as in
        // `return(0) < a - b`. The fix must not join the tokens into `returna > b`.
        if new_first != first
            && first
                .prev_token()
                .is_some_and(|previous| are_joined_words(&previous, &first))
        {
            new_left = new_left.with_leading_trivia_pieces(
                make::token_decorated_with_space(T![=])
                    .leading_trivia()
                    .pieces(),
            )?;
        }
        if new_last != last
            && last
                .next_token()
                .is_some_and(|next| are_joined_words(&last, &next))
        {
            new_right = new_right.with_trailing_trivia_pieces(
                make::token_decorated_with_space(T![=])
                    .trailing_trivia()
                    .pieces(),
            )?;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node_transfer_trivia(
            node.clone(),
            make::js_binary_expression(
                new_left,
                make::token_decorated_with_space(state.operator),
                new_right,
            ),
        )?;

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Compare the values directly." }.to_owned(),
            mutation,
        ))
    }
}

/// A comparison of a subtraction `a - b` with `0`.
pub struct SubtractionComparison {
    /// The `a - b` side of the comparison.
    subtraction: JsBinaryExpression,
    /// The operator that compares `a` and `b` directly. It's the original operator, flipped
    /// when `0` is on the left.
    operator: JsSyntaxKind,
}

const COMPARISON_OPERATORS: TokenSet<JsSyntaxKind> = token_set![
    T![>],
    T![>=],
    T![<],
    T![<=],
    T![===],
    T![!==],
    T![==],
    T![!=]
];

/// Tokens that change how the code is parsed when they start an expression statement, the
/// body of an arrow function, or an `export default` declaration.
const UNSAFE_STATEMENT_STARTS: TokenSet<JsSyntaxKind> = token_set![
    T!['('],
    T!['['],
    T!['{'],
    T!['`'],
    T![+],
    T![++],
    T![-],
    T![--],
    T![/],
    T![/=],
    T![<],
    T![async],
    T![class],
    T![function],
    T![let],
    JsSyntaxKind::JS_REGEX_LITERAL,
];

/// Parents of the expressions that start a statement or a body.
const STATEMENT_START_PARENTS: SyntaxKindSet<JsLanguage> = JsExpressionStatement::KIND_SET
    .union(JsArrowFunctionExpression::KIND_SET)
    .union(JsExportDefaultExpressionClause::KIND_SET);

/// Returns whether `expression` is the number `0`, ignoring parentheses. The bigint `0n`
/// doesn't match.
fn is_zero(expression: &AnyJsExpression) -> bool {
    matches!(
        expression.clone().omit_parentheses(),
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(number),
        ) if number.as_number() == Some(0.0)
    )
}

/// Returns `expression` if it is a subtraction such as `a - b`, ignoring parentheses.
fn as_subtraction(expression: &AnyJsExpression) -> Option<JsBinaryExpression> {
    let AnyJsExpression::JsBinaryExpression(binary) = expression.clone().omit_parentheses() else {
        return None;
    };
    (binary.operator().ok()? == JsBinaryOperator::Minus).then_some(binary)
}

/// Returns the operator that gives the same result when the operands of `operator` swap
/// sides. For example, `0 < x` is the same as `x > 0`.
fn mirrored_operator(operator: JsSyntaxKind) -> JsSyntaxKind {
    match operator {
        T![<] => T![>],
        T![<=] => T![>=],
        T![>] => T![<],
        T![>=] => T![<=],
        operator => operator,
    }
}

/// Returns whether `before` and `after` touch without whitespace in between, and one of them is
/// a keyword or a name. Replacing the other token with a name would merge the two into a single
/// word.
fn are_joined_words(before: &JsSyntaxToken, after: &JsSyntaxToken) -> bool {
    (is_word(before) || is_word(after))
        && before.trailing_trivia().pieces().next().is_none()
        && after.leading_trivia().pieces().next().is_none()
}

fn is_word(token: &JsSyntaxToken) -> bool {
    token.kind().is_keyword() || token.kind() == JsSyntaxKind::IDENT
}

/// Returns whether `comparison` is at the start of an expression statement, of the body of an
/// arrow function, or of an `export default` declaration.
fn starts_statement(comparison: &JsBinaryExpression) -> bool {
    let first = comparison.syntax().first_token();
    comparison
        .syntax()
        .ancestors()
        .take_while(|node| node.first_token() == first)
        .any(|node| {
            node.parent()
                .is_some_and(|parent| STATEMENT_START_PARENTS.matches(parent.kind()))
        })
}

/// Returns whether the fix would delete a comment.
///
/// The fix keeps the comments before and after `comparison`, and the comments inside each of
/// `operands`. It deletes the comments next to the operators, the `0`, the parentheses that
/// it removes, and the comments at the start or end of an operand.
fn has_removed_comments(comparison: &JsBinaryExpression, operands: [&AnyJsExpression; 2]) -> bool {
    let first = comparison.syntax().first_token();
    let last = comparison.syntax().last_token();
    comparison
        .syntax()
        .descendants_tokens(Direction::Next)
        .any(|token| {
            let operand = operands.iter().find(|operand| {
                operand
                    .syntax()
                    .text_trimmed_range()
                    .contains_range(token.text_trimmed_range())
            });
            let keeps_leading = first.as_ref() == Some(&token)
                || operand
                    .is_some_and(|operand| operand.syntax().first_token() != Some(token.clone()));
            let keeps_trailing = last.as_ref() == Some(&token)
                || operand
                    .is_some_and(|operand| operand.syntax().last_token() != Some(token.clone()));
            (!keeps_leading && token.has_leading_comments())
                || (!keeps_trailing && token.has_trailing_comments())
        })
}
