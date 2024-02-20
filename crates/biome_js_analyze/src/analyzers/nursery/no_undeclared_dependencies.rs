use crate::manifest_services::Manifest;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::AnyJsImportSpecifierLike;
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow the use of dependencies that aren't specified in the `package.json`.
    ///
    /// Indirect dependencies will trigger the rule because they aren't declared in the `package.json`. This means that if package `@org/foo` has a dependency on `lodash`, and then you use
    /// `import "lodash"` somewhere in your project, the rule will trigger a diagnostic for this import.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,ignore
    /// import "vite";
    /// ```
    ///
    pub NoUndeclaredDependencies {
        version: "next",
        name: "noUndeclaredDependencies",
        recommended: false,
    }
}

impl Rule for NoUndeclaredDependencies {
    type Query = Manifest<AnyJsImportSpecifierLike>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let text = node.inner_string_text()?;
        if !text.text().starts_with('.')
            && !ctx.is_dependency(text.text())
            && !ctx.is_dev_dependency(text.text())
        {
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
