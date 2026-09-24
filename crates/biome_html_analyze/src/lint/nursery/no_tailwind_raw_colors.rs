use crate::tailwind::host_range;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::HtmlAttribute;
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
    /// ```html,expect_diagnostic
    /// <div class="bg-pink-500"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div class="hover:text-red-500/80"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="bg-primary hover:text-muted-foreground"></div>
    /// ```
    ///
    /// ```html
    /// <div class="bg-white text-black border-transparent fill-current stroke-inherit"></div>
    /// ```
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
    /// ```html,use_options
    /// <div class="bg-pink-500 hover:text-pink-500/80 border-black"></div>
    /// ```
    ///
    /// ## See Also
    ///
    /// - [noTailwindArbitraryValue](https://biomejs.dev/linter/rules/no-tailwind-arbitrary-value/)
    ///
    pub NoTailwindRawColors {
        version: "next",
        name: "noTailwindRawColors",
        language: "html",
        domains: &[RuleDomain::Tailwind],
        recommended: false,
        sources: &[RuleSource::EslintShadcn("no-raw-colors").inspired()],
        issue_number: Some("11342"),
    }
}

impl Rule for NoTailwindRawColors {
    type Query = TailwindSyntax<HtmlAttribute>;
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
