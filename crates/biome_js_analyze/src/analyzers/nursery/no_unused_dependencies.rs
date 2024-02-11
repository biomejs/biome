use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsModuleSource;
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow the use of dependencies that aren't specified in the `package.json`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import "vite";
    /// ```
    ///
    pub(crate) NoUnusedDependencies {
        version: "next",
        name: "noUnusedDependencies",
        recommended: false,
    }
}

impl Rule for NoUnusedDependencies {
    type Query = Ast<JsModuleSource>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let text = node.inner_string_text().ok()?;
        if !text.text().starts_with('.') && !ctx.is_dependency(text.text()) {
            return Some(());
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The current dependency isn't specified in your package.json."
                },
            )
            .note(markup! {
                "This could lead to errors."
            })
            .note(markup! {
                "Add the dependency in your manifest."
            }),
        )
    }
}
