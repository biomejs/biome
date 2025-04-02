use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule, RuleDomain};
use biome_console::markup;
use biome_js_syntax::JsCallExpression;
use biome_rowan::{TextRange, AstNode};

declare_lint_rule! {
    /// Disallow useVisibleTask$() which can impact performance.
    ///
    /// useVisibleTask$() runs immediately when the user sees this component in the browser, which is an anti-pattern 
    /// that can lead to poor user experience. Consider using alternative hooks that run tasks 
    /// asynchronously or in response to user interactions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// useVisibleTask$(() => {
    ///   // Heavy computation that runs immediately on page load
    ///   browserOperation$();
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // Use alternative hooks that run tasks asynchronously or in response to user interactions
    /// useTask$(async () => {
    ///   track(() => interaction.value);
    ///   if (isServer) return;
    ///   await expensiveBrowserOperation$();
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
        let ident = callee.as_js_identifier_expression()?;
        
        let name_token = ident.name().ok()?.value_token().ok()?;
        let function_name = name_token.text_trimmed();

        if function_name == "useVisibleTask$" {
            Some(node.syntax().text_trimmed_range())
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
                    "useVisibleTask$() runs immediately and can temporarily make the user interface unresponsive, preventing user interactions until the task completes"
                },
            )
            .note(markup! {
                "Consider using useTask$(), useOn(), useOnDocument(), or useOnWindow() instead"
            }),
        )
    }
}
