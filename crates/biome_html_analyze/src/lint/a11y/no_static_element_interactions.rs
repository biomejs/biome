use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_rowan::AstNode;
use biome_rule_options::no_static_element_interactions::NoStaticElementInteractionsOptions;

use crate::{Aria, a11y::is_hidden_from_screen_reader};

declare_lint_rule! {
    /// Enforce that static, visible elements (such as `<div>`) that have click handlers use the valid role attribute.
    ///
    /// Static HTML elements do not have semantic meaning. This is clear in the case of `<div>` and `<span>`.
    /// It is less so clear in the case of elements that seem semantic, but that do not have a semantic mapping in the accessibility layer. For example `<a>` without href attribute, `<meta>`, `<script>`, `<picture>`, `<section>`, and `<colgroup>` -- to name a few -- have no semantic layer mapping. They are as void of meaning as `<div>`.
    ///
    /// The [WAI-ARIA role attribute](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) confers a semantic mapping to an element. The semantic value can then be expressed to a user via assistive technology.
    /// In order to add interactivity such as a mouse or key event listener to a static element, that element must be given a role value as well.
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
    /// When `<a>` does not have "href" attribute, that is non-interactive.
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
        version: "next",
        name: "noStaticElementInteractions",
        language: "html",
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

        if is_hidden_from_screen_reader(node) {
            return None;
        }

        if ctx.aria_roles().is_not_static_element(node) {
            return None;
        }

        // Check if the element has any interactive event handlers.
        if has_handler_props(node) {
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
                "Static Elements should not be interactive."
            },
        ).note(
            markup! {
                "To add interactivity such as a mouse or key event listener to a static element, give the element an appropriate role value."
            }
        ))
    }
}

const INTERACTIVE_HANDLERS: &[&str] = &[
    "onclick",
    "oncontextmenu",
    "ondblclick",
    "ondoubleclick",
    "ondrag",
    "ondragend",
    "ondragenter",
    "ondragexit",
    "ondragleave",
    "ondragover",
    "ondragstart",
    "ondrop",
    "onmousedown",
    "onmouseenter",
    "onmouseleave",
    "onmousemove",
    "onmouseout",
    "onmouseover",
    "onkeydown",
    "onkeypress",
    "onkeyup",
    "onfocus",
    "onblur",
];

/// Check if the element contains event handler
fn has_handler_props(element: &AnyHtmlTagElement) -> bool {
    INTERACTIVE_HANDLERS.iter().any(|handler| {
        element.find_attribute_by_name(handler).is_some()
            || handler.strip_prefix("on").is_some_and(|handler_name| {
                element
                    .find_vue_event_handling_directive(handler_name)
                    .is_some()
            })
    })
}
