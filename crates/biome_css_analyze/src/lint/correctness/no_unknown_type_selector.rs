use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssAtRule, CssCompoundSelector, CssNestedQualifiedRule, CssPseudoElementFunctionSelector,
    CssQualifiedRule, CssRelativeSelector, CssSyntaxKind, CssTypeSelector, ScssEachAtRule,
    ScssForAtRule, ScssIfAtRule, ScssIncludeAtRule, ScssMixinAtRule, ScssWhileAtRule,
};
use biome_diagnostics::Severity;
use biome_languages::CssFileSource;
use biome_rowan::AstNode;
use biome_rule_options::no_unknown_type_selector::NoUnknownTypeSelectorOptions;

use crate::utils::is_known_type_selector;

const VIEW_TRANSITION_PSEUDO_ELEMENTS: [&str; 5] = [
    "view-transition",
    "view-transition-group",
    "view-transition-image-pair",
    "view-transition-old",
    "view-transition-new",
];

/// Returns whether the type selector is the argument of a view transition
/// pseudo-element, such as `page` in `::view-transition-group(page)`.
///
/// That argument names a `view-transition-name` rather than an element, so it
/// can be any author-defined identifier.
fn is_in_view_transition_pseudo_element(type_selector: &CssTypeSelector) -> bool {
    // According to the AST structure: CssTypeSelector -> CssCompoundSelector -> CssPseudoElementFunctionSelector
    type_selector
        .syntax()
        .grand_parent()
        .and_then(CssPseudoElementFunctionSelector::cast)
        .and_then(|func| func.name().ok())
        .and_then(|name| name.value_token().ok())
        .is_some_and(|token| {
            let name = token.text_trimmed();
            VIEW_TRANSITION_PSEUDO_ELEMENTS
                .iter()
                .any(|pseudo_element| pseudo_element.eq_ignore_ascii_case(name))
        })
}

/// Returns whether the type selector is the whole selector `from` or `to` of a rule placed
/// directly in a mixin body or an `@include` content block, where it becomes a keyframe selector
/// once Sass places it inside `@keyframes`.
///
/// Control-flow at-rules such as `@if` between the rule and the block don't change where Sass puts
/// the rule. An enclosing style rule or another at-rule, such as `@media`, does.
fn is_keyframe_selector_in_scss_block(type_selector: &CssTypeSelector, name: &str) -> bool {
    if !(name.eq_ignore_ascii_case("from") || name.eq_ignore_ascii_case("to"))
        || type_selector.namespace().is_some()
    {
        return false;
    }
    // Equal ranges rule out sibling selectors and combinators without allocating child nodes.
    let range = type_selector.syntax().text_trimmed_range();
    let Some(mut selector) = type_selector.syntax().parent().filter(|node| {
        CssCompoundSelector::can_cast(node.kind()) && node.text_trimmed_range() == range
    }) else {
        return false;
    };
    if let Some(relative) = selector
        .parent()
        .filter(|node| CssRelativeSelector::can_cast(node.kind()))
    {
        if relative.text_trimmed_range() != range {
            return false;
        }
        selector = relative;
    }
    let Some(rule) = selector
        .grand_parent()
        .filter(|rule| is_style_rule(rule.kind()))
    else {
        return false;
    };
    rule.ancestors()
        .skip(1)
        .find_map(|node| {
            let kind = node.kind();
            if ScssMixinAtRule::can_cast(kind) || ScssIncludeAtRule::can_cast(kind) {
                Some(true)
            } else if is_style_rule(kind)
                || (AnyCssAtRule::can_cast(kind) && !is_control_flow(kind))
            {
                Some(false)
            } else {
                None
            }
        })
        .unwrap_or(false)
}

fn is_control_flow(kind: CssSyntaxKind) -> bool {
    ScssIfAtRule::can_cast(kind)
        || ScssEachAtRule::can_cast(kind)
        || ScssForAtRule::can_cast(kind)
        || ScssWhileAtRule::can_cast(kind)
}

fn is_style_rule(kind: CssSyntaxKind) -> bool {
    CssQualifiedRule::can_cast(kind) || CssNestedQualifiedRule::can_cast(kind)
}

declare_lint_rule! {
    /// Disallow unrecognized element names in type selectors.
    ///
    /// A type selector targets elements by tag name, as in `button {}`. Names from the HTML, SVG,
    /// and MathML specifications are recognized. Valid custom-element names such as `x-button` are
    /// also allowed.
    ///
    /// See the [MDN type-selector reference](https://developer.mozilla.org/en-US/docs/Web/CSS/Type_selectors)
    /// and the element references for [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML/Element),
    /// [SVG](https://developer.mozilla.org/en-US/docs/Web/SVG/Element), and
    /// [MathML](https://developer.mozilla.org/en-US/docs/Web/MathML/Element).
    ///
    /// ## SCSS limitations
    ///
    /// Type selector names containing SCSS interpolation are ignored because the emitted name cannot
    /// be determined statically.
    ///
    /// `from` and `to` aren't reported as the whole selector of a rule directly inside a `@mixin`
    /// body or an `@include` block, because Sass can place them inside `@keyframes`, where they are
    /// keyframe selectors.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// unknown {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// unknown > ul {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// x-Foo {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// input {}
    /// ```
    ///
    /// ```css
    /// ul > li {}
    /// ```
    ///
    /// ```css
    /// x-foo {}
    /// ```
    ///
    pub NoUnknownTypeSelector {
        version: "1.9.4",
        name: "noUnknownTypeSelector",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("selector-type-no-unknown").same()],
    }
}

impl Rule for NoUnknownTypeSelector {
    type Query = Ast<CssTypeSelector>;
    type State = CssTypeSelector;
    type Signals = Option<Self::State>;
    type Options = NoUnknownTypeSelectorOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let css_type_selector = ctx.query();

        if is_in_view_transition_pseudo_element(css_type_selector) {
            return None;
        }

        let type_selector = css_type_selector
            .ident()
            .ok()?
            .as_css_identifier()?
            .value_token()
            .ok()?
            .token_text_trimmed();
        if is_known_type_selector(&type_selector) {
            return None;
        }
        if ctx.source_type::<CssFileSource>().is_scss()
            && is_keyframe_selector_in_scss_block(css_type_selector, &type_selector)
        {
            return None;
        }
        Some(css_type_selector.clone())
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let span = node.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unknown type selector is not allowed."
                },
            )
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/Type_selectors">"MDN web docs"</Hyperlink>" for more details."
            })
            .note(markup! {
                "A selector for an element that does not exist never matches, so its styles are never applied."
            })
            .note(markup! {
                "Consider replacing the unknown type selector with valid one."
            }),
        )
    }
}
