use crate::{services::aria::Aria, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    jsx_ext::AnyJsxElement, AnyJsxAttributeValue, AnyNumberLikeExpression, TextRange,
};
use biome_rowan::{AstNode, BatchMutationExt, TokenText};

declare_lint_rule! {
    /// Enforce that `tabIndex` is not assigned to non-interactive HTML elements.
    ///
    /// When using the tab key to navigate a webpage, limit it to interactive elements.
    /// You don't need to add tabindex to items in an unordered list as assistive technology can navigate through the HTML.
    /// Keep the tab ring small, which is the order of elements when tabbing, for a more efficient and accessible browsing experience.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div tabIndex="0" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="article" tabIndex="0" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <article tabIndex="0" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div />
    /// ```
    ///
    /// ```jsx
    /// <MyButton tabIndex={0} />
    /// ```
    ///
    /// ```jsx
    /// <article tabIndex="-1" />
    /// ```
    ///
    pub NoNoninteractiveTabindex {
        version: "1.0.0",
        name: "noNoninteractiveTabindex",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("no-noninteractive-tabindex")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    attribute_range: TextRange,
    element_name: TokenText,
}

impl Rule for NoNoninteractiveTabindex {
    type Query = Aria<AnyJsxElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !node.is_element() {
            return None;
        }

        if ctx.aria_roles().is_not_interactive_element(node) {
            let tabindex_attribute = node.find_attribute_by_name("tabIndex")?;
            let tabindex_attribute_value = tabindex_attribute.initializer()?.value().ok()?;
            if attribute_has_negative_tabindex(&tabindex_attribute_value)? {
                return None;
            }

            let element_name = node
                .name()
                .ok()?
                .as_jsx_name()?
                .value_token()
                .ok()?
                .token_text_trimmed();

            let role_attribute = node.find_attribute_by_name("role");
            let Some(role_attribute) = role_attribute else {
                return Some(RuleState {
                    attribute_range: tabindex_attribute.range(),
                    element_name,
                });
            };

            let role_attribute_value = role_attribute.initializer()?.value().ok()?;
            if attribute_has_interactive_role(&role_attribute_value)? {
                return None;
            }

            return Some(RuleState {
                attribute_range: tabindex_attribute.range(),
                element_name,
            });
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let element_name = state.element_name.text();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.attribute_range,
                markup! {
                "The HTML element "<Emphasis>{{element_name}}</Emphasis>" is non-interactive. Do not use "<Emphasis>"tabIndex"</Emphasis>"."

                },
            )
            .note(markup! {
                "Adding non-interactive elements to the keyboard navigation flow can confuse users."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let tabindex_attribute = node.find_attribute_by_name("tabIndex")?;
        let mut mutation = ctx.root().begin();

        mutation.remove_node(tabindex_attribute);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"tabIndex"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}

/// Verifies if number string is an integer less than 0.
/// Non-integer numbers are considered valid.
fn is_negative_tabindex(number_like_string: &str) -> bool {
    let number_string_result = number_like_string.trim().parse::<i32>();
    match number_string_result {
        Ok(number) => number < 0,
        Err(_) => true,
    }
}

/// Checks if the given tabindex attribute value has negative integer or not.
fn attribute_has_negative_tabindex(
    tabindex_attribute_value: &AnyJsxAttributeValue,
) -> Option<bool> {
    match tabindex_attribute_value {
        AnyJsxAttributeValue::JsxString(jsx_string) => {
            let value = jsx_string.inner_string_text().ok()?.to_string();
            Some(is_negative_tabindex(&value))
        }
        AnyJsxAttributeValue::JsxExpressionAttributeValue(value) => {
            let expression = value.expression().ok()?;
            let expression_value =
                AnyNumberLikeExpression::cast(expression.into_syntax())?.value()?;
            Some(is_negative_tabindex(&expression_value))
        }
        _ => None,
    }
}

/// Checks if the given role attribute value is interactive or not based on ARIA roles.
fn attribute_has_interactive_role(role_attribute_value: &AnyJsxAttributeValue) -> Option<bool> {
    Some(
        AriaRole::from_roles(role_attribute_value.as_static_value()?.text())
            .is_some_and(|role| role.is_interactive()),
    )
}
