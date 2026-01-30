use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::CssColor;
use biome_rowan::AstNode;
use biome_rule_options::no_hex_colors::NoHexColorsOptions;

declare_lint_rule! {
    /// Disallow hex colors.
    ///
    /// While hex colors are widely supported and compact, they can be less readable
    /// and have limitations in terms of color representation compared to color models
    /// like HSL or OKLCH. This rule encourages the use of more expressive color formats.
    ///
    /// This rule is inspired by the Stylelint rule
    /// [`color-no-hex`](https://stylelint.io/user-guide/rules/color-no-hex/).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a { color: #000; }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { color: #fff1aa; }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { color: #123456aa; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { color: black; }
    /// ```
    ///
    /// ```css
    /// a { color: rgb(0, 0, 0); }
    /// ```
    ///
    /// ### References
    ///
    /// - [MDN Web Docs on CSS color values](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/color_value)
    ///
    pub NoHexColors {
        version: "next",
        name: "noHexColors",
        language: "css",
        sources: &[RuleSource::Stylelint("color-no-hex").same()],
        recommended: false,
    }
}

impl Rule for NoHexColors {
    type Query = Ast<CssColor>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoHexColorsOptions;

    fn run(_ctx: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Unexpected hex color."
                },
            )
            .note(markup! {
                "Hex colors are less readable and have limitations compared to other color models."
            })
            .note(markup! {
                "Consider using a named color or a color function like rgb(), hsl() or oklch(). See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/color_value">"MDN Web Docs on CSS color values"</Hyperlink>" for more information."
            }),
        )
    }
}
