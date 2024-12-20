use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, BatchMutationExt};
use serde::{Deserialize, Serialize};

declare_lint_rule! {
    /// Elements with ARIA roles must use a valid, non-abstract ARIA role.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="datepicker"></div>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="range"></div>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div role=""></div>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <Foo role="foo"></Foo>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///   <div role="button"></div>
    ///   <div role={role}></div>
    ///   <div></div>
    /// </>
    /// ```
    ///
    /// ## Options
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowInvalidRoles": ["invalid-role", "text"],
    ///         "ignoreNonDom": true
    ///     }
    /// }
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
        version: "1.4.0",
        name: "useValidAriaRole",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("aria-role")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct ValidAriaRoleOptions {
    pub allow_invalid_roles: Box<[Box<str>]>,
    pub ignore_non_dom: bool,
}

impl Rule for UseValidAriaRole {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = Box<ValidAriaRoleOptions>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();

        let ignore_non_dom = options.ignore_non_dom;
        let allowed_invalid_roles = &options.allow_invalid_roles;

        if ignore_non_dom && node.is_custom_component() {
            return None;
        }

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_attribute_static_value = role_attribute.as_static_value()?;
        let role_attribute_value = role_attribute_static_value.text().trim();
        if role_attribute_value.is_empty() {
            return Some(());
        }
        let mut role_attribute_value = role_attribute_value.split_ascii_whitespace();

        let is_valid = role_attribute_value.all(|val| {
            AriaRole::from_roles(val).is_some()
                || allowed_invalid_roles
                    .iter()
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

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let role_attribute = node.find_attribute_by_name("role")?;
        mutation.remove_node(role_attribute);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),

                markup! { "Remove the invalid "<Emphasis>"role"</Emphasis>" attribute.\n Check the list of all "<Hyperlink href="https://www.w3.org/TR/wai-aria/#role_definitions">"valid"</Hyperlink>" role attributes." }
                    .to_owned(),
            mutation,
        ))
    }
}
