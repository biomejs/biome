use crate::tailwind::{AnyTailwindClassString, host_range};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_rowan::TextRange;
use biome_rule_options::no_tailwind_arbitrary_value::NoTailwindArbitraryValueOptions;
use biome_tailwind_logic::no_tailwind_arbitrary_value::arbitrary_value_ranges;
use biome_tailwind_logic::syntax_service::TailwindSyntax;

declare_lint_rule! {
    /// Disallow arbitrary values in Tailwind CSS utility classes.
    ///
    /// Arbitrary values (e.g. `w-[400px]`, `text-[#555]`) and arbitrary properties
    /// (e.g. `[color:red]`) bypass Tailwind's configured theme scales. This rule reports
    /// them so teams can keep styling constrained to named utilities from their Tailwind
    /// configuration.
    ///
    /// Arbitrary variants, such as `[&_svg]:size-4`, and arbitrary modifiers, such as
    /// `/[0.5]` in `bg-black/[0.5]`, are not reported.
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
    /// <div className="[&:nth-child(3)]:px-2 bg-black/[0.5]" />;
    /// ```
    ///
    /// ## Recognized class strings
    ///
    /// This rule checks the attributes and functions recognized by the top-level
    /// [`tailwind` configuration](https://biomejs.dev/reference/configuration/#tailwind).
    ///
    /// ## Options
    ///
    /// ### allowedCategories
    ///
    /// Default: `[]`
    ///
    /// Categories of utilities that may use arbitrary values:
    ///
    /// - `layout`, such as `w-[320px]`, `m-[13px]`, and `top-[3px]`
    /// - `color`, such as `bg-[#333]`
    /// - `typography`, such as `text-[13px]`
    /// - `spacing`, such as `p-[13px]` and `gap-[3px]`
    /// - `shape`, such as `rounded-[10px]` and `border-[3px]`
    /// - `effects`, such as `shadow-[0_1px_2px_black]` and `opacity-[0.15]`
    /// - `motion`, such as `duration-[250ms]`
    ///
    /// An arbitrary property belongs to the category of the CSS property it sets, so `[padding:13px]` is in `spacing`.
    ///
    /// ```json,options
    /// { "options": { "allowedCategories": ["layout"] } }
    /// ```
    ///
    /// ```jsx,use_options
    /// <div className="w-[320px] m-[13px]" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// <div className="p-[13px]" />;
    /// ```
    ///
    /// ### allowedClasses
    ///
    /// Default: `[]`
    ///
    /// Classes that may use arbitrary values. Write each class without variants or `!`.
    /// An allowed class is also allowed with variants and `!`, so allowing `p-[13px]`
    /// also allows `md:p-[13px]` and `p-[13px]!`.
    ///
    /// ```json,options
    /// { "options": { "allowedClasses": ["p-[13px]"] } }
    /// ```
    ///
    /// ```jsx,use_options
    /// <div className="p-[13px] md:p-[13px]" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// <div className="p-[15px]" />;
    /// ```
    ///
    pub NoTailwindArbitraryValue {
        version: "2.5.7",
        name: "noTailwindArbitraryValue",
        language: "jsx",
        sources: &[
            RuleSource::EslintTailwindcss("no-arbitrary-value").same(),
            RuleSource::EslintShadcn("no-arbitrary-values").inspired(),
        ],
        domains: &[RuleDomain::Tailwind],
        recommended: false,
    }
}

impl Rule for NoTailwindArbitraryValue {
    type Query = TailwindSyntax<AnyTailwindClassString>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoTailwindArbitraryValueOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        arbitrary_value_ranges(&query.tailwind_root().candidates(), ctx.options())
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
