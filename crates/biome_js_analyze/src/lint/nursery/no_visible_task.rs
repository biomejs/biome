use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule, RuleDomain};
use biome_console::markup;
use biome_js_syntax::JsCallExpression;
use biome_rowan::{TextRange, AstNode};

declare_lint_rule! {
    /// Disallow useVisibleTask$() which blocks the main thread.
    ///
    /// useVisibleTask$() runs eagerly and blocks the main thread, preventing user interaction 
    /// until the task is finished.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// useVisibleTask$(() => {
    ///   // Heavy computation that blocks the main thread
    ///   browserOperation$();
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // Use alternative hooks that don't block the main thread
    /// useTask$(async () => {
    ///   await expensiveServerOperation$();
    /// });
    ///
    /// useOn('scroll', handleScroll$);
    /// ```
    pub(crate) NoVisibleTask {
        version: "next",
        name: "noVisibleTask",
        language: "js",
        recommended: true,
        domains: &[RuleDomain::Qwik],
    }
}

impl Rule for NoVisibleTask {
    type Query = Ast<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        
        let callee = node.callee().ok()?;
        if let Some(ident) = callee.as_js_identifier_expression() {
            let name_token = ident.name().ok()?.value_token().ok()?;
            let name = name_token.text_trimmed();
            if name == "useVisibleTask$" {
                Some(node.syntax().text_trimmed_range())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "useVisibleTask$() runs eagerly and blocks the main thread, preventing user interaction until the task is finished"
                },
            )
            .note(markup! {
                "Consider using useTask$(), useOn(), useOnDocument(), or useOnWindow() instead"
            }),
        )
    }
}
