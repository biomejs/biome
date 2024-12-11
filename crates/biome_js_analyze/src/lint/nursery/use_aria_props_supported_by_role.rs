use std::str::FromStr;

use crate::services::aria::Aria;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_aria_metadata::AriaAttribute;
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{jsx_ext::AnyJsxElement, AnyJsxAttribute};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Enforce that ARIA properties are valid for the roles that are supported by the element.
    ///
    /// Invalid ARIA properties can make it difficult for users of assistive technologies to understand the purpose of the element.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <a href="#" aria-checked />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <img alt="foobar" aria-checked />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///     <a href="#" aria-expanded />
    ///     <img alt="foobar" aria-hidden />
    ///     <div role="heading" aria-level="1" />
    /// </>
    /// ```
    ///
    pub UseAriaPropsSupportedByRole {
        version: "1.9.0",
        name: "useAriaPropsSupportedByRole",
        language: "js",
        sources: &[RuleSource::EslintJsxA11y("role-supports-aria-props")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseAriaPropsSupportedByRole {
    type Query = Aria<AnyJsxElement>;
    type State = AriaAttribute;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if !node.name().is_ok_and(|name| name.as_jsx_name().is_some()) {
            // Ignore custom components and namespaced elements
            return None;
        }

        let role = node
            .find_attribute_by_name("role")
            .and_then(|attribute| attribute.as_static_value())
            .and_then(|value| AriaRole::from_roles(value.text()))
            .or_else(|| ctx.aria_roles().get_implicit_role(node));

        let role_attributes = role.map_or(Default::default(), |role| role.attributes());
        let role_prohibited_attributes =
            role.map_or(Default::default(), |role| role.prohibited_attributes());

        for attribute in node.attributes() {
            let AnyJsxAttribute::JsxAttribute(attribute) = attribute else {
                continue;
            };
            let aria_attribute = attribute.name().ok().and_then(|x| {
                AriaAttribute::from_str(x.as_jsx_name()?.value_token().ok()?.text_trimmed()).ok()
            });
            let Some(aria_attribute) = aria_attribute else {
                continue;
            };
            // Allow null/undefined values regardless of the role
            if attribute
                .as_static_value()
                .is_some_and(|value| matches!(value.text(), "null" | "undefined"))
            {
                continue;
            }
            if role_prohibited_attributes.contains(&aria_attribute)
                || (!aria_attribute.is_global() && !role_attributes.contains(&aria_attribute))
            {
                return Some(aria_attribute);
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let invalid_aria_prop = state.as_str();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The ARIA attribute '"{invalid_aria_prop}"' is not supported by this element."
                },
            )
            .note(markup! {
                "Ensure that ARIA attributes are valid for the role of the element."
            }),
        )
    }
}
