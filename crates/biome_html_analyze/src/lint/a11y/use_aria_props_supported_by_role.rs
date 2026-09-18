use crate::services::aria::Aria;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_aria_metadata::{AriaAttribute, AriaRole};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlAttribute, element_ext::AnyHtmlTagElement};
use biome_rowan::AstNode;
use biome_rule_options::use_aria_props_supported_by_role::UseAriaPropsSupportedByRoleOptions;
use std::str::FromStr;

declare_lint_rule! {
    /// Enforce that ARIA properties are valid for the roles that are supported by the element.
    ///
    /// Invalid ARIA properties can make it difficult for users of assistive technologies to understand the purpose of the element.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <a href="#" aria-checked="true"></a>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <img alt="foobar" aria-checked="true" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <a href="#" aria-expanded="true"></a>
    /// <img alt="foobar" aria-hidden="true" />
    /// <div role="heading" aria-level="1"></div>
    /// ```
    ///
    pub UseAriaPropsSupportedByRole {
        version: "2.5.0",
        name: "useAriaPropsSupportedByRole",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("role-supports-aria-props").inspired()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseAriaPropsSupportedByRole {
    type Query = Aria<AnyHtmlTagElement>;
    type State = AriaAttribute;
    type Signals = Option<Self::State>;
    type Options = UseAriaPropsSupportedByRoleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // Ignore custom components
        if node.is_custom_component() {
            return None;
        }

        let role = node
            .find_attribute_by_name("role")
            .and_then(|attribute| attribute.initializer()?.value().ok()?.as_static_value())
            .and_then(|value| AriaRole::from_roles(value.text()))
            .or_else(|| ctx.aria_roles().get_implicit_role(node));

        let role_attributes = role.map_or(Default::default(), |role| role.attributes());
        let role_prohibited_attributes =
            role.map_or(Default::default(), |role| role.prohibited_attributes());

        for attribute in node.attributes() {
            let AnyHtmlAttribute::HtmlAttribute(attribute) = attribute else {
                continue;
            };
            let Some(aria_attribute) = attribute
                .name()
                .ok()
                .and_then(|x| AriaAttribute::from_str(x.value_token().ok()?.text_trimmed()).ok())
            else {
                continue;
            };
            // Allow null/undefined values regardless of the role
            if attribute
                .value()
                .is_some_and(|f| matches!(f.text(), "null" | "undefined"))
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
