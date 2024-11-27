use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::JsxAttribute;
use biome_rowan::{AstNode, TokenText};

declare_lint_rule! {
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
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("role-has-required-aria-props")],
        recommended: true,
        severity: Severity::Error,
    }
}

#[derive(Default, Debug)]
pub struct UseAriaPropsForRoleState {
    missing_aria_props: Box<[&'static str]>,
    attribute: Option<(JsxAttribute, TokenText)>,
}

impl Rule for UseAriaPropsForRole {
    type Query = Ast<AnyJsxElement>;
    type State = UseAriaPropsForRoleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let is_inside_element = node
            .syntax()
            .ancestors()
            .find_map(|ancestor| AnyJsxElement::cast(ancestor).map(|element| element.is_element()))
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
            let role = AriaRole::from_roles(name.text());
            let missing_aria_props: Vec<_> = role
                .into_iter()
                .flat_map(|role| role.required_attributes().iter())
                .filter_map(|attribute| {
                    let attribute_name = attribute.as_str();
                    node.find_attribute_by_name(attribute_name)
                        .is_none()
                        .then_some(attribute_name)
                })
                .collect();
            if !missing_aria_props.is_empty() {
                return Some(UseAriaPropsForRoleState {
                    attribute: Some((role_attribute, name)),
                    missing_aria_props: missing_aria_props.into_boxed_slice(),
                });
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        if state.missing_aria_props.is_empty() {
            return None;
        }
        state.attribute.as_ref().map(|(attribute, role_name)| {
            let role_name = role_name.text();
            let joined_attributes = &state.missing_aria_props.join(", ");
            let description = format!("The element with the {role_name} ARIA role does not have the required ARIA attributes: {joined_attributes}.");
            RuleDiagnostic::new(
                rule_category!(),
                attribute.range(),
                markup! {
                "The element with the "<Emphasis>{role_name}</Emphasis>" ARIA role does not have the required ARIA attributes."
                },
            )
            .description(description)
            .footer_list(markup! { "Missing ARIA prop(s):" }, &state.missing_aria_props)
        })
    }
}
