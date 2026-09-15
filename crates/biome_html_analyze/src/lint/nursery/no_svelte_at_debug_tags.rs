use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::SvelteDebugBlock;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_svelte_at_debug_tags::NoSvelteAtDebugTagsOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Disallow the use of Svelte's `{@debug}` tag.
    ///
    /// The `{@debug}` tag is a debugging aid that logs the values of the given variables to the
    /// console whenever they change, and pauses execution when developer tools are open. It
    /// should be removed once you are done debugging, as it should not remain in production code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// {@debug user}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// {user}
    /// ```
    ///
    /// ### References
    ///
    /// - [Svelte debug tag](https://svelte.dev/docs/svelte/@debug)
    ///
    pub NoSvelteAtDebugTags {
        version: "2.5.14",
        name: "noSvelteAtDebugTags",
        language: "html",
        domains: &[RuleDomain::Svelte],
        recommended: true,
        sources: &[RuleSource::EslintSvelte("no-at-debug-tags").same()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoSvelteAtDebugTags {
    type Query = Ast<SvelteDebugBlock>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoSvelteAtDebugTagsOptions;

    fn run(_ctx: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Unexpected "<Emphasis>"{@debug}"</Emphasis>" tag."
                },
            )
            .note(markup! {
                "The "<Emphasis>"{@debug}"</Emphasis>" tag should be removed once you no longer need it for debugging."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(node.clone());

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"{@debug}"</Emphasis>" tag." }.to_owned(),
            mutation,
        ))
    }
}
