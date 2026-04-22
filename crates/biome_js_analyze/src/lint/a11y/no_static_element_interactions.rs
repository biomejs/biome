use crate::a11y::{has_event_handler, is_hidden_from_screen_reader};
use crate::services::aria::Aria;
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;
use biome_rule_options::no_static_element_interactions::NoStaticElementInteractionsOptions;

declare_lint_rule! {
    /// Enforce that static, visible elements (such as `<div>`) that have click handlers use the valid role attribute.
    ///
    /// Static HTML elements do not have semantic meaning. This is clear in the case of `<div>` and `<span>`. It is less so clear in the case of elements that seem semantic, but that do not have a semantic mapping in the accessibility layer. For example `<a>` without href attribute, `<meta>`, `<script>`, `<picture>`, `<section>`, and `<colgroup>` -- to name a few -- have no semantic layer mapping. They are as void of meaning as `<div>`.
    ///
    /// The [WAI-ARIA role attribute](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) confers a semantic mapping to an element. The semantic value can then be expressed to a user via assistive technology.
    /// In order to add interactivity such as a mouse or key event listener to a static element, that element must be given a role value as well.
    ///
    /// Source: [jsx-a11y/no-static-element-interactions](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-static-element-interactions.md)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div onClick={() => {}}></div>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <span onClick={() => {}}></span>;
    /// ```
    ///
    /// When `<a>` does not have "href" attribute, that is non-interactive.
    /// ```jsx,expect_diagnostic
    /// <a onClick={() => {}}></a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///     <div role="button" onClick={() => {}}></div>
    ///     <span role="scrollbar" onClick={() => {}}></span>
    ///     <a href="http://example.com" onClick={() => {}}></a>
    /// </>
    /// ```
    ///
    /// Custom components are not checked.
    /// ```jsx
    /// <TestComponent onClick={doFoo} />
    /// ```
    ///
    pub NoStaticElementInteractions {
        version: "1.9.0",
        name: "noStaticElementInteractions",
        language: "js",
        sources: &[RuleSource::EslintJsxA11y("no-static-element-interactions").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoStaticElementInteractions {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoStaticElementInteractionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // Custom components are not checked because we do not know what DOM will be used.
        if node.is_custom_component() {
            return None;
        }

        if is_hidden_from_screen_reader(node) {
            return None;
        }

        if ctx.aria_roles().is_not_static_element(node) {
            return None;
        }

        if has_event_handler(EVENT_HANDLER_TYPES, node) {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Unexpected event handler on static element."
            },
        ).note(
            markup! {
                "Static elements should not be interactive. To add interactivity such as a mouse or key event listener to a static element, give the element an appropriate role value."
            }
        ))
    }
}

// Only check the focus, keyboard and mouse event handler types.
const EVENT_HANDLER_TYPES: &[&str] = &["focus", "keyboard", "mouse"];

#[test]
fn test_order() {
    assert!(EVENT_HANDLER_TYPES.is_sorted());
}
