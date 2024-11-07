use std::str::FromStr;

use crate::services::aria::Aria;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_aria::AriaAttribute;
use biome_aria_metadata::{AriaAttributes, AriaRole};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
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
    /// ```js
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
    }
}

impl Rule for UseAriaPropsSupportedByRole {
    type Query = Aria<AnyJsxElement>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        let element_name = element_name.text_trimmed();
        let aria_roles = ctx.aria_roles();
        let attributes = ctx.extract_attributes(&node.attributes());
        let attributes = ctx.convert_all_attribute_values(attributes);

        if let Some(attributes) = &attributes {
            let role_attributes = attributes
                .get("role")
                .and_then(|roles| roles.first())
                .and_then(|role| AriaRole::from_str(role).ok())
                .map_or(AriaAttributes::empty(), |role| role.attributes());
            let implicit_role = aria_roles.get_implicit_role(element_name, attributes)?;
            for attribute in attributes.keys() {
                let Ok(aria_attribute) = AriaAttribute::from_str(attribute) else {
                    continue;
                };
                if !role_attributes.contains(&aria_attribute)
                    && !is_valid_aria_props_supported_by_role(
                        implicit_role.type_name(),
                        attribute.as_str(),
                    )
                {
                    return Some(attribute.clone());
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let invalid_aria_prop = state;

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

fn is_valid_aria_props_supported_by_role(role_name: &'static str, aria_attribute: &str) -> bool {
    if is_global_aria(aria_attribute) {
        return true;
    }

    match role_name {
        "biome_aria::roles::LinkRole" => {
            matches!(
                aria_attribute,
                "aria-expanded" | "aria-haspopup" | "aria-current"
            )
        }
        "biome_aria::roles::ButtonRole" => {
            matches!(aria_attribute, "aria-expanded" | "aria-pressed")
        }
        "biome_aria::roles::CheckboxRole"
        | "biome_aria::roles::RadioRole"
        | "biome_aria::roles::MenuItemCheckboxRole"
        | "biome_aria::roles::MenuItemRadioRole" => {
            matches!(aria_attribute, "aria-checked")
        }
        "biome_aria::roles::ComboBoxRole" => {
            matches!(aria_attribute, "aria-expanded")
        }
        "biome_aria::roles::SliderRole" => {
            matches!(
                aria_attribute,
                "aria-valuemax" | "aria-valuemin" | "aria-valuenow"
            )
        }
        "biome_aria::roles::ListRole" => {
            matches!(aria_attribute, "aria-activedescendant")
        }
        "biome_aria::roles::HeadingRole" => matches!(aria_attribute, "aria-level"),
        // This rule is not concerned with the abstract role
        "biome_aria::roles::PresentationRole" | "biome_aria::roles::GenericRole" => true,
        _ => false,
    }
}

/// Check if the aria attribute is global
/// https://www.w3.org/TR/wai-aria-1.1/#global_states
///
/// However, aria-invalid and aria-haspopup are not included this list
/// Because every elements cannot have These attributes.
/// https://www.w3.org/TR/wai-aria-1.1/#aria-invalid
/// https://www.w3.org/TR/wai-aria-1.1/#aria-haspopup
fn is_global_aria(aria_attribute: &str) -> bool {
    matches! {
        aria_attribute,
        "aria-atomic"
            | "aria-busy"
            | "aria-controls"
            | "aria-describedby"
            | "aria-disabled"
            | "aria-dropeffect"
            | "aria-flowto"
            | "aria-grabbed"
            | "aria-hidden"
            | "aria-label"
            | "aria-labelledby"
            | "aria-live"
            | "aria-owns"
            | "aria-relevant"
            | "aria-roledescription"
    }
}
