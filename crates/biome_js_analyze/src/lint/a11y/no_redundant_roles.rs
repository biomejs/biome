use crate::{JsRuleAction, services::aria::Aria};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsxAttributeValue, JsxAttribute, jsx_ext::AnyJsxElement};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_redundant_roles::NoRedundantRolesOptions;

declare_lint_rule! {
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
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("no-redundant-roles").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    redundant_attribute: JsxAttribute,
    redundant_attribute_value: AnyJsxAttributeValue,
}

impl Rule for NoRedundantRoles {
    type Query = Aria<AnyJsxElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoRedundantRolesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_attribute_value = role_attribute.initializer()?.value().ok()?;
        let explicit_role = AriaRole::from_roles(role_attribute_value.as_static_value()?.text())?;

        if ctx.aria_roles().get_implicit_role(node)? == explicit_role {
            return Some(RuleState {
                redundant_attribute: role_attribute,
                redundant_attribute_value: role_attribute_value,
            });
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let binding = state.redundant_attribute_value.as_static_value()?;
        let role_attribute = binding.text();
        let element_name = ctx.query().name().ok()?.as_jsx_name()?.value_token().ok()?;
        let element_name = element_name.text_trimmed();
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.redundant_attribute_value.range(),
            markup! {
                "Using the role attribute '"{role_attribute}"' on the '"{element_name}"' element is redundant, because it is implied by its semantic."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(state.redundant_attribute.clone());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"role"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}
