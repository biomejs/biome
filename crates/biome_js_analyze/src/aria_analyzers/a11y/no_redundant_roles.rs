use crate::{aria_services::Aria, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_aria::{roles::AriaRoleDefinition, AriaRoles};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::{
    jsx_ext::AnyJsxElement, AnyJsxAttributeValue, JsxAttribute, JsxAttributeList,
};
use biome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
    /// Enforce explicit `role` property is not the same as implicit/default role property on an element.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <article role='article'></article>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <button role='button'></button>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <h1 role='heading' aria-level='1'>title</h1>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <article role='presentation'></article>
    /// ```
    ///
    /// ```jsx
    /// <Button role='button'></Button>
    /// ```
    ///
    /// ```jsx
    /// <span></span>
    /// ```
    ///
    pub NoRedundantRoles {
        version: "1.0.0",
        name: "noRedundantRoles",
        source: RuleSource::EslintJsxA11y("no-redundant-roles"),
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    redundant_attribute: JsxAttribute,
    redundant_attribute_value: AnyJsxAttributeValue,
    element_name: String,
}

impl Rule for NoRedundantRoles {
    type Query = Aria<AnyJsxElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_roles = ctx.aria_roles();

        let (element_name, attributes) = get_element_name_and_attributes(node)?;
        let attribute_name_to_values = ctx.extract_attributes(&attributes)?;
        let implicit_role =
            aria_roles.get_implicit_role(&element_name, &attribute_name_to_values)?;

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_attribute_value = role_attribute.initializer()?.value().ok()?;
        let explicit_role = get_explicit_role(aria_roles, &role_attribute_value)?;

        let is_redundant = implicit_role.type_name() == explicit_role.type_name();
        if is_redundant {
            return Some(RuleState {
                redundant_attribute: role_attribute,
                redundant_attribute_value: role_attribute_value,
                element_name: element_name.to_string(),
            });
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let binding = state.redundant_attribute_value.as_static_value()?;
        let role_attribute = binding.text();
        let element = state.element_name.to_string();
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.redundant_attribute_value.range(),
            markup! {
                "Using the role attribute '"{role_attribute}"' on the '"{element}"' element is redundant, because it is implied by the semantic '"{element}"' element."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(state.redundant_attribute.clone());
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the "<Emphasis>"role"</Emphasis>" attribute." }.to_owned(),
            mutation,
        })
    }
}

fn get_element_name_and_attributes(node: &AnyJsxElement) -> Option<(String, JsxAttributeList)> {
    match node {
        AnyJsxElement::JsxOpeningElement(elem) => {
            let token = elem.name().ok()?;
            let element_name = token.as_jsx_name()?.value_token().ok()?;
            let trimmed_element_name = element_name.text_trimmed().to_string();
            Some((trimmed_element_name, elem.attributes()))
        }
        AnyJsxElement::JsxSelfClosingElement(elem) => {
            let token = &elem.name().ok()?;
            let element_name = token.as_jsx_name()?.value_token().ok()?;
            let trimmed_element_name = element_name.text_trimmed().to_string();
            Some((trimmed_element_name, elem.attributes()))
        }
    }
}

fn get_explicit_role(
    aria_roles: &AriaRoles,
    role_attribute_value: &AnyJsxAttributeValue,
) -> Option<&'static dyn AriaRoleDefinition> {
    let static_value = role_attribute_value.as_static_value()?;

    // If a role attribute has multiple values, the first valid value (specified role) will be used.
    // Check: https://www.w3.org/TR/2014/REC-wai-aria-implementation-20140320/#mapping_role
    let explicit_role = static_value
        .text()
        .split(' ')
        .find_map(|role| aria_roles.get_role(role))?;
    Some(explicit_role)
}
