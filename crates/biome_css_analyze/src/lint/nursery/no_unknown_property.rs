use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::CssGenericProperty;
use biome_rowan::{AstNode, TextRange};

use crate::utils::{is_known_properties, vendor_prefixed};

declare_lint_rule! {
    /// Disallow unknown properties.
    ///
    /// This rule considers properties defined in the CSS Specifications and browser specific properties to be known.
    /// https://github.com/known-css/known-css-properties#source
    ///
    ///
    /// This rule ignores:
    ///
    /// - custom variables e.g. `--custom-property`
    /// - vendor-prefixed properties (e.g., `-moz-align-self,` `-webkit-align-self`)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   colr: blue;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   my-property: 1;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a {
    ///   color: green;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   fill: black;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   -moz-align-self: center;
    /// }
    /// ```
    ///
    pub NoUnknownProperty {
        version: "1.8.0",
        name: "noUnknownProperty",
        language: "css",
        recommended: true,
        sources: &[RuleSource::Stylelint("property-no-unknown")],
    }
}

impl Rule for NoUnknownProperty {
    type Query = Ast<CssGenericProperty>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let property_name = node.name().ok()?.text().to_lowercase();
        if !property_name.starts_with("--")
            // Ignore `composes` property.
            // See https://github.com/css-modules/css-modules/blob/master/docs/composition.md for more details.
            && property_name != "composes"
            && !is_known_properties(&property_name)
            && !vendor_prefixed(&property_name)
        {
            return Some(node.name().ok()?.range());
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Unknown property is not allowed."
                },
            )
            .note(markup! {
                "See "<Hyperlink href="https://stylelint.io/user-guide/rules/property-no-unknown/">"CSS Specifications and browser specific properties"</Hyperlink>" for more details."
            })
           .note(markup! {
                "To resolve this issue, replace the unknown property with a valid CSS property."
            })
        )
    }
}
