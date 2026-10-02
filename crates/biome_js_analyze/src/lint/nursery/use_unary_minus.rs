use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
    JsPreUpdateOperator, JsSyntaxKind, JsUnaryOperator, OperatorPrecedence, T,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::use_unary_minus::UseUnaryMinusOptions;

use crate::{JsRuleAction, ast_utils::starts_unterminated_statement};

declare_lint_rule! {
    /// Enforce the use of the unary minus operator over multiplying or dividing by `-1`.
    ///
    /// Multiplying or dividing a value by `-1` is an indirect way to negate it (flip its sign).
    /// The unary minus operator (`-x`) is shorter and states the intent directly.
    ///
    /// The rule reports `x * -1`, `-1 * x`, and `x / -1`.
    /// It doesn't report `-1 / x`, because dividing `-1` by `x` doesn't produce `-x`.
    ///
    /// The fix is unsafe because it can change behavior for BigInt values:
    /// `x * -1` throws a `TypeError` when `x` is a BigInt, but `-x` doesn't.
    /// Biome doesn't offer a fix when the expression contains comments that the fix would remove.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const inverted = value * -1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const flipped = -1 * someNumber;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const divided = value / -1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const inverted = -value;
    /// const reciprocal = -1 / value;
    /// const doubled = value * -2;
    /// ```
    ///
    pub UseUnaryMinus {
        version: "next",
        name: "useUnaryMinus",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-unary-minus").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseUnaryMinus {
    type Query = Ast<JsBinaryExpression>;
    /// The operand negated by the expression.
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = UseUnaryMinusOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let left = node.left().ok()?;
        let right = node.right().ok()?;
        match node.operator().ok()? {
            JsBinaryOperator::Times => match (is_negative_one(&left), is_negative_one(&right)) {
                (false, true) => Some(left),
                (true, false) => Some(right),
                _ => None,
            },
            // `-1 / x` isn't `-x`.
            JsBinaryOperator::Divide if is_negative_one(&right) && !is_negative_one(&left) => {
                Some(left)
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let operation = match node.operator().ok()? {
            JsBinaryOperator::Divide => "Dividing",
            _ => "Multiplying",
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    {operation}" by "<Emphasis>"-1"</Emphasis>" is a roundabout way to negate a value."
                },
            )
            .note(markup! {
                "The unary minus operator is shorter and states the intent directly."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, operand: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        // The fix only keeps the operand, so comments around the operator and `-1` would be lost.
        // Comments before the first token and after the last token are kept, because the fix
        // moves them to the replacement.
        // A replacement starting with `-` can also join the previous statement when no
        // semicolon separates them.
        let expression = AnyJsExpression::from(node.clone());
        if node.syntax().has_inner_comments() || starts_unterminated_statement(&expression) {
            return None;
        }

        let inner = operand.clone().omit_parentheses();
        let argument = if needs_parentheses(&inner)? {
            match operand {
                AnyJsExpression::JsParenthesizedExpression(_) => {
                    operand.clone().trim_comments_and_trivia()?
                }
                _ => make::parenthesized(inner.trim_comments_and_trivia()?).into(),
            }
        } else {
            inner.trim_comments_and_trivia()?
        };
        let last = node.syntax().last_token()?;
        let argument = argument.with_trailing_trivia_pieces(last.trailing_trivia().pieces())?;

        let first = node.syntax().first_token()?;
        let mut minus =
            make::token(T![-]).with_leading_trivia_pieces(first.leading_trivia().pieces());
        // Without a space, `a-x*-1` would become `a--x`, which parses as a decrement.
        if let Some(previous) = first.prev_token()
            && MINUS_TOKENS.contains(previous.kind())
            && previous.text_trimmed_range().end() == first.text_trimmed_range().start()
        {
            minus = minus.with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]);
        }
        let replacement: AnyJsExpression = make::js_unary_expression(minus, argument).into();

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(expression, replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the unary minus operator." }.to_owned(),
            mutation,
        ))
    }
}

/// Tokens that would merge with a following `-`.
const MINUS_TOKENS: TokenSet<JsSyntaxKind> = token_set![T![-], T![--]];

/// Returns `true` if `expression` is the number `-1`, ignoring parentheses.
///
/// Any spelling of the number one matches, such as `-1.0`, `-0x1`, or `-(1)`.
/// The bigint `-1n` doesn't match.
fn is_negative_one(expression: &AnyJsExpression) -> bool {
    let AnyJsExpression::JsUnaryExpression(unary) = expression.clone().omit_parentheses() else {
        return false;
    };
    if !matches!(unary.operator(), Ok(JsUnaryOperator::Minus)) {
        return false;
    }
    matches!(
        unary.argument().map(AnyJsExpression::omit_parentheses),
        Ok(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(number)
        )) if number.as_number() == Some(1.0)
    )
}

/// Returns `true` if `argument` must be parenthesized to be the argument of a unary minus.
///
/// Returns `None` if `argument` contains a syntax error.
fn needs_parentheses(argument: &AnyJsExpression) -> Option<bool> {
    Some(match argument {
        // A `-` directly before `-y` or `--y` produces `--y` or `---y`, which both start
        // with a decrement.
        AnyJsExpression::JsUnaryExpression(unary) => {
            unary.operator().ok()? == JsUnaryOperator::Minus
        }
        AnyJsExpression::JsPreUpdateExpression(update) => {
            update.operator().ok()? == JsPreUpdateOperator::Decrement
        }
        // An arrow function is ranked as a primary expression, but `-() => 1` is a syntax error.
        AnyJsExpression::JsArrowFunctionExpression(_) => true,
        _ => argument.precedence().ok()? < OperatorPrecedence::Unary,
    })
}
