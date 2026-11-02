use crate::tailwind::host_range;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::HtmlAttribute;
use biome_rule_options::no_tailwind_raw_colors::NoTailwindRawColorsOptions;
use biome_tailwind_logic::no_tailwind_raw_colors::{RawColor, raw_colors};
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use biome_tailwind_logic::use_tailwind_sorted_classes::TailwindDesignSystem;
use std::sync::Arc;

declare_lint_rule! {
    /// Disallow Tailwind CSS utility classes that use raw palette colors.
    ///
    /// Palette colors such as `pink-500` and `slate-950` tie styles to specific
    /// colors instead of the role those colors play. Design system color names such as `primary`
    /// or `muted` let a design system change its palette without editing each component.
    /// This rule checks color utilities, including variants and opacity modifiers.
    /// It allows custom color names, `transparent`, `current`, and `inherit`.
    /// It also allows `black` and `white`, unless the `allowBlackAndWhite` option is turned off.
    /// This rule does not check arbitrary values such as `bg-[#ff00aa]`.
    /// Giving a default palette name another value in your Tailwind CSS theme
    /// does not exempt it.
    ///
    /// When the [`tailwind.stylesheet`](https://biomejs.dev/reference/configuration/#tailwindstylesheet)
    /// option points at the CSS file that holds your Tailwind CSS configuration, the rule
    /// also lists up to three classes to use instead, such as `bg-brand` for `bg-pink-500`.
    /// Each one sets a color from your theme that looks similar to the reported one.
    /// It can only compare theme colors whose value is written out, such as
    /// `--color-brand: #00f`. It skips colors that refer to another value, such as
    /// `--color-brand: var(--brand)`.
    ///
    /// Biome scans your project when this rule is enabled, so that it can read the
    /// stylesheet and the files it imports. The scan happens even if `tailwind.stylesheet`
    /// isn't set.
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
    /// ## Recognized class strings
    ///
    /// This rule checks the attributes and functions recognized by the top-level
    /// [`tailwind` configuration](https://biomejs.dev/reference/configuration/#tailwind).
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
    /// ### allowBlackAndWhite
    ///
    /// Default: `true`
    ///
    /// Whether to allow `black` and `white`.
    /// Set it to `false` to report them like other palette colors, so that black and
    /// white also come from your design system, such as `bg-surface` instead of `bg-white`.
    /// You can still allow one of them with `allowedColors`, such as `["white"]`.
    ///
    /// ```json,options
    /// { "options": { "allowBlackAndWhite": false } }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```html,expect_diagnostic,use_options
    /// <div class="hover:bg-white/80"></div>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```html,use_options
    /// <div class="bg-surface text-foreground border-transparent fill-current"></div>
    /// ```
    ///
    /// ## See Also
    ///
    /// - [noTailwindArbitraryValue](https://biomejs.dev/linter/rules/no-tailwind-arbitrary-value/)
    ///
    pub NoTailwindRawColors {
        version: "2.5.15",
        name: "noTailwindRawColors",
        language: "html",
        domains: &[RuleDomain::Tailwind, RuleDomain::Project],
        recommended: false,
        sources: &[RuleSource::EslintShadcn("no-raw-colors").inspired()],
        issue_number: Some("11342"),
    }
}

impl Rule for NoTailwindRawColors {
    type Query = TailwindSyntax<HtmlAttribute>;
    type State = RawColor;
    type Signals = Box<[Self::State]>;
    type Options = NoTailwindRawColorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if ctx.query().tailwind_has_errors() {
            return Box::default();
        }
        let root = ctx.query().tailwind_root();
        raw_colors(&root.candidates(), ctx.options()).into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, raw_color: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            host_range(ctx.query().node(), raw_color.range())?,
            markup! { "This Tailwind CSS class uses a raw palette color." },
        )
        .note(markup! {
            "Raw palette colors couple components to specific colors, making design-system changes harder."
        });
        let design: &TailwindDesignSystem = match ctx.get_service::<Arc<TailwindDesignSystem>>() {
            Some(design) => design,
            None => TailwindDesignSystem::default_ref(),
        };
        let closest_classes = raw_color.closest_design_classes(design, ctx.options());
        Some(if closest_classes.is_empty() {
            diagnostic.note(markup! {
                "Use a color from your design system. Check your Tailwind config for the available colors."
            })
        } else {
            diagnostic.footer_list(
                "Use a color from your design system. These classes are the closest matches:",
                closest_classes,
            )
        })
    }
}
