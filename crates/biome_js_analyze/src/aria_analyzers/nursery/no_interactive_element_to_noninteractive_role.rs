use std::{collections::HashMap, str::FromStr};

use crate::aria_services::Aria;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_deserialize::{Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, VisitableType, Text};
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, TextRange};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_rule! {
    /// Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements.
    ///
    /// Interactive HTML elements indicate controls in the user interface.
    /// Interactive elements include `<a href>`, `<button>`, `<input>`, `<select>`, `<textarea>`.
    /// Non-interactive HTML elements and non-interactive ARIA roles indicate content and containers in the user interface.
    /// Non-interactive elements include `<main>`, `<area>`, `<h1>` (,`<h2>`, etc), `<img>`, `<li>`, `<ul>` and `<ol>`.
    ///
    /// [WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) should not be used to convert an interactive element to a non-interactive element.
    /// Non-interactive ARIA roles include `article`, `banner`, `complementary`, `img`, `listitem`, `main`, `region` and `tooltip`.
    ///
    /// Source: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-interactive-element-to-noninteractive-role.md
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <input role="img" />;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```jsx
    /// <input role="button" />;
    /// ```
    ///
    pub(crate) NoInteractiveElementToNoninteractiveRole {
        version: "1.3.0",
        name: "noInteractiveElementToNoninteractiveRole",
        recommended: false,
    }
}

/// Options for the rule `noInteractiveElementToNoninteractiveRole`
#[derive(Default, Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InteractiveElementToNoninteractiveRoleOptions {
    #[bpaf(external, hide, many)]
    allowed_roles: Vec<String>,
}

impl FromStr for InteractiveElementToNoninteractiveRoleOptions {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(InteractiveElementToNoninteractiveRoleOptions::default())
    }
}

impl Deserializable for InteractiveElementToNoninteractiveRoleOptions {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(InteractiveElementToNoninteractiveRoleOptionsVisitor, name, diagnostics)
    }
}

struct InteractiveElementToNoninteractiveRoleOptionsVisitor;
impl DeserializationVisitor for InteractiveElementToNoninteractiveRoleOptionsVisitor {
    type Output = InteractiveElementToNoninteractiveRoleOptions;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<
            Item = Option<(
                impl biome_deserialize::DeserializableValue,
                impl biome_deserialize::DeserializableValue,
            )>,
        >,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["allowed_roles"];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "allowed_roles" => {
                    let val_range = value.range();
                    result.allowed_roles = Deserializable::deserialize(&value, &key_text, diagnostics)
                        .unwrap_or_default();
                    if result.allowed_roles.is_empty() {
                        diagnostics.push(
                            DeserializationDiagnostic::new("At least one element is needed")
                                .with_range(val_range),
                        );
                    }
                }
                text => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    text,
                    key.range(),
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(result)
    }
}

#[derive(Default, Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ElementRoles {
    #[bpaf(hide)]
    pub element: String,
    #[bpaf(hide, argument::<String>("roles"), many)]
    pub roles: Vec<String>,
}

impl FromStr for ElementRoles {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(ElementRoles::default())
    }
}

impl Deserializable for ElementRoles {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(ElementRolesVisitor, name, diagnostics)
    }
}

struct ElementRolesVisitor;
impl DeserializationVisitor for ElementRolesVisitor {
    type Output = ElementRoles;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["element", "roles"];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "element" => {
                    let val_range = value.range();
                    result.element = Deserializable::deserialize(&value, &key_text, diagnostics)
                        .unwrap_or_default();
                    if result.element.is_empty() {
                        diagnostics.push(
                            DeserializationDiagnostic::new(markup!(
                                "The field "<Emphasis>"name"</Emphasis>" is mandatory"
                            ))
                            .with_range(val_range),
                        )
                    }
                }
                "roles" => {
                    if let Some(roles) = Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        result.roles = roles;
                    }
                }
                unknown_key => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(result)
    }
}

impl Rule for NoInteractiveElementToNoninteractiveRole {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = InteractiveElementToNoninteractiveRoleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_roles = ctx.aria_roles();
        let options = ctx.options();

        let allowed_roles = options.allowed_roles;

        if node.is_element() {
            let role_attribute = node.find_attribute_by_name("role")?;
            let role_attribute_static_value = role_attribute.as_static_value()?;
            let role_attribute_value = role_attribute_static_value.text();
            let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;

            let attributes = ctx.extract_attributes(&node.attributes());

            // check for valid role in options
            for element_and_role in allowed_roles.iter() {
                let element = element_and_role.element.as_str();
                let roles = element_and_role.roles;

                if element == element_name.text_trimmed() && roles.contains(&element_name.text_trimmed().to_string()) {
                    return None;
                }
            }
            
            if !aria_roles.is_not_interactive_element(element_name.text_trimmed(), attributes)
                && !aria_roles.is_role_interactive(role_attribute_value)
            {
                // <div> and <span> are considered neither interactive nor non-interactive, depending on the presence or absence of the role attribute.
                // We don't report <div> and <span> here, because we cannot determine whether they are interactive or non-interactive.
                let role_sensitive_elements = ["div", "span"];
                if role_sensitive_elements.contains(&element_name.text_trimmed()) {
                    return None;
                }

                // a tag without href is considered non-interactive
                if element_name.text_trimmed() == "a"
                    && node.find_attribute_by_name("href").is_none()
                {
                    return None;
                }

                return Some(());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),

                "Interactive elements should not be assigned non-interactive roles."

            )
            .note(
                "WAI-ARIA roles should not be used to convert an interactive element to a non-interactive element."
            )
            .note(
                "Wrap your interactive element in a <div> with the desired role or put the content inside your interactive element."
            ),
        )
    }
}
