use std::str::FromStr;

use crate::services::aria::Aria;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_aria_metadata::AriaAttribute;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsxAttribute, jsx_ext::AnyJsxElement};
use biome_rowan::AstNode;
use biome_rule_options::use_aria_props_supported_by_role::UseAriaPropsSupportedByRoleOptions;

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
        sources: &[RuleSource::EslintJsxA11y("role-supports-aria-props").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseAriaPropsSupportedByRole {
    type Query = Aria<AnyJsxElement>;
    type State = AriaAttribute;
    type Signals = Option<Self::State>;
    type Options = UseAriaPropsSupportedByRoleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // Ignore custom components and namespaced elements
        if !node.is_element() {
            return None;
        }

        let role = ctx.aria_roles().get_role_by_element_name(node);

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
                .is_some_and(|static_value| static_value.is_null_or_undefined())
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
