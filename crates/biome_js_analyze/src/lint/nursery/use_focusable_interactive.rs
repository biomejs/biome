use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_aria::AriaRoles;
use biome_console::markup;
use biome_js_syntax::{jsx_ext::AnyJsxElement, AnyJsxAttributeValue};
use biome_rowan::AstNode;

use crate::services::aria::Aria;

declare_lint_rule! {
    /// Elements with an interactive role and interaction handlers must be focusable.
    ///
    /// HTML elements with interactive roles must have `tabIndex` defined to ensure they are
    /// focusable. Without tabIndex, assistive technologies may not recognize these elements as
    /// interactive.
    /// You could also consider switching from an interactive role to its semantic HTML element
    /// instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="button" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="tab" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div role="button" tabIndex={0} />
    /// ```
    ///
    /// ```jsx
    /// <div />
    /// ```
    ///
    pub UseFocusableInteractive {
        version: "1.8.0",
        name: "useFocusableInteractive",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("interactive-support-focus")],
        recommended: true,
    }
}

impl Rule for UseFocusableInteractive {
    type Query = Aria<AnyJsxElement>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !node.is_element() {
            return None;
        }

        let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        let aria_roles = ctx.aria_roles();
        let attributes = ctx.extract_attributes(&node.attributes());

        if aria_roles.is_not_interactive_element(element_name.text_trimmed(), attributes) {
            let role_attribute = node.find_attribute_by_name("role");
            if let Some(role_attribute) = role_attribute {
                let tabindex_attribute = node.find_attribute_by_name("tabIndex");
                let role_attribute_value = role_attribute.initializer()?.value().ok()?;
                if attribute_has_interactive_role(&role_attribute_value, aria_roles)?
                    && tabindex_attribute.is_none()
                {
                    return Some(role_attribute_value.text());
                }
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The HTML element with the interactive role "<Emphasis>{state}</Emphasis>" is not focusable."
                },
            ).note(markup! {
                "A non-interactive HTML element that is not focusable may not be reachable for users that rely on keyboard navigation, even with an added role like "<Emphasis>{state}</Emphasis>"."
            })
            .note(markup! {
                "Add a "<Emphasis>"tabIndex"</Emphasis>" attribute to make this element focusable."
            }),
        )
    }
}

/// Checks if the given role attribute value is interactive or not based on ARIA roles.
fn attribute_has_interactive_role(
    role_attribute_value: &AnyJsxAttributeValue,
    aria_roles: &AriaRoles,
) -> Option<bool> {
    Some(aria_roles.is_role_interactive(role_attribute_value.as_static_value()?.text()))
}
