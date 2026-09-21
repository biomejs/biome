use crate::tailwind::{AnyTailwindClassString, host_range};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_rowan::TextRange;
use biome_rule_options::no_tailwind_raw_colors::NoTailwindRawColorsOptions;
use biome_tailwind_logic::no_tailwind_raw_colors::raw_color_ranges;
use biome_tailwind_logic::syntax_service::TailwindSyntax;

declare_lint_rule! {
    /// Disallow Tailwind CSS utility classes that use raw palette colors.
    ///
    /// Palette colors such as `pink-500` and `slate-950` tie styles to specific
    /// colors instead of the role those colors play. Design system color names such as `primary`
    /// or `muted` let a design system change its palette without editing each component.
    /// This rule checks color utilities, including variants and opacity modifiers.
    /// It allows custom color names, `black`, `white`, `transparent`, `current`, and `inherit`.
    /// This rule does not check arbitrary values such as `bg-[#ff00aa]`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="bg-pink-500" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="hover:text-red-500/80" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div className="bg-primary hover:text-muted-foreground" />;
    /// ```
    ///
    /// ```jsx
    /// <div className="bg-white text-black border-transparent fill-current stroke-inherit" />;
    /// ```
    ///
    /// ## Supported class strings
    ///
    /// The rule checks `class` and `className` JSX attributes and string arguments
    /// to `clsx`, `tw`, `twMerge`, `twJoin`, `cva`, `tv`, `cn`, `cc`, `cnb`, and `ctl`.
    /// Tagged templates using these names, including members such as `tw.div`,
    /// are also checked. Static template chunks and class expressions in JSX,
    /// Svelte, Vue, and Astro attributes are checked. Dynamically constructed
    /// class names are not resolved.
    /// The rule does not read your Tailwind configuration; redefining a default
    /// palette name does not exempt it.
    ///
    /// ## Options
    ///
    /// ### allowedColors
    ///
    /// Default: `[]`
    ///
    /// Exact palette colors to allow.
    /// Entries are case-sensitive color names, such as `slate-950` or `pink-500`,
    /// without utility prefixes, variants, or opacity modifiers.
    /// Allowing `pink-500` permits it in every color utility, including variants
    /// and opacity modifiers; it does not allow other pink shades.
    ///
    /// ```json,options
    /// { "options": { "allowedColors": ["pink-500"] } }
    /// ```
    ///
    /// ```jsx,use_options
    /// <div className="bg-pink-500 hover:text-pink-500/80 border-black" />;
    /// ```
    ///
    /// ## See Also
    ///
    /// - [noTailwindArbitraryValue](https://biomejs.dev/linter/rules/no-tailwind-arbitrary-value/)
    ///
    pub NoTailwindRawColors {
        version: "next",
        name: "noTailwindRawColors",
        language: "jsx",
        domains: &[RuleDomain::Tailwind],
        recommended: false,
        sources: &[RuleSource::EslintShadcn("no-raw-colors").inspired()],
        issue_number: Some("11342"),
    }
}

impl Rule for NoTailwindRawColors {
    type Query = TailwindSyntax<AnyTailwindClassString>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoTailwindRawColorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if ctx.query().tailwind_has_errors() {
            return Box::default();
        }
        let root = ctx.query().tailwind_root();
        raw_color_ranges(&root.candidates(), ctx.options()).into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                host_range(ctx.query().node(), *range)?,
                markup! { "This Tailwind CSS class uses a raw palette color." },
            )
            .note(markup! {
                "Raw palette colors couple components to specific colors, making design-system changes harder."
            })
            .note(markup! {
                "Use a color from your design system. Check your Tailwind config for the available colors."
            }),
        )
    }
}
