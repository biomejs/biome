use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::AnyHtmlElement;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_valid_aria_role::UseValidAriaRoleOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Elements with ARIA roles must use a valid, non-abstract ARIA role.
    ///
    /// Remember that this rule only supports static values for the `role` attribute.
    /// Dynamic `role` values are not checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div role="datepicker"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div role="range"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div role=""></div>
    /// ```
    ///
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div role="button"></div>
    /// <div></div>
    /// ```
    ///
    /// ## Options
    ///
    ///
    /// ### `allowInvalidRoles`
    ///
    /// It allows specifying a list of roles that might be invalid otherwise
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowInvalidRoles": ["datepicker"]
    ///     }
    /// }
    /// ```
    ///
    /// ```html,use_options
    /// <div role="datepicker"></div>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    /// ## Resources
    ///
    /// - [Chrome Audit Rules, AX_ARIA_01](https://github.com/GoogleChrome/accessibility-developer-tools/wiki/Audit-Rules#ax_aria_01)
    /// - [DPUB-ARIA roles](https://www.w3.org/TR/dpub-aria-1.0/)
    /// - [MDN: Using ARIA: Roles, states, and properties](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/ARIA_Techniques)
    ///
    pub UseValidAriaRole {
        version: "2.4.0",
        name: "useValidAriaRole",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("aria-role").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseValidAriaRole {
    type Query = Ast<AnyHtmlElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseValidAriaRoleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();

        let allowed_invalid_roles = &options.allow_invalid_roles;

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_attribute_static_value =
            role_attribute.initializer()?.value().ok()?.string_value()?;
        let role_attribute_value = role_attribute_static_value.trim();
        if role_attribute_value.is_empty() {
            return Some(());
        }
        let mut role_attribute_value = role_attribute_value.split_ascii_whitespace();

        let is_valid = role_attribute_value.all(|val| {
            AriaRole::from_roles(val).is_some()
                || allowed_invalid_roles
                    .iter()
                    .flatten()
                    .any(|role| role.as_ref() == val)
        });

        if is_valid {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Enforce that elements with ARIA roles must use a valid, non-abstract ARIA role."
                },
            )
            .note(markup! {
                "Check "<Hyperlink href="https://www.w3.org/TR/wai-aria/#namefromauthor">"WAI-ARIA"</Hyperlink>" for valid roles or provide options accordingly."
            })
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let role_attribute = node.find_attribute_by_name("role")?;
        mutation.remove_node(role_attribute);
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),

                markup! { "Remove the invalid "<Emphasis>"role"</Emphasis>" attribute.\n Check the list of all "<Hyperlink href="https://www.w3.org/TR/wai-aria/#role_definitions">"valid"</Hyperlink>" role attributes." }
                    .to_owned(),
            mutation,
        ))
    }
}
