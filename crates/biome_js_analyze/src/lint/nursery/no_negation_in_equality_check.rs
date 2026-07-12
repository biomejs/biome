use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsBinaryExpression, JsSyntaxKind::*, JsSyntaxToken};
use biome_js_syntax::is_negation;
use biome_rowan::AstNode;
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
        fix_kind: FixKind::None,
    }
}

impl Rule for NoNegationInEqualityCheck {
    type Query = Ast<JsBinaryExpression>;
    type State = JsSyntaxToken;
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

        // Check if the left side is a negation expression (!expr)
        let unary = is_negation(left.syntax())?;

        // Skip double negation (!!expr) — this is intentional boolean coercion
        let argument = unary.argument().ok()?;
        if is_negation(argument.syntax()).is_some() {
            return None;
        }

        Some(op)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _op: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
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

}
