use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_embeds::astro::component_imports;
use biome_js_syntax::JsImport;
use biome_rowan::TextRange;
use biome_rule_options::no_astro_component_in_framework::NoAstroComponentInFrameworkOptions;

declare_lint_rule! {
    /// Disallow importing Astro components into framework component files.
    ///
    /// Astro components run in Astro's rendering environment. React, Vue, Svelte,
    /// and other UI frameworks cannot render them directly.
    /// Compose these components in an `.astro` file and pass the rendered content
    /// to the framework component through a slot or children.
    ///
    /// This rule checks default `.astro` imports and named `Image` and `Picture`
    /// imports from `astro:assets` in `.jsx`, `.tsx`, `.vue`, and `.svelte` files.
    /// It allows type-only imports, named helper exports, and imports with query
    /// suffixes such as `?raw`. Namespace imports and re-exports are not checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// import Card from "./Card.astro";
    /// export default function App() { return <Card />; }
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    ///   import { Picture } from "astro:assets";
    /// </script>
    /// <Picture src="/image.png" alt="A mountain" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// ---
    /// import Card from "./Card.astro";
    /// import App from "./App.jsx";
    /// ---
    /// <App client:load><Card /></App>
    /// ```
    ///
    /// ## References
    ///
    /// - [Mixing frameworks](https://docs.astro.build/en/guides/framework-components/#mixing-frameworks)
    pub NoAstroComponentInFramework {
        version: "next",
        name: "noAstroComponentInFramework",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Astro],
    }
}

impl Rule for NoAstroComponentInFramework {
    type Query = Ast<JsImport>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoAstroComponentInFrameworkOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !matches!(
            ctx.file_path().extension(),
            Some("jsx" | "tsx" | "vue" | "svelte")
        ) {
            return Box::default();
        }
        component_imports(ctx.query())
            .into_iter()
            .map(|binding| binding.range)
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(rule_category!(), *range, markup! {
            "This Astro component is imported into a framework component file."
        }).note(markup! {
            "UI frameworks cannot render Astro components directly."
        }).note(markup! {
            "Import the component in an .astro file and pass its rendered content through a slot or children."
        }))
    }
}
