use std::str::FromStr;

use crate::{aria_services::Aria, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_deserialize::json::{report_unknown_map_key, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, VisitNode};
use biome_diagnostics::Applicability;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_json_syntax::JsonLanguage;
use biome_rowan::{AstNode, BatchMutationExt, SyntaxNode};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

declare_rule! {
    /// Elements with ARIA roles must use a valid, non-abstract ARIA role.
    ///
    /// Source: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-role.md
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    ///  ```js,expect_diagnostic
    ///   <div role="datepicker"></div>
    ///  ```
    ///
    ///  ```js,expect_diagnostic
    ///   <div role="range"></div>
    ///  ```
    ///
    ///  ```js,expect_diagnostic
    ///   <div role=""></div>
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///   <Foo role="foo"></Foo>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// <>
    ///   <div role="button"></div>
    ///   <div role={role}></div>
    ///   <div></div>
    /// </>
    /// ```
    ///
    /// ### Options
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "allowInvalidRoles": ["invalid", "text"],
    ///         "nonIgnoreDom": true
    ///     }
    /// }
    /// ```
    ///
    /// ## Accessibility guidelines
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    /// ## Resources
    /// - [Chrome Audit Rules, AX_ARIA_01](https://github.com/GoogleChrome/accessibility-developer-tools/wiki/Audit-Rules#ax_aria_01)
    /// - [DPUB-ARIA roles](https://www.w3.org/TR/dpub-aria-1.0/)
    /// - [MDN: Using ARIA: Roles, states, and properties](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/ARIA_Techniques)
    ///
    pub(crate) UseValidAriaRole {
        version: "next",
        name: "useValidAriaRole",
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Default, Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidAriaRoleOptions {
    #[bpaf(hide, argument::<String>("roles"), many)]
    allowed_invalid_roles: Vec<String>,
    #[bpaf(hide)]
    ignore_non_dom: bool,
}

impl ValidAriaRoleOptions {
    pub const ALLOWED_KEYS: &'static [&'static str] = &["allowInvalidRoles", "ignoreNonDom"];
}

impl FromStr for ValidAriaRoleOptions {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(ValidAriaRoleOptions::default())
    }
}

impl VisitNode<JsonLanguage> for ValidAriaRoleOptions {
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.inner_string_text().ok()?;
        let name_text = name_text.text();
        match name_text {
            "allowInvalidRoles" => {
                self.allowed_invalid_roles =
                    self.map_to_array_of_strings(&value, name_text, diagnostics)?;
            }
            "ignoreNonDom" => {
                self.ignore_non_dom = self.map_to_boolean(&value, name_text, diagnostics)?;
            }
            _ => {
                report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
            }
        }

        Some(())
    }
}

impl Rule for UseValidAriaRole {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ValidAriaRoleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        let aria_roles = ctx.aria_roles();

        let ignore_non_dom = options.ignore_non_dom;
        let allowed_invalid_roles = &options.allowed_invalid_roles;

        if ignore_non_dom && node.is_custom_component() {
            return None;
        }

        let role_attribute = node.find_attribute_by_name("role")?;

        let role_attribute_static_value = role_attribute.as_static_value()?;
        let role_attribute_value = role_attribute_static_value.text();
        let mut role_attribute_value = role_attribute_value.split(' ');

        let is_valid = role_attribute_value.all(|val| {
            let role_data = aria_roles.get_role(val);
            allowed_invalid_roles.contains(&val.to_string()) || role_data.is_some()
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

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message:
                markup! { "Remove the invalid "<Emphasis>"role"</Emphasis>" attribute.\n Check the list of all "<Hyperlink href="https://www.w3.org/TR/wai-aria/#role_definitions">"valid"</Hyperlink>" role attributes." }
                    .to_owned(),
            mutation,
        })
    }
}
