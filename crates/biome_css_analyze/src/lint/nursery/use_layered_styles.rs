use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_semantic::model::SemanticModel;
use biome_css_syntax::{
    CssImportAtRule, CssLanguage, CssLayerAtRule, CssNestedQualifiedRule, CssQualifiedRule,
    CssSyntaxNode, ScssContentAtRule, ScssIncludeAtRule, ScssMixinAtRule,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, SyntaxKindSet, TextSize, declare_node_union};
use biome_rule_options::use_layered_styles::UseLayeredStylesOptions;

use crate::services::semantic::Semantic;

/// Ancestors that decide the layer of a style rule: a layer, an outer style rule that is checked
/// itself, or a mixin body, whose rules take the layer of the place the mixin is included.
const LAYER_DECIDING_ANCESTOR_KINDS: SyntaxKindSet<CssLanguage> = CssLayerAtRule::KIND_SET
    .union(CssQualifiedRule::KIND_SET)
    .union(CssNestedQualifiedRule::KIND_SET)
    .union(ScssMixinAtRule::KIND_SET);

declare_lint_rule! {
    /// Enforce style rules to be defined within a cascade layer.
    ///
    /// This rule reports style rules that are not contained within a cascade layer (`@layer`).
    /// Rules outside of a cascade layer (excluding `!important`) always take precedence over
    /// layered rules, making the cascade more difficult to predict and override.
    ///
    /// ## SCSS limitations
    ///
    /// Layer membership is determined from the authored ancestors of a style rule. A style rule in a
    /// mixin body takes its layer from where the mixin is included, so the rule reports the
    /// `@include` instead: an `@include` outside a cascade layer is reported when the mixin emits a
    /// style rule outside a cascade layer, directly or through the mixins it includes. A style rule
    /// in an `@include` content block is layered when the mixin puts every `@content` inside a
    /// cascade layer. Only mixins defined in the same file before the outermost `@include` are
    /// resolved. Mixins from other modules, such as `@include theme.button`, aren't checked.
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
    type Query = Semantic<AnyUseLayeredStylesQuery>;
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
        if layer_deciding_ancestor(node, node.range().start(), ctx.model()).is_some() {
            return None;
        }

        if let AnyUseLayeredStylesQuery::ScssIncludeAtRule(include) = node {
            return emits_unlayered_style_rules(
                include,
                include.range().start(),
                &mut Vec::new(),
                ctx.model(),
            )
            .then_some(());
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        if let AnyUseLayeredStylesQuery::ScssIncludeAtRule(_) = node {
            return Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "This include emits style rules outside of a cascade layer."
                    },
                )
                .note(markup! {
                    "Style rules outside a cascade layer always take precedence over layered styles, which makes the cascade harder to predict and override."
                })
                .note(markup! {
                    "Wrap the include in a "<Emphasis>"@layer"</Emphasis>" block to control its place in the cascade."
                }),
            );
        }

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
    pub AnyUseLayeredStylesQuery = CssQualifiedRule | CssNestedQualifiedRule | CssImportAtRule | ScssIncludeAtRule
}

fn layer_deciding_ancestor(
    node: &AnyUseLayeredStylesQuery,
    called_at: TextSize,
    model: &SemanticModel,
) -> Option<CssSyntaxNode> {
    node.syntax().ancestors().skip(1).find(|ancestor| {
        LAYER_DECIDING_ANCESTOR_KINDS.matches(ancestor.kind())
            || ScssIncludeAtRule::cast_ref(ancestor)
                .is_some_and(|include| puts_content_in_layer(&include, called_at, model))
    })
}

/// Returns `true` if the mixin that `include` refers to puts every `@content` inside a `@layer`.
fn puts_content_in_layer(
    include: &ScssIncludeAtRule,
    called_at: TextSize,
    model: &SemanticModel,
) -> bool {
    model
        .scss_mixin_for_include(include, called_at)
        .is_some_and(|mixin| {
            let mut contents = mixin
                .syntax()
                .descendants()
                .filter(|node| ScssContentAtRule::can_cast(node.kind()))
                .peekable();
            contents.peek().is_some()
                && contents.all(|content| {
                    content
                        .ancestors()
                        .take_while(|ancestor| ancestor != mixin.syntax())
                        .any(|ancestor| CssLayerAtRule::can_cast(ancestor.kind()))
                })
        })
}

/// Returns `true` if the mixin that `include` resolves to emits a style rule outside a cascade
/// layer, directly or through the mixins it includes.
///
/// Sass resolves the includes in a mixin body when the mixin runs, so every mixin is looked up among
/// the definitions before `called_at`, the position of the outermost `@include`. `visited` holds the
/// mixins being checked, so recursive mixins end the search.
fn emits_unlayered_style_rules(
    include: &ScssIncludeAtRule,
    called_at: TextSize,
    visited: &mut Vec<CssSyntaxNode>,
    model: &SemanticModel,
) -> bool {
    if let Some(mixin) = model.scss_mixin_for_include(include, called_at)
        && !visited.contains(mixin.syntax())
    {
        visited.push(mixin.syntax().clone());
        mixin
            .syntax()
            .descendants()
            .filter_map(AnyUseLayeredStylesQuery::cast)
            .any(|item| {
                layer_deciding_ancestor(&item, called_at, model).as_ref() == Some(mixin.syntax())
                    && match item {
                        AnyUseLayeredStylesQuery::CssQualifiedRule(_)
                        | AnyUseLayeredStylesQuery::CssNestedQualifiedRule(_) => true,
                        AnyUseLayeredStylesQuery::ScssIncludeAtRule(nested) => {
                            emits_unlayered_style_rules(&nested, called_at, visited, model)
                        }
                        AnyUseLayeredStylesQuery::CssImportAtRule(_) => false,
                    }
            })
    } else {
        false
    }
}
