use crate::services::aria::Aria;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;

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
    pub NoStaticElementInteractions {
        version: "1.9.0",
        name: "noStaticElementInteractions",
        language: "js",
        sources: &[RuleSource::EslintJsxA11y("no-static-element-interactions")],
        recommended: false,
    }
}

const EVENT_TO_HANDLERS: &[(&str, &[&str])] = &[
    ("keyboard", &["onKeyDown", "onKeyUp", "onKeyPress"]),
    ("focus", &["onFocus", "onBlur"]),
    (
        "mouse",
        &[
            "onClick",
            "onContextMenu",
            "onDblClick",
            "onDoubleClick",
            "onDrag",
            "onDragEnd",
            "onDragEnter",
            "onDragLeave",
            "onDragExit",
            "onDragOver",
            "onDragStart",
            "onDrop",
            "onMouseDown",
            "onMouseEnter",
            "onMouseLeave",
            "onMouseMove",
            "onMouseOut",
            "onMouseOver",
            "onMouseUp",
        ],
    ),
];

// no-static-element-interactions rule checks only focus, keyboard and mouse categories.
const CATEGORIES_TO_CHECK: &[&str] = &["focus", "keyboard", "mouse"];

impl Rule for NoStaticElementInteractions {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        let element_name = element_name.text_trimmed();

        // Check if the element is hidden from screen readers.
        if is_hidden_from_screen_reader(node, element_name) {
            return None;
        }

        if ctx.aria_roles().is_not_static_element(node) {
            return None;
        }

        // Check if the element has any interactive event handlers.
        if !CATEGORIES_TO_CHECK.iter().any(|&category| {
            if let Some(handlers) = EVENT_TO_HANDLERS
                .iter()
                .find(|&&(cat, _)| cat == category)
                .map(|&(_, handlers)| handlers)
            {
                handlers.iter().any(|&handler| {
                    if let Some(value) = node.find_attribute_by_name(handler) {
                        value
                            .as_static_value()
                            .map_or(true, |value| value.text() != "null")
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        }) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {{"Static Elements should not be interactive."}},
        ).note(
            markup! {{"To add interactivity such as a mouse or key event listener to a static element, give the element an appropriate role value."}}
        ))
    }
}

/**
 * Returns boolean indicating that the aria-hidden prop
 * is present or the value is true. Will also return true if
 * there is an input with type='hidden'.
 *
 * <div aria-hidden /> is equivalent to the DOM as <div aria-hidden=true />.
 * ref: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/src/util/isHiddenFromScreenReader.js
 */
fn is_hidden_from_screen_reader(node: &AnyJsxElement, element_name: &str) -> bool {
    node.find_attribute_by_name("aria-hidden")
        .is_some_and(|attr| {
            attr.as_static_value()
                .map_or(true, |val| val.text() == "true")
        })// <div aria-hidden />
        || (element_name == "input"
            && node.find_attribute_by_name("type").is_some_and(|attr| {
                attr.as_static_value()
                    .is_some_and(|val| val.text() == "hidden")
            })) // <input type="hidden" />
}
