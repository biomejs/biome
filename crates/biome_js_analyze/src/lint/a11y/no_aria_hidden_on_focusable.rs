use crate::{JsRuleAction, a11y::find_tabindex_attribute, services::aria::Aria};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsxAttributeValue, AnyNumberLikeExpression, JsxAttribute, jsx_ext::AnyJsxElement,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_aria_hidden_on_focusable::NoAriaHiddenOnFocusableOptions;

declare_lint_rule! {
    /// Enforce that aria-hidden="true" is not set on focusable elements.
    ///
    /// `aria-hidden="true"` can be used to hide purely decorative content from screen reader users.
    /// A focusable element with `aria-hidden="true"` can be reached by keyboard.
    /// This can lead to confusion or unexpected behavior for screen reader users.
    ///
    /// The tab index is read from either the React attribute name `tabIndex` or the
    /// HTML attribute name `tabindex`, used by JSX dialects that mirror HTML names.
    /// A tab index whose value the rule cannot read as a number, such as
    /// `tabIndex={0 + 0}`, says nothing about the element either way, so the element
    /// is then judged on whether it is focusable in its own right.
    ///
    /// ## Example
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div aria-hidden="true" tabIndex="0" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a href="/" aria-hidden="true" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <button aria-hidden="true" tabIndex="-1" />
    /// ```
    ///
    /// ```jsx
    /// <button aria-hidden="true" tabIndex={-1} />
    /// ```
    ///
    /// ```jsx
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
        language: "jsx",
        sources: &[
            RuleSource::EslintJsxA11y("no-aria-hidden-on-focusable").same(),
            RuleSource::HtmlEslint("no-aria-hidden-on-focusable").inspired(),
            RuleSource::EslintAstro("jsx-a11y/no-aria-hidden-on-focusable").same(),
        ],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoAriaHiddenOnFocusable {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoAriaHiddenOnFocusableOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_element() {
            let aria_hidden_attr = node.find_attribute_by_name("aria-hidden")?;
            let attr_static_val = aria_hidden_attr.as_static_value()?;
            let attr_text = attr_static_val.text();

            if attr_text == "false" {
                return None;
            }

            if let Some(tabindex_attr) = find_tabindex_attribute(node)
                && let Some(tabindex) = tabindex_value(&tabindex_attr)
            {
                return (tabindex >= 0).then_some(());
            }

            if !ctx.aria_roles().is_not_interactive_element(node) {
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
                    "Incorrect use of "<Emphasis>"aria-hidden=\"true\""</Emphasis>" detected."
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
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"aria-hidden"</Emphasis>" attribute from the element." }.to_owned(),
            mutation,
        ))
    }
}

/// Read the element's tab index as a number.
///
/// Returns `None` when the value is not one the rule can read, such as an
/// arbitrary expression: the element is then judged on its own focusability.
fn tabindex_value(tabindex_attr: &JsxAttribute) -> Option<i32> {
    match tabindex_attr.initializer()?.value().ok()? {
        AnyJsxAttributeValue::AnyJsxTag(jsx_tag) => jsx_tag.to_trimmed_text().text().parse().ok(),
        AnyJsxAttributeValue::JsxString(jsx_string) => {
            jsx_string.inner_string_text().ok()?.text().parse().ok()
        }
        AnyJsxAttributeValue::JsxExpressionAttributeValue(value) => {
            let expression = value.expression().ok()?;
            AnyNumberLikeExpression::cast(expression.into_syntax())?
                .value()?
                .parse()
                .ok()
        }
        AnyJsxAttributeValue::JsTemplateExpression(_) => None,
    }
}
