use crate::aria_services::Aria;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::JsxAttribute;
use biome_rowan::AstNode;

declare_rule! {
    /// Enforce that elements with ARIA roles must have all required ARIA attributes for that role.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <span role="checkbox"></span>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <span role="heading"></span>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <span role="checkbox" aria-checked="true"></span>
    /// ```
    ///
    /// ```jsx
    /// <span role="heading" aria-level="1"></span>
    /// ```
    ///
    ///
    /// ## Accessibility guidelines
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    /// ### Resources
    /// - [ARIA Spec, Roles](https://www.w3.org/TR/wai-aria/#roles)
    /// - [Chrome Audit Rules, AX_ARIA_03](https://github.com/GoogleChrome/accessibility-developer-tools/wiki/Audit-Rules#ax_aria_03)
    pub UseAriaPropsForRole {
        version: "1.0.0",
        name: "useAriaPropsForRole",
        source: RuleSource::EslintJsxA11y("role-has-required-aria-props"),
        recommended: true,
    }
}

#[derive(Default, Debug)]
pub struct UseAriaPropsForRoleState {
    missing_aria_props: Vec<String>,
    attribute: Option<(JsxAttribute, String)>,
}

impl UseAriaPropsForRoleState {
    pub(crate) fn as_diagnostic(&self) -> Option<RuleDiagnostic> {
        if self.missing_aria_props.is_empty() {
            return None;
        }
        self.attribute.as_ref().map(|(attribute, role_name)| {
            let joined_attributes = &self.missing_aria_props.join(", ");
            let description = format!("The element with the {role_name} ARIA role does not have the required ARIA attributes: {}.", joined_attributes);
            RuleDiagnostic::new(
                rule_category!(),
                attribute.range(),
                markup! {
                "The element with the "<Emphasis>{role_name}</Emphasis>" ARIA role does not have the required ARIA attributes."
                },
            )
            .description(description)
            .footer_list(markup! { "Missing ARIA prop(s):" }, &self.missing_aria_props)
        })
    }
}

impl Rule for UseAriaPropsForRole {
    type Query = Aria<AnyJsxElement>;
    type State = UseAriaPropsForRoleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let roles = ctx.aria_roles();
        let is_inside_element = node
            .syntax()
            .ancestors()
            .find_map(|ancestor| {
                AnyJsxElement::cast(ancestor)
                    .map(|element| Some(element.is_element()))
                    .unwrap_or(None)
            })
            .unwrap_or(false);

        if is_inside_element {
            let role_attribute = node.find_attribute_by_name("role")?;

            let name = role_attribute
                .initializer()?
                .value()
                .ok()?
                .as_jsx_string()?
                .inner_string_text()
                .ok()?;

            let role = roles.get_role(name.text());
            let missing_aria_props: Vec<_> = role
                .into_iter()
                .flat_map(|role| role.properties())
                .filter_map(|(property_name, required)| {
                    if *required {
                        let attribute = node.find_attribute_by_name(property_name);
                        if attribute.is_none() {
                            Some((*property_name).to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            if !missing_aria_props.is_empty() {
                return Some(UseAriaPropsForRoleState {
                    attribute: Some((role_attribute, name.text().to_string())),
                    missing_aria_props,
                });
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        state.as_diagnostic()
    }
}
