use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlAttribute, HtmlFileSource};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_aria_hidden_on_focusable::NoAriaHiddenOnFocusableOptions;

use crate::HtmlRuleAction;
use crate::a11y::get_truthy_aria_hidden_attribute;

declare_lint_rule! {
    /// Enforce that aria-hidden="true" is not set on focusable elements.
    ///
    /// `aria-hidden="true"` can be used to hide purely decorative content from screen reader users.
    /// A focusable element with `aria-hidden="true"` can be reached by keyboard.
    /// This can lead to confusion or unexpected behavior for screen reader users.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div aria-hidden="true" tabindex="0"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <a href="/" aria-hidden="true">link</a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div aria-hidden="true"></div>
    /// ```
    ///
    /// ```html
    /// <button aria-hidden="true" tabindex="-1"></button>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [aria-hidden elements do not contain focusable elements](https://dequeuniversity.com/rules/axe/html/4.4/aria-hidden-focus)
    /// - [Element with aria-hidden has no content in sequential focus navigation](https://www.w3.org/WAI/standards-guidelines/act/rules/6cfa84/proposed/)
    /// - [MDN aria-hidden](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden)
    ///
    pub NoAriaHiddenOnFocusable {
        version: "next",
        name: "noAriaHiddenOnFocusable",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("no-aria-hidden-on-focusable").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct NoAriaHiddenOnFocusableState {
    aria_hidden_attribute: HtmlAttribute,
}

impl Rule for NoAriaHiddenOnFocusable {
    type Query = Ast<AnyHtmlElement>;
    type State = NoAriaHiddenOnFocusableState;
    type Signals = Option<Self::State>;
    type Options = NoAriaHiddenOnFocusableOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        // Check if element has aria-hidden attribute with truthy value
        let aria_hidden_attr = get_truthy_aria_hidden_attribute(element)?;

        let file_source = ctx.source_type::<HtmlFileSource>();
        let is_html = file_source.is_html();

        // Check tabindex attribute first
        if let Some(tabindex_attr) = element.find_attribute_by_name("tabindex") {
            if let Some(tabindex_value) = get_tabindex_value(&tabindex_attr) {
                if tabindex_value < 0 {
                    // Negative tabindex removes element from tab order — valid
                    return None;
                }
                // Non-negative tabindex makes the element focusable — violation
                return Some(NoAriaHiddenOnFocusableState {
                    aria_hidden_attribute: aria_hidden_attr,
                });
            }
        }

        // Check if element is natively focusable or has contenteditable
        if is_focusable_element(element, is_html) {
            return Some(NoAriaHiddenOnFocusableState {
                aria_hidden_attribute: aria_hidden_attr,
            });
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Disallow "<Emphasis>"aria-hidden=\"true\""</Emphasis>" from being set on focusable elements."
                },
            )
            .note(markup! {
                ""<Emphasis>"aria-hidden"</Emphasis>" should not be set to "<Emphasis>"true"</Emphasis>" on focusable elements because this can lead to confusing behavior for screen reader users."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(state.aria_hidden_attribute.clone());
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"aria-hidden"</Emphasis>" attribute from the element." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Parses the tabindex attribute value as an integer.
fn get_tabindex_value(attribute: &HtmlAttribute) -> Option<i32> {
    let value = attribute.value()?;
    value.trim().parse::<i32>().ok()
}

/// Checks if an element is natively focusable or has contenteditable.
fn is_focusable_element(element: &AnyHtmlElement, is_html: bool) -> bool {
    if let Some(element_name) = element.name() {
        let name_matches = |name: &str| -> bool {
            if is_html {
                element_name.eq_ignore_ascii_case(name)
            } else {
                element_name.text() == name
            }
        };

        // <a> and <area> are only focusable when they have an href attribute
        if (name_matches("a") || name_matches("area"))
            && element.find_attribute_by_name("href").is_some()
        {
            return true;
        }

        // These elements are always natively focusable
        if name_matches("button")
            || name_matches("select")
            || name_matches("textarea")
            || name_matches("details")
            || name_matches("summary")
        {
            return true;
        }

        // <input> is focusable unless type="hidden"
        if name_matches("input") {
            let is_hidden_input = element
                .find_attribute_by_name("type")
                .and_then(|attr| attr.value())
                .is_some_and(|value| value.trim().eq_ignore_ascii_case("hidden"));
            return !is_hidden_input;
        }
    }

    // Check contenteditable attribute
    has_contenteditable_true(element)
}

/// Checks if the element has contenteditable set to a truthy value.
fn has_contenteditable_true(element: &AnyHtmlElement) -> bool {
    element
        .find_attribute_by_name("contenteditable")
        .is_some_and(|attr| match attr.value() {
            None => true,
            Some(value) => {
                let trimmed = value.trim();
                trimmed.is_empty() || !trimmed.eq_ignore_ascii_case("false")
            }
        })
}
