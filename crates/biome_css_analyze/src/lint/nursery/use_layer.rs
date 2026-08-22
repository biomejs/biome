use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{CssQualifiedRule, CssSyntaxKind};
use biome_diagnostics::Severity;
use biome_rowan::AstNode;
use biome_rule_options::use_layer::UseLayerOptions;

declare_lint_rule! {
    /// Enforce style rules to be defined within a cascade layer.
    ///
    /// This rule reports style rules that are not contained within a cascade layer (`@layer`).
    /// Rules outside of a cascade layer (excluding `!important`) always take precedence over
    /// layered rules, making the cascade more difficult to predict and override.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// .my-style {
    ///   color: red;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @media (min-width: 600px) {
    ///   .my-style {
    ///     color: red;
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @layer base {
    ///   .my-style {
    ///     color: red;
    ///   }
    /// }
    /// ```
    ///
    /// ```css
    /// @layer base {
    ///   @media (min-width: 600px) {
    ///     .my-style {
    ///       color: red;
    ///     }
    ///   }
    /// }
    /// ```
    ///
    pub UseLayer {
        version: "next",
        name: "useLayer",
        language: "css",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintCss("use-layers").inspired()],
    }
}

impl Rule for UseLayer {
    type Query = Ast<CssQualifiedRule>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseLayerOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        for ancestor in node.syntax().ancestors().skip(1) {
            match ancestor.kind() {
                // The rule is contained within a cascade layer.
                CssSyntaxKind::CSS_LAYER_AT_RULE => return None,
                // The rule is nested inside another style rule, which is reported
                // on its own when it is outside a layer.
                CssSyntaxKind::CSS_QUALIFIED_RULE | CssSyntaxKind::CSS_NESTED_QUALIFIED_RULE => {
                    return None;
                }
                _ => {}
            }
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This style rule is defined outside of a cascade layer."
                },
            )
            .note(markup! {
                "Style rules outside a cascade layer always take precedence over layered rules, which makes the cascade harder to predict and override."
            })
            .note(markup! {
                "Wrap the style rule in a "<Emphasis>"@layer"</Emphasis>" block to control its place in the cascade."
            }),
        )
    }
}
