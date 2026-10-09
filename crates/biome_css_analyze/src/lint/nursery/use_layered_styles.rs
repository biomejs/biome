use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    CssImportAtRule, CssLanguage, CssLayerAtRule, CssNestedQualifiedRule, CssQualifiedRule,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, SyntaxKindSet, declare_node_union};
use biome_rule_options::use_layered_styles::UseLayeredStylesOptions;

const LAYERED_STYLE_ANCESTOR_KINDS: SyntaxKindSet<CssLanguage> = CssLayerAtRule::KIND_SET
    .union(CssQualifiedRule::KIND_SET)
    .union(CssNestedQualifiedRule::KIND_SET);

declare_lint_rule! {
    /// Enforce style rules to be defined within a cascade layer.
    ///
    /// This rule reports style rules that are not contained within a cascade layer (`@layer`).
    /// Rules outside of a cascade layer (excluding `!important`) always take precedence over
    /// layered rules, making the cascade more difficult to predict and override.
    ///
    /// ## SCSS limitations
    ///
    /// Layer membership is determined from the authored ancestors of a style rule. SCSS mixins and
    /// includes are not expanded, so a rule in a mixin is checked at its definition site rather than
    /// at the layer where the mixin may be included.
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
    /// ```css,expect_diagnostic
    /// @import "foo.css";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @layer {
    ///   .my-style {
    ///     color: red;
    ///   }
    /// }
    /// ```
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
    /// ```css
    /// @import "foo.css" layer;
    /// ```
    ///
    /// ```css
    /// @import "foo.css" layer(base);
    /// ```
    ///
    /// ## Options
    ///
    /// ### `requireImportLayers`
    ///
    /// Whether `@import` rules must specify a cascade layer.
    ///
    /// When set to `false`, `@import` rules without a `layer` keyword are allowed.
    ///
    /// Default: `true`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "requireImportLayers": false
    ///   }
    /// }
    /// ```
    ///
    /// ```css,use_options
    /// @import "foo.css";
    /// ```
    ///
    pub UseLayeredStyles {
        version: "2.5.13",
        name: "useLayeredStyles",
        language: "css",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintCss("use-layers").inspired()],
    }
}

impl Rule for UseLayeredStyles {
    type Query = Ast<AnyUseLayeredStylesQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseLayeredStylesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if let AnyUseLayeredStylesQuery::CssImportAtRule(import) = node {
            if !ctx.options().require_import_layers() {
                return None;
            }
            return import.layer().is_none().then_some(());
        }

        // A containing layer satisfies the rule. Nested rules are covered by their outer style rule.
        for ancestor in node.syntax().ancestors().skip(1) {
            if LAYERED_STYLE_ANCESTOR_KINDS.matches(ancestor.kind()) {
                return None;
            }
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        if let AnyUseLayeredStylesQuery::CssImportAtRule(_) = node {
            Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "This import is defined outside of a cascade layer."
                    },
                )
                .note(markup! {
                    "Imported styles without a cascade layer always take precedence over layered styles, which makes the cascade harder to predict and override."
                })
                .note(markup! {
                    "Add the "<Emphasis>"layer"</Emphasis>" keyword after the import to control its place in the cascade."
                }),
            )
        } else {
            Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "This style rule is defined outside of a cascade layer."
                    },
                )
                .note(markup! {
                    "Style rules outside a cascade layer always take precedence over layered styles, which makes the cascade harder to predict and override."
                })
                .note(markup! {
                    "Wrap the style rule in a "<Emphasis>"@layer"</Emphasis>" block to control its place in the cascade."
                }),
            )
        }
    }
}

declare_node_union! {
    pub AnyUseLayeredStylesQuery = CssQualifiedRule | CssNestedQualifiedRule | CssImportAtRule
}
