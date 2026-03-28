use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria::AriaRoles;
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlAttribute, HtmlFileSource};
use biome_rowan::{AstNode, BatchMutationExt, Text};
use biome_rule_options::no_redundant_roles::NoRedundantRolesOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce explicit `role` property is not the same as implicit/default role property on an element.
    ///
    /// :::note
    /// In `.html` files, all elements are treated as native HTML elements.
    ///
    /// In component-based frameworks (Vue, Svelte, Astro), only native HTML element names are checked.
    /// PascalCase names like `<Button>` and kebab-case names like `<my-button>` are assumed to be
    /// custom components and are ignored.
    /// :::
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
        sources: &[RuleSource::EslintJsxA11y("no-redundant-roles").same(), RuleSource::HtmlEslint("no-redundant-role").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoRedundantRoles {
    type Query = Ast<AnyHtmlElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoRedundantRolesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let source_type = ctx.source_type::<HtmlFileSource>();
        if !source_type.is_html() {
            let element_name = node.name()?;
            let name_text = element_name.text();
            if name_text.chars().next().is_some_and(|c| c.is_uppercase())
                || name_text.contains('-')
            {
                return None;
            }
        }

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_attribute_value = role_attribute.initializer()?.value().ok()?.string_value()?;
        let trimmed = role_attribute_value.trim();
        let explicit_role = AriaRole::from_roles(trimmed)?;

        if AriaRoles.get_implicit_role(node)? == explicit_role {
            let has_multiple_roles = trimmed.split_ascii_whitespace().nth(1).is_some();
            return Some(RuleState {
                redundant_attribute: role_attribute,
                role_attribute_value,
                has_multiple_roles,
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
                "Using the role attribute '"{role_attribute}"' on the '"{element_name}"' element is redundant, because it is implied by its semantics."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        if state.has_multiple_roles {
            return None;
        }
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

pub struct RuleState {
    redundant_attribute: HtmlAttribute,
    role_attribute_value: Text,
    has_multiple_roles: bool,
}
