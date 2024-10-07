use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsAssignmentExpression;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow use `document.cookie` directly.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// document.cookie = "foo=bar";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const array = document.cookie.split("; ");
    /// ```
    ///
    pub NoDocumentCookie {
        version: "next",
        name: "noDocumentCookie",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoDocumentCookie {
    type Query = Ast<JsAssignmentExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let left = node.left().ok()?;
        if left.text() == "document.cookie" {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Using" <Emphasis>"document.cookie"</Emphasis>" directly is not recommended"
                },
            )
            .note(markup! {
                "Consider using the `Cookie Store API` or a library instead"
            }),
        )
    }
}
