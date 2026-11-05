use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyScssIncludeTarget, CssImportAtRule, CssLanguage, CssLayerAtRule, CssNestedQualifiedRule,
    CssQualifiedRule, CssSyntaxNode, ScssIncludeAtRule, ScssMixinAtRule,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, SyntaxKindSet, TextSize, declare_node_union};
use biome_rule_options::use_layered_styles::UseLayeredStylesOptions;

use crate::utils::scss_function_names_equal;

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
    /// style rule outside a cascade layer, directly or through the mixins it includes. Only mixins
    /// defined in the same file before the outermost `@include` are resolved. Mixins from other modules, such
    /// as `@include theme.button`, aren't checked.
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

        if let AnyUseLayeredStylesQuery::ScssIncludeAtRule(include) = node {
            return includes_unlayered_style_rules(include).then_some(());
        }

        // A containing layer satisfies the rule. Nested rules are covered by their outer style rule.
        layer_deciding_ancestor(node.syntax())
            .is_none()
            .then_some(())
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

fn layer_deciding_ancestor(node: &CssSyntaxNode) -> Option<CssSyntaxNode> {
    node.ancestors()
        .skip(1)
        .find(|ancestor| LAYER_DECIDING_ANCESTOR_KINDS.matches(ancestor.kind()))
}

/// Returns `true` if `include` is outside a cascade layer and includes a mixin that emits a style
/// rule outside a cascade layer.
fn includes_unlayered_style_rules(include: &ScssIncludeAtRule) -> bool {
    layer_deciding_ancestor(include.syntax()).is_none()
        && emits_unlayered_style_rules(include, include.range().start(), &mut Vec::new())
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
) -> bool {
    let Some(mixin) = resolve_mixin(include, called_at) else {
        return false;
    };
    if visited.contains(mixin.syntax()) {
        return false;
    }
    visited.push(mixin.syntax().clone());
    mixin.syntax().descendants().any(|node| {
        let is_style_rule = CssQualifiedRule::can_cast(node.kind())
            || CssNestedQualifiedRule::can_cast(node.kind());
        let nested_include = ScssIncludeAtRule::cast_ref(&node);
        (is_style_rule || nested_include.is_some())
            && layer_deciding_ancestor(&node).as_ref() == Some(mixin.syntax())
            && (is_style_rule
                || nested_include.is_some_and(|nested_include| {
                    emits_unlayered_style_rules(&nested_include, called_at, visited)
                }))
    })
}

/// Returns the mixin defined in the same file that `include` refers to: the last one with its name
/// that is defined before `called_at`, in a block that contains `include`.
///
/// Returns `None` for a mixin from another module, such as `@include theme.button`.
fn resolve_mixin(include: &ScssIncludeAtRule, called_at: TextSize) -> Option<ScssMixinAtRule> {
    let name = match include.name().ok()? {
        AnyScssIncludeTarget::CssIdentifier(name) => name.value_token().ok()?,
        AnyScssIncludeTarget::ScssModuleMemberAccess(_) => return None,
    };
    include
        .syntax()
        .ancestors()
        .last()?
        .descendants()
        .filter_map(ScssMixinAtRule::cast)
        .take_while(|mixin| mixin.range().end() <= called_at)
        .filter(|mixin| {
            mixin.syntax().grand_parent().is_some_and(|scope| {
                include
                    .syntax()
                    .ancestors()
                    .any(|ancestor| ancestor == scope)
            }) && mixin
                .name()
                .ok()
                .and_then(|name| name.value_token().ok())
                .is_some_and(|mixin_name| {
                    scss_function_names_equal(mixin_name.text_trimmed(), name.text_trimmed())
                })
        })
        .last()
}
