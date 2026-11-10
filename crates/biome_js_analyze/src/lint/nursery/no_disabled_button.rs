use crate::JsRuleAction;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsxAttributeName, AnyJsxAttributeValue, JsxAttribute, T, jsx_ext::AnyJsxElement,
    static_value::StaticValue,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_disabled_button::NoDisabledButtonOptions;

declare_lint_rule! {
    /// Discourages `disabled` on elements that act as buttons.
    ///
    /// This is an opinionated usability rule, not a Web Content Accessibility Guidelines (WCAG)
    /// requirement. Native `disabled` controls are removed from sequential focus navigation and
    /// suppress activation. Keeping an unavailable button focusable can make it easier to discover
    /// with a keyboard or assistive technology.
    ///
    /// The rule reports native `button` elements, `input` elements with the `button`, `submit`,
    /// `reset`, or `image` type, and built-in elements whose first valid explicit role is
    /// `button`. A native `button` is still reported when another role is assigned because
    /// `disabled` continues to change its native behavior.
    ///
    /// The unsafe fix replaces `disabled` with `aria-disabled`. The ARIA attribute only exposes
    /// the disabled state to assistive technology. It does not prevent pointer or keyboard
    /// activation, change form submission or validation, omit values from submitted data, or add
    /// disabled styling. Implement those behaviors and provide disabled styling, including for
    /// forced-colors mode.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <button disabled>Save</button>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="button" disabled>Save</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <button aria-disabled="true">Save</button>
    /// ```
    ///
    /// ## References
    ///
    /// - [MDN: `aria-disabled`](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-disabled)
    /// - [Making Disabled Buttons More Inclusive](https://css-tricks.com/making-disabled-buttons-more-inclusive/)
    ///
    pub NoDisabledButton {
        version: "next",
        name: "noDisabledButton",
        language: "jsx",
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    attribute: JsxAttribute,
    kind: ButtonKind,
}

#[derive(Clone, Copy)]
enum ButtonKind {
    Native,
    Role,
}

impl Rule for NoDisabledButton {
    type Query = Ast<AnyJsxElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoDisabledButtonOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        if element.is_custom_component() || element.is_custom_element() {
            return None;
        }

        let attribute = element.find_attribute_by_name("disabled")?;
        if is_inactive_disabled_attribute(&attribute) || element.has_trailing_spread_prop(&attribute) {
            return None;
        }

        let kind = if is_native_button(element)? {
            ButtonKind::Native
        } else {
            if is_native_disabled_control(element)? {
                return None;
            }
            let role_attribute = element.find_attribute_by_name("role")?;
            if element.has_trailing_spread_prop(&role_attribute)
                || AriaRole::from_roles(role_attribute.as_static_value()?.text())?
                    != AriaRole::Button
            {
                return None;
            }
            ButtonKind::Role
        };

        Some(RuleState { attribute, kind })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state.kind {
            ButtonKind::Native => RuleDiagnostic::new(
                rule_category!(),
                state.attribute.range(),
                markup! {
                    "Avoid the native "<Emphasis>"disabled"</Emphasis>" attribute on this button."
                },
            )
            .note(markup! {
                "The native disabled state removes the button from sequential focus navigation, suppresses activation, and changes form behavior."
            }),
            ButtonKind::Role => RuleDiagnostic::new(
                rule_category!(),
                state.attribute.range(),
                markup! {
                    "The "<Emphasis>"disabled"</Emphasis>" attribute has no effect on this element."
                },
            )
            .note(markup! {
                "The "<Emphasis>"button"</Emphasis>" role changes accessibility semantics but does not add native button behavior."
            }),
        };

        Some(diagnostic.note(markup! {
            <Emphasis>"aria-disabled"</Emphasis>" only exposes the disabled state to assistive technology. It does not suppress pointer or keyboard activation, form submission or validation, or apply disabled styling. Implement those behaviors, including forced-colors styling."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if ctx
            .query()
            .find_attribute_by_name("aria-disabled")
            .is_some()
        {
            return None;
        }

        let old_name = state.attribute.name_value_token().ok()?;
        let preserves_initializer = state.attribute.initializer().is_some()
            && state.attribute.as_static_value().is_none();
        if !preserves_initializer && state.attribute.syntax().has_comments_descendants() {
            return None;
        }
        let mut new_name = make::jsx_ident("aria-disabled")
            .with_leading_trivia_pieces(old_name.leading_trivia().pieces());
        if preserves_initializer {
            new_name = new_name.with_trailing_trivia_pieces(old_name.trailing_trivia().pieces());
        }
        let initializer = if preserves_initializer {
            state.attribute.initializer()?
        } else {
            let last_token = state.attribute.syntax().last_token()?;
            let value = make::jsx_string_literal("true")
                .with_trailing_trivia_pieces(last_token.trailing_trivia().pieces());
            make::jsx_attribute_initializer_clause(
                make::token(T![=]),
                AnyJsxAttributeValue::JsxString(make::jsx_string(value)),
            )
        };
        let replacement = make::jsx_attribute(AnyJsxAttributeName::JsxName(make::jsx_name(
            new_name,
        )))
        .with_initializer(initializer)
        .build();

        let mut mutation = ctx.root().begin();
        mutation.replace_node(state.attribute.clone(), replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"aria-disabled"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

fn is_inactive_disabled_attribute(attribute: &JsxAttribute) -> bool {
    match attribute.as_static_value() {
        Some(StaticValue::Boolean(token)) => token.text_trimmed() == "false",
        Some(StaticValue::Null(_) | StaticValue::Undefined(_)) => true,
        _ => false,
    }
}

fn is_native_button(element: &AnyJsxElement) -> Option<bool> {
    if element.matches_name("button").ok()? {
        return Some(true);
    }
    if !element.matches_name("input").ok()? {
        return Some(false);
    }

    let type_attribute = element.find_attribute_by_name("type")?;
    if element.has_trailing_spread_prop(&type_attribute) {
        return Some(false);
    }
    let input_type = type_attribute.as_static_value()?;
    Some(
        ["button", "submit", "reset", "image"]
            .into_iter()
            .any(|button_type| input_type.text().eq_ignore_ascii_case(button_type)),
    )
}

fn is_native_disabled_control(element: &AnyJsxElement) -> Option<bool> {
    for name in [
        "input",
        "select",
        "textarea",
        "fieldset",
        "optgroup",
        "option",
    ] {
        if element.matches_name(name).ok()? {
            return Some(true);
        }
    }
    Some(false)
}
