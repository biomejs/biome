use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria::AriaRoles;
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlAttribute};
use biome_rowan::{AstNode, BatchMutationExt, Text};
use biome_rule_options::no_redundant_roles::NoRedundantRolesOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce explicit `role` property is not the same as implicit/default role property on an element.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <article role="article"></article>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <button role="button"></button>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <h1 role="heading" aria-level="1">title</h1>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <article role="presentation"></article>
    /// ```
    ///
    /// ```html
    /// <span></span>
    /// ```
    ///
    pub NoRedundantRoles {
        version: "next",
        name: "noRedundantRoles",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("no-redundant-roles").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    redundant_attribute: HtmlAttribute,
    role_attribute_value: Text,
}

impl Rule for NoRedundantRoles {
    type Query = Ast<AnyHtmlElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoRedundantRolesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_attribute_value = role_attribute.initializer()?.value().ok()?.string_value()?;
        let explicit_role = AriaRole::from_roles(role_attribute_value.trim())?;

        if AriaRoles.get_implicit_role(node)? == explicit_role {
            return Some(RuleState {
                redundant_attribute: role_attribute,
                role_attribute_value,
            });
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let element_name = ctx.query().name()?;
        let element_name = element_name.text();
        let role_attribute = state.role_attribute_value.to_string();
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.redundant_attribute.range(),
            markup! {
                "Using the role attribute '"{role_attribute}"' on the '"{element_name}"' element is redundant, because it is implied by its semantic."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(state.redundant_attribute.clone());
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"role"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}
