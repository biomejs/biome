use biome_analyze::RuleSource;
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, declare_lint_rule};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlAttribute};
use biome_rowan::{AstNode, Text};
use biome_rule_options::use_aria_props_supported_by_role::UseAriaPropsSupportedByRoleOptions;

declare_lint_rule! {
    /// Enforce that elements with ARIA roles must have all required ARIA attributes for that role.
    ///
    /// Remember that this rule only supports static values for the `role` attribute.
    /// Dynamic `role` values are not checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <span role="checkbox"></span>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <span role="heading"></span>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <span role="checkbox" aria-checked="true"></span>
    /// ```
    ///
    /// ```html
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
    ///
    pub UseAriaPropsForRole {
        version: "next",
        name: "useAriaPropsForRole",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("role-has-required-aria-props").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

#[derive(Default, Debug)]
pub struct UseAriaPropsForRoleState {
    missing_aria_props: Box<[&'static str]>,
    attribute: Option<(HtmlAttribute, Text)>,
}

impl Rule for UseAriaPropsForRole {
    type Query = Ast<AnyHtmlElement>;
    type State = UseAriaPropsForRoleState;
    type Signals = Option<Self::State>;
    type Options = UseAriaPropsSupportedByRoleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_attribute_name = role_attribute.initializer()?.value().ok()?.string_value()?;
        let role = AriaRole::from_roles(role_attribute_name.trim());
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
        if missing_aria_props.is_empty() {
            return None;
        }

        Some(UseAriaPropsForRoleState {
            attribute: Some((role_attribute, role_attribute_name)),
            missing_aria_props: missing_aria_props.into_boxed_slice(),
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        state.attribute.as_ref().map(|(attribute, role_name)| {
            let role_name = role_name.trim();
            RuleDiagnostic::new(
                rule_category!(),
                attribute.range(),
                markup! {
                "The element with the "<Emphasis>{role_name}</Emphasis>" ARIA role does not have the required ARIA attributes."
                },
            )
            .footer_list(markup! { "Missing ARIA prop(s):" }, &state.missing_aria_props)
        })
    }
}
