use crate::tailwind::host_range;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::HtmlAttribute;
use biome_rowan::TextRange;
use biome_rule_options::no_tailwind_arbitrary_value::NoTailwindArbitraryValueOptions;
use biome_tailwind_logic::no_tailwind_arbitrary_value::analyze_tailwind_arbitrary_values;
use biome_tailwind_logic::syntax_service::TailwindSyntax;

declare_lint_rule! {
    /// Disallow arbitrary values in Tailwind CSS utility classes.
    ///
    /// Arbitrary values (e.g. `w-[400px]`, `text-[#555]`) and arbitrary properties
    /// (e.g. `[color:red]`) bypass Tailwind's configured theme scales. This rule reports
    /// them so teams can keep styling constrained to named utilities from their Tailwind
    /// configuration.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div class="w-[400px]"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div class="text-[#555] bg-white"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div class="[color:red]"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="w-4 text-red-500 bg-white"></div>
    /// ```
    ///
    /// ```html
    /// <div class="[&:nth-child(3)]:px-2"></div>
    /// ```
    ///
    /// ## Tailwind configuration
    ///
    /// Use the top-level `tailwind` configuration to control which attributes
    /// contain Tailwind classes. Specified arrays replace the defaults.
    ///
    /// ```json
    /// {
    ///     "tailwind": {
    ///         "attributes": ["class", "classList"]
    ///     }
    /// }
    /// ```
    ///
    pub NoTailwindArbitraryValue {
        version: "2.5.7",
        name: "noTailwindArbitraryValue",
        language: "html",
        sources: &[RuleSource::EslintTailwindcss("no-arbitrary-value").same()],
        domains: &[RuleDomain::Tailwind],
        recommended: false,
    }
}

impl Rule for NoTailwindArbitraryValue {
    type Query = TailwindSyntax<HtmlAttribute>;
    type State = TextRange;
    type Signals = Vec<TextRange>;
    type Options = NoTailwindArbitraryValueOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        analyze_tailwind_arbitrary_values(&ctx.query().tailwind_root().candidates())
    }

    fn diagnostic(ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                host_range(ctx.query().node(), *range)?,
                markup! { "Found an arbitrary value in a Tailwind CSS class." },
            )
            .note(markup! {
                "Arbitrary values bypass Tailwind's theme configuration, defeating design-system consistency and making styles harder to refactor."
            })
            .note(markup! {
                "Use a named utility from your Tailwind configuration instead."
            }),
        )
    }
}
