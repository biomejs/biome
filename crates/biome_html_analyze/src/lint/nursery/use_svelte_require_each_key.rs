use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnySvelteBlockItem, SvelteEachOpeningBlock};
use biome_rowan::AstNode;
use biome_rule_options::use_svelte_require_each_key::UseSvelteRequireEachKeyOptions;

declare_lint_rule! {
    /// Require keyed `{#each}` blocks in Svelte templates.
    ///
    /// Svelte uses keyed each blocks to track list items across updates. Without a key, Svelte
    /// updates items by position, which can cause state to move between items when the list changes.
    ///
    /// For more information, see the Svelte documentation on [keyed each blocks](https://svelte.dev/docs/svelte/each#Keyed-each-blocks).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// {#each items as item}
    ///   <div>{item}</div>
    /// {/each}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// {#each items as item (item.id)}
    ///   <div>{item}</div>
    /// {/each}
    /// ```
    ///
    pub UseSvelteRequireEachKey {
        version: "next",
        name: "useSvelteRequireEachKey",
        language: "html",
        domains: &[RuleDomain::Svelte],
        recommended: true,
        sources: &[RuleSource::EslintSvelte("require-each-key").same()],
    }
}

impl Rule for UseSvelteRequireEachKey {
    type Query = Ast<SvelteEachOpeningBlock>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSvelteRequireEachKeyOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let opening_block = ctx.query();
        let item = opening_block.item()?;

        match item {
            AnySvelteBlockItem::SvelteEachAsKeyedItem(item) if item.key().is_none() => Some(()),
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let opening_block = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                opening_block.range(),
                markup! {
                    "This "<Emphasis>"{#each}"</Emphasis>" block is missing a key."
                },
            )
            .note(markup! {
                "Providing a key helps Svelte track each item when the list changes."
            })
            .note(markup! {
                "Add a unique key using the "<Emphasis>"(key)"</Emphasis>" syntax, for example: "<Emphasis>"{#each items as item (item.id)}"</Emphasis>"."
            }),
        )
    }
}
