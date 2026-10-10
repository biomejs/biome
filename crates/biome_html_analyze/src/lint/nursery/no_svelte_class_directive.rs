use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::SvelteClassDirective;
use biome_rowan::AstNode;
use biome_rule_options::no_svelte_class_directive::NoSvelteClassDirectiveOptions;

declare_lint_rule! {
    /// Disallow Svelte's `class:` directive.
    ///
    /// The `class:` directive adds one class to an element when a condition is true.
    /// Since Svelte 5.16, the `class` attribute accepts objects and arrays, which can do the same.
    /// Svelte recommends the `class` attribute, because an object or an array can add several
    /// classes with one condition, and can be combined with other class values, such as the
    /// classes that a component receives from its parent.
    ///
    /// Objects and arrays in the `class` attribute need Svelte 5.16 or later.
    /// This rule doesn't check which Svelte version your project uses,
    /// so turn it off if your project uses an older version.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <div class:active={isActive}></div>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <div class:active></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <div class={{ active: isActive }}></div>
    /// <div class={{ active }}></div>
    /// <div class={["button", isActive && "active"]}></div>
    /// <div class={isActive ? "active" : "inactive"}></div>
    /// ```
    ///
    /// ### References
    ///
    /// - [Svelte `class` documentation](https://svelte.dev/docs/svelte/class)
    ///
    pub NoSvelteClassDirective {
        version: "next",
        name: "noSvelteClassDirective",
        language: "html",
        domains: &[RuleDomain::Svelte],
        recommended: true,
    }
}

impl Rule for NoSvelteClassDirective {
    type Query = Ast<SvelteClassDirective>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoSvelteClassDirectiveOptions;

    fn run(_ctx: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"class:"</Emphasis>" directive can be replaced with the "<Emphasis>"class"</Emphasis>" attribute."
                },
            )
            .note(markup! {
                "Since Svelte 5.16, the "<Emphasis>"class"</Emphasis>" attribute accepts objects and arrays. Unlike the directive, they can add several classes with one condition and can be combined with other class values."
            })
            .note(markup! {
                "Move the class into the element's "<Emphasis>"class"</Emphasis>" attribute, using an object such as "<Emphasis>"class={{ active: isActive }}"</Emphasis>"."
            }),
        )
    }
}
