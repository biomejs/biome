use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::SvelteConstBlock;
use biome_rowan::AstNode;
use biome_rule_options::no_svelte_legacy_const::NoSvelteLegacyConstOptions;

declare_lint_rule! {
    /// Disallow legacy Svelte `{@const}` tags.
    ///
    /// Declaration tags provide the current syntax for deriving values in Svelte markup (available since Svelte 5.56).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// {#each boxes as box}
    ///     {@const area = box.width * box.height}
    ///     <p>{area}</p>
    /// {/each}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// {#each boxes as box}
    ///     {const area = $derived(box.width * box.height)}
    ///     <p>{area}</p>
    /// {/each}
    /// ```
    ///
    /// ### References
    ///
    /// - [Svelte declaration tags](https://svelte.dev/docs/svelte/declaration-tags)
    pub NoSvelteLegacyConst {
        version: "2.5.8",
        name: "noSvelteLegacyConst",
        language: "html",
        domains: &[RuleDomain::Svelte],
        recommended: false,
    }
}

impl Rule for NoSvelteLegacyConst {
    type Query = Ast<SvelteConstBlock>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoSvelteLegacyConstOptions;

    fn run(_ctx: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Avoid legacy Svelte "<Emphasis>"{@const}"</Emphasis>" tags."
                },
            )
            .note(markup! {
                <Emphasis>"{@const}"</Emphasis>" legacy syntax is less versatile. "<Hyperlink href="https://svelte.dev/docs/svelte/declaration-tags">"Declaration tags"</Hyperlink>" are the preferred syntax for deriving values in Svelte markup."
            })
            .note(markup! {
                "Remove the "<Emphasis>"@"</Emphasis>" and add "<Emphasis>"$state()"</Emphasis>" or "<Emphasis>"$derived()"</Emphasis>" runes if the value should be reactive, for example: "<Emphasis>"{const area = $derived(box.width * box.height)}"</Emphasis>"."
            }),
        )
    }
}
