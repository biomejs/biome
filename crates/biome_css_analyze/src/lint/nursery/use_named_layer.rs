use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{CssImportAnonymousLayer, CssLayerAtRule};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, AstSeparatedList, declare_node_union};
use biome_rule_options::use_named_layer::UseNamedLayerOptions;

declare_lint_rule! {
    /// Disallow anonymous cascade layers.
    ///
    /// A cascade layer created with `@layer { ... }` or imported with
    /// `@import "..." layer` has no name. Anonymous layers get their own place
    /// in the cascade order, but because they cannot be referred to by name,
    /// no later rule can add styles to them or reorder them. This makes the
    /// cascade harder to reason about and prevents reusing the layer.
    ///
    /// Give every layer a name so it can be referenced, appended to, and
    /// ordered explicitly through a `@layer` statement.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// @layer {
    ///   a {
    ///     color: red;
    ///   }
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @import "theme.css" layer;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @layer base {
    ///   a {
    ///     color: red;
    ///   }
    /// }
    /// ```
    ///
    /// ```css
    /// @import "theme.css" layer(base);
    /// ```
    ///
    pub UseNamedLayer {
        version: "2.5.9",
        name: "useNamedLayer",
        language: "css",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintCss("use-layers").inspired()],
    }
}

impl Rule for UseNamedLayer {
    type Query = Ast<AnyUseNamedLayerQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseNamedLayerOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            AnyUseNamedLayerQuery::CssImportAnonymousLayer(_) => return Some(()),
            AnyUseNamedLayerQuery::CssLayerAtRule(at_rule) => {
                let layer = at_rule.layer().ok()?;
                let dec = layer.as_css_layer_declaration()?;
                if dec.references().is_empty() {
                    return Some(());
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Missing cascade layer name."
                },
            )
            .note(markup! {
                "An anonymous layer cannot be referenced, so later rules cannot append to it or reorder it."
            })
            .note(markup! {
                "Give the layer a name, for example "<Emphasis>"@layer base { ... }"</Emphasis>"."
            }),
        )
    }
}

declare_node_union! {
    pub AnyUseNamedLayerQuery = CssLayerAtRule | CssImportAnonymousLayer
}
