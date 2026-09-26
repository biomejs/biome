use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::AnyJsImportLike;
use biome_rowan::TextRange;
use biome_rule_options::use_svelte_kit_rune_imports::UseSvelteKitRuneImportsOptions;

declare_lint_rule! {
    /// Require importing SvelteKit's app state from `$app/state` instead of `$app/stores`.
    ///
    /// SvelteKit 2.12 deprecated `$app/stores` in favor of `$app/state`. The `$app/state` module
    /// exposes `page`, `navigating`, and `updated` as reactive objects built on Svelte 5 runes,
    /// so they can be read directly instead of through a store subscription.
    ///
    /// Switching modules also changes how the values are read. For example, `$page.url` becomes
    /// `page.url`, and `$updated` becomes `updated.current`. For this reason, the rule doesn't
    /// provide a code fix.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { page } from "$app/stores";
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// import { page } from "$app/stores";
    /// </script>
    ///
    /// <h1>{$page.url.pathname}</h1>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { page } from "$app/state";
    /// ```
    ///
    /// ```svelte
    /// <script>
    /// import { page } from "$app/state";
    /// </script>
    ///
    /// <h1>{page.url.pathname}</h1>
    /// ```
    ///
    pub UseSvelteKitRuneImports {
        version: "next",
        name: "useSvelteKitRuneImports",
        language: "js",
        domains: &[RuleDomain::Svelte],
        recommended: true,
    }
}

impl Rule for UseSvelteKitRuneImports {
    type Query = Ast<AnyJsImportLike>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseSvelteKitRuneImportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let module_name_token = node.module_name_token()?;
        let import_path = node.inner_string_text()?;

        (import_path.text() == DEPRECATED_MODULE).then_some(module_name_token.text_trimmed_range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The "<Emphasis>"$app/stores"</Emphasis>" module is deprecated."
                },
            )
            .note(markup! {
                "SvelteKit deprecated "<Emphasis>"$app/stores"</Emphasis>" in favor of "<Emphasis>"$app/state"</Emphasis>", which provides the same values as reactive objects built on runes."
            })
            .note(markup! {
                "Import from "<Emphasis>"$app/state"</Emphasis>" instead, and read its values directly, for example "<Emphasis>"page.url"</Emphasis>" instead of "<Emphasis>"$page.url"</Emphasis>"."
            }),
        )
    }
}

const DEPRECATED_MODULE: &str = "$app/stores";
