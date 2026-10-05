use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsReferenceIdentifier;
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use biome_rule_options::no_svelte_inspect::NoSvelteInspectOptions;

declare_lint_rule! {
    /// Disallow the use of the `$inspect` rune.
    ///
    /// `$inspect` is a debugging aid: it logs values to the console every time they change.
    /// It only works during development and becomes a no-op in production builds, so calls
    /// to it are usually leftovers from a debugging session that clutter the console and
    /// the code.
    ///
    /// The rule reports every use of `$inspect`, including `$inspect(...).with(...)` and
    /// `$inspect.trace()`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// let count = $state(0);
    /// $inspect(count);
    /// </script>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// $effect(() => {
    ///     $inspect.trace();
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <script>
    /// let count = $state(0);
    /// </script>
    /// ```
    ///
    /// ### References
    ///
    /// - [Svelte `$inspect`](https://svelte.dev/docs/svelte/$inspect)
    pub NoSvelteInspect {
        version: "next",
        name: "noSvelteInspect",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-inspect").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoSvelteInspect {
    type Query = Ast<JsReferenceIdentifier>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoSvelteInspectOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_svelte()
        {
            return None;
        }

        let reference = ctx.query();
        if reference.value_token().ok()?.text_trimmed() != "$inspect" {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let reference = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.range(),
                markup! {
                    "Unexpected use of the "<Emphasis>"$inspect"</Emphasis>" rune."
                },
            )
            .note(markup! {
                <Emphasis>"$inspect"</Emphasis>" is a debugging aid that logs values to the console during development, and is usually left over from a debugging session."
            })
            .note(markup! {
                "Remove the "<Emphasis>"$inspect"</Emphasis>" call once you're done debugging."
            }),
        )
    }
}
