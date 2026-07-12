use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, JsBinaryExpression, JsParenthesizedExpression, JsSyntaxKind::*, JsSyntaxNode};
use biome_js_syntax::is_negation;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_negation_in_equality_check::NoNegationInEqualityCheckOptions;

declare_lint_rule! {
    /// Disallow negated expressions on the left side of an equality check.
    ///
    /// When a negation operator (`!`) is used on the left side of an equality check (`===` or `!==`),
    /// the negation binds more tightly than the comparison operator due to operator precedence.
    /// This means `!foo === bar` is evaluated as `(!foo) === bar`, which is almost always
    /// unintended. The developer likely meant `foo !== bar`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (!foo === bar) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (!foo !== bar) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (foo !== bar) {}
    /// ```
    ///
    /// ```js
    /// if (!(foo === bar)) {}
    /// ```
    ///
    pub NoNegationInEqualityCheck {
        version: "next",
        name: "noNegationInEqualityCheck",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-negation-in-equality-check").same()],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

/// Recursively walks through `JsParenthesizedExpression` wrappers to find
/// the innermost non-parenthesized syntax node.
fn skip_parens(mut node: JsSyntaxNode) -> JsSyntaxNode {
    while let Some(paren) = JsParenthesizedExpression::cast_ref(&node) {
        match paren.expression() {
            Ok(inner) => node = inner.syntax().clone(),
            Err(_) => break,
        }
    }
    node
}

impl Rule for NoNegationInEqualityCheck {
    type Query = Ast<JsBinaryExpression>;
    type State = JsBinaryExpression;
    type Signals = Option<Self::State>;
    type Options = NoNegationInEqualityCheckOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let op = node.operator_token().ok()?;

        // Only check strict equality operators (=== and !==)
        if !matches!(op.kind(), EQ3 | NEQ2) {
            return None;
        }

        let left = node.left().ok()?;

        // Recursively unwrap parenthesized expressions, e.g. `((!foo)) === bar`
        let inner = skip_parens(left.syntax().clone());

        // Check if the unwrapped left side is a negation expression (!expr)
        let unary = is_negation(&inner)?;

        // Skip double negation (!!expr or !(...( !expr )...)) — this is
        // intentional boolean coercion, e.g. `!!foo === bar` or `!((!foo)) === bar`.
        let argument = unary.argument().ok()?;
        let arg_inner = skip_parens(argument.syntax().clone());
        if is_negation(&arg_inner).is_some() {
            return None;
        }

        Some(node.clone())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "A negation is used on the left side of this equality check."
                },
            )
            .note(markup! {
                    "Due to operator precedence, the negation binds more tightly than the equality operator. "
                    "The expression "<Emphasis>"!foo === bar"</Emphasis>" evaluates as "<Emphasis>"(!foo) === bar"</Emphasis>", not "<Emphasis>"!(foo === bar)"</Emphasis>"."
            })
            .note(markup! {
                "Replace "<Emphasis>"!left operator right"</Emphasis>" with the proper negated comparison, or wrap the comparison in parentheses with a negation."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = state;
        let left = node.left().ok()?;
        let right = node.right().ok()?;
        let operator = node.operator_token().ok()?;

        // Recursively unwrap parenthesized expressions to find the negation
        let inner = skip_parens(left.syntax().clone());
        let unary = is_negation(&inner)?;

        // The argument of `!` is the expression to use as the new left side
        let negated_expr = unary.argument().ok()?;

        // Flip the operator: === → !==, !== → ===
        let new_op_kind = match operator.kind() {
            EQ3 => NEQ2,
            NEQ2 => EQ3,
            _ => return None,
        };

        let new_binary = make::js_binary_expression(
            negated_expr,
            make::token_decorated_with_space(new_op_kind),
            right,
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::JsBinaryExpression(node.clone()),
            AnyJsExpression::from(new_binary),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the proper negated comparison operator instead." }.to_owned(),
            mutation,
        ))
    }
}
