use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_rowan::AstNode;
use biome_rule_options::no_static_element_interactions::NoStaticElementInteractionsOptions;

use crate::{
    Aria,
    a11y::{has_event_handler, is_hidden_from_screen_reader},
};

declare_lint_rule! {
    /// Require an appropriate role when a non-interactive element has event handlers.
    ///
    /// Elements such as `<div>` and `<span>` have no built-in interactive meaning. Adding a mouse,
    /// keyboard, or focus handler does not tell assistive technologies that the element behaves like
    /// a control. Prefer a native interactive element such as `<button>`. When that is not possible,
    /// add an appropriate [ARIA role](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) so the control's
    /// purpose can be announced.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div onclick="myFunction()"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <span onclick="myFunction()"></span>
    /// ```
    ///
    /// An `<a>` element without an `href` attribute is non-interactive.
    /// ```html,expect_diagnostic
    /// <a onclick="myFunction()"></a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div role="button" onclick="myFunction()"></div>
    /// <span role="scrollbar" onclick="myFunction()"></span>
    /// <a href="http://example.com" onclick="myFunction()"></a>
    /// ```
    ///
    /// Custom components are not checked.
    /// ```astro
    /// <TestComponent onclick={doFoo} />
    /// ```
    ///
    pub NoStaticElementInteractions {
        version: "2.5.0",
        name: "noStaticElementInteractions",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("no-static-element-interactions").inspired()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoStaticElementInteractions {
    type Query = Aria<AnyHtmlTagElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoStaticElementInteractionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // Custom components are not checked because we do not know what DOM will be used.
        if node.is_custom_component() {
            return None;
        }

        // Svelte special elements (e.g. `<svelte:window>`) are not real DOM elements.
        if node.is_svelte_special_element() {
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

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
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
