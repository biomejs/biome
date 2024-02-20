use crate::{aria_services::Aria, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
    /// Enforce that aria-hidden="true" is not set on focusable elements.
    ///
    /// `aria-hidden="true"` can be used to hide purely decorative content from screen reader users.
    /// A focusable element with `aria-hidden="true"` can be reached by keyboard.
    /// This can lead to confusion or unexpected behavior for screen reader users.
    ///
    /// ## Example
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <div aria-hidden="true" tabIndex="0" />
    /// ```
    ///
    /// ```js, expect_diagnostic
    /// <a href="/" aria-hidden="true" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// <button aria-hidden="true" tabIndex="-1" />
    /// ```
    ///
    /// ```js
    /// <div aria-hidden="true"><a href="#"></a></div>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [aria-hidden elements do not contain focusable elements](https://dequeuniversity.com/rules/axe/html/4.4/aria-hidden-focus)
    /// - [Element with aria-hidden has no content in sequential focus navigation](https://www.w3.org/WAI/standards-guidelines/act/rules/6cfa84/proposed/)
    /// - [MDN aria-hidden](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden)
    ///
    pub NoAriaHiddenOnFocusable {
        version: "1.4.0",
        name: "noAriaHiddenOnFocusable",
        source: RuleSource::EslintJsxA11y("no-aria-hidden-on-focusable"),
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoAriaHiddenOnFocusable {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_roles = ctx.aria_roles();
        let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;

        if node.is_element() {
            let aria_hidden_attr = node.find_attribute_by_name("aria-hidden")?;
            let attr_static_val = aria_hidden_attr.as_static_value()?;
            let attr_text = attr_static_val.text();

            let attributes = ctx.extract_attributes(&node.attributes());

            if attr_text == "false" {
                return None;
            }

            if let Some(tabindex_attr) = node.find_attribute_by_name("tabIndex") {
                if let Some(tabindex_static) = tabindex_attr.as_static_value() {
                    let tabindex_text = tabindex_static.text();
                    let tabindex_val = tabindex_text.trim().parse::<i32>();

                    if let Ok(num) = tabindex_val {
                        return (num >= 0).then_some(());
                    }
                }
            }

            if !aria_roles.is_not_interactive_element(element_name.text_trimmed(), attributes) {
                return Some(());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Disallow "<Emphasis>"aria-hidden=\"true\""</Emphasis>" from being set on focusable elements."
                },
            )
            .note(markup! {
                ""<Emphasis>"aria-hidden"</Emphasis>" should not be set to "<Emphasis>"true"</Emphasis>" on focusable elements because this can lead to confusing behavior for screen reader users."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let aria_hidden_attr = node.find_attribute_by_name("aria-hidden")?;
        mutation.remove_node(aria_hidden_attr);
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the aria-hidden attribute from the element." }.to_owned(),
            mutation,
        })
    }
}
