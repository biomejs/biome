use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlElement, AnyVueDirective, HtmlElement, HtmlOpeningElement, HtmlSyntaxKind, T,
    element_ext::AnyHtmlTagElement,
};
use biome_languages::HtmlFileSource;
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::use_vue_toggle_inside_transition::UseVueToggleInsideTransitionOptions;

declare_lint_rule! {
    /// Require the element inside a Vue `<Transition>` to be shown and hidden with `v-if` or `v-show`.
    ///
    /// A `<Transition>` plays its animation when the element inside it is added,
    /// removed, shown, or hidden. An element that is always shown never changes,
    /// so the animation never plays.
    ///
    /// The rule checks the first element inside each `<Transition>`. That element
    /// is accepted when one of the following is true:
    ///
    /// - it has a `v-if` or `v-show` directive;
    /// - it has a `key` bound with `:key` or `v-bind:key`, because Vue replaces the
    ///   element when the key changes;
    /// - it is a component, a `<component>`, a `<slot>`, or an element with an `is`
    ///   attribute, because the rule can't see what they render;
    /// - the `<Transition>` has the `appear` attribute, which plays the animation
    ///   when the element is first shown. `:appear="false"` doesn't count.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <Transition>
    ///     <div>Content</div>
    ///   </Transition>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <Transition>
    ///     <div v-if="visible">Content</div>
    ///   </Transition>
    ///   <Transition>
    ///     <div v-show="visible">Content</div>
    ///   </Transition>
    ///   <Transition>
    ///     <div :key="step">Content</div>
    ///   </Transition>
    ///   <Transition>
    ///     <MyComponent />
    ///   </Transition>
    ///   <Transition appear>
    ///     <div>Content</div>
    ///   </Transition>
    /// </template>
    /// ```
    ///
    pub UseVueToggleInsideTransition {
        version: "2.6.3",
        name: "useVueToggleInsideTransition",
        language: "html",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("require-toggle-inside-transition").same()],
    }
}

impl Rule for UseVueToggleInsideTransition {
    type Query = Ast<HtmlElement>;
    /// The opening tag of the first element inside the transition.
    type State = AnyHtmlTagElement;
    type Signals = Option<Self::State>;
    type Options = UseVueToggleInsideTransitionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }

        let transition = ctx.query();
        let opening = transition.opening_element().ok()?;
        // Vue resolves the built-in component from these two spellings only.
        if !matches!(opening.tag_name()?.text(), "Transition" | "transition") {
            return None;
        }
        if has_appear(&opening) {
            return None;
        }

        // A transition animates a single element, so only the first one matters.
        // Text and comments are skipped.
        let child = transition
            .children()
            .iter()
            .find_map(AnyHtmlElement::as_any_html_tag_element)?;
        if !is_native_element(&child) || is_toggled(&child) {
            return None;
        }

        Some(child)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, child: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                child.range(),
                markup! {
                    "This element inside "<Emphasis>"<Transition>"</Emphasis>" is never shown or hidden."
                },
            )
            .note(markup! {
                "A transition only plays when the element inside it appears or disappears. This element is always present, so the transition never plays."
            })
            .note(markup! {
                "Add "<Emphasis>"v-if"</Emphasis>" or "<Emphasis>"v-show"</Emphasis>" to the element, or bind a "<Emphasis>":key"</Emphasis>" that changes when the content does."
            }),
        )
    }
}

/// Tag kinds that Vue doesn't render as a known native element: the built-in
/// `<component>` and `<slot>`, and any tag name that isn't a known HTML or SVG
/// tag.
const NON_NATIVE_TAGS: TokenSet<HtmlSyntaxKind> =
    token_set![T![component], T![slot], HtmlSyntaxKind::HTML_UNKNOWN_TAG];

/// Returns `true` if the transition has an `appear` attribute that isn't bound
/// to the literal `false`.
fn has_appear(transition: &HtmlOpeningElement) -> bool {
    let attributes = transition.attributes();
    if attributes.find_attribute_by_name("appear").is_some() {
        return true;
    }

    let initializer = match attributes.find_vue_binding("appear") {
        Some(AnyVueDirective::VueDirective(directive)) => directive.initializer(),
        Some(AnyVueDirective::VueVBindShorthandDirective(directive)) => directive.initializer(),
        _ => return false,
    };
    !initializer
        .and_then(|initializer| initializer.value().ok())
        .and_then(|value| value.as_html_string()?.inner_string_text().ok())
        .is_some_and(|value| value.trim() == "false")
}

/// Returns `true` if `element` is a known HTML or SVG element, rather than a
/// component or an element that renders something else in its place.
fn is_native_element(element: &AnyHtmlTagElement) -> bool {
    element
        .tag_name_kind()
        .is_some_and(|kind| !NON_NATIVE_TAGS.contains(kind))
        && element.find_attribute_or_vue_binding("is").is_none()
}

/// Returns `true` if `element` is conditionally rendered or displayed, or is
/// replaced whenever its bound `key` changes.
fn is_toggled(element: &AnyHtmlTagElement) -> bool {
    let attributes = element.attributes();
    attributes.find_vue_binding("key").is_some()
        || attributes.iter().any(|attribute| {
            attribute
                .as_any_vue_directive()
                .and_then(|directive| directive.as_vue_directive())
                .is_some_and(|directive| directive.is_if() || directive.is_show())
        })
}
