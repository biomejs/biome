use std::str::FromStr;

use crate::services::aria::Aria;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_aria::AriaAttribute;
use biome_aria_metadata::AriaRole;
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
            let role = attributes
                .get("role")
                .and_then(|roles| roles.first())
                .and_then(|role| AriaRole::from_roles(role))
                .or_else(|| aria_roles.get_implicit_role(element_name, attributes));
            let role_attributes = role.map_or(Default::default(), |role| role.attributes());
            let role_prohibited_attributes =
                role.map_or(Default::default(), |role| role.prohibited_attributes());
            for (attribute, values) in attributes {
                let Ok(aria_attribute) = AriaAttribute::from_str(attribute) else {
                    continue;
                };
                // Allow null/undefined values regardless of the role
                if values
                    .iter()
                    .any(|val| matches!(val.as_str(), "null" | "undefined"))
                {
                    continue;
                }
                if role_prohibited_attributes.contains(&aria_attribute)
                    || (!aria_attribute.is_global() && !role_attributes.contains(&aria_attribute))
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
