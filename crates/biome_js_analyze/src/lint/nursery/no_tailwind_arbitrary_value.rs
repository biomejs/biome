use crate::tailwind::{AnyTailwindClassString, host_range};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_rowan::{TextRange, TextSize};
use biome_rule_options::no_tailwind_arbitrary_value::NoTailwindArbitraryValueOptions;
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use biome_tailwind_syntax::lint_utils::arbitrary_ranges;

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
    /// ```jsx,expect_diagnostic
    /// <div className="w-[400px]" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="text-[#555] bg-white" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="[color:red]" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div className="w-4 text-red-500 bg-white" />;
    /// ```
    ///
    /// ```jsx
    /// <div className="[&:nth-child(3)]:px-2" />;
    /// ```
    ///
    /// The rule checks `class` and `className` JSX attributes, and strings passed to
    /// `clsx`, `tw`, `twMerge`, `twJoin`, `cva`, `tv`, `cn`, `cc`, `cnb`, and `ctl`.
    /// Tagged templates using these helpers, including members such as `tw.div`,
    /// are also checked.
    ///
    pub NoTailwindArbitraryValue {
        version: "2.5.7",
        name: "noTailwindArbitraryValue",
        language: "jsx",
        sources: &[RuleSource::EslintTailwindcss("no-arbitrary-value").same()],
        domains: &[RuleDomain::Tailwind],
        recommended: false,
    }
}

impl Rule for NoTailwindArbitraryValue {
    type Query = TailwindSyntax<AnyTailwindClassString>;
    type State = TextRange;
    type Signals = Vec<TextRange>;
    type Options = NoTailwindArbitraryValueOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        arbitrary_ranges(&query.tailwind_root().candidates(), TextSize::from(0))
            .into_iter()
            .filter_map(|range| host_range(query.node(), range))
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
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
