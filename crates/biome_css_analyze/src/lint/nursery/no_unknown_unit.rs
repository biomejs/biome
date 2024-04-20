use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::CssUnknownDimension;
use biome_rowan::TextRange;

declare_rule! {
    /// Disallow unknown units.
    ///
    /// This rule considers units defined in the CSS Specifications, up to and including Editor's Drafts, to be known.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   width: 10pixels;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   width: calc(10px + 10pixels);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a {
    ///   width: 10px;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   width: 10Px;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   width: 10pX;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   width: calc(10px + 10px);
    /// }
    /// ```
    ///
    pub NoUnknownUnit {
        version: "next",
        name: "noUnknownUnit",
        recommended: false,
        sources: &[RuleSource::Stylelint("unit-no-unknown")],
    }
}

pub struct RuleState {
    value: String,
    span: TextRange,
}

fn is_css_hack_unit(value: &str) -> bool {
    value == "\\0"
}

impl Rule for NoUnknownUnit {
    type Query = Ast<CssUnknownDimension>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if let Ok(unit_token) = node.unit_token() {
            let value = unit_token.text().to_string();
            let span = unit_token.text_range();

            if is_css_hack_unit(&value) {
                return None;
            }

            return Some(RuleState { value, span });
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let span = state.span;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected unknown unit: "<Emphasis>{ state.value }</Emphasis>
                },
            )
            .note(markup! {
                "Fix to a known unit."
            }),
        )
    }
}
