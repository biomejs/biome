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
use biome_rule_options::no_disabled_attribute::NoDisabledAttributeOptions;

declare_lint_rule! {
    /// Discourages the `disabled` attribute on form controls and on elements that act as buttons.
    ///
    /// This is an opinionated usability rule, not a Web Content Accessibility Guidelines (WCAG)
    /// requirement. A form control with the native `disabled` attribute can't receive keyboard
    /// focus, so people who move through a page with the Tab key skip over it. Keeping an
    /// unavailable control focusable can make it easier to discover with a keyboard or assistive
    /// technology.
    ///
    /// The rule reports the following elements when `disabled` is set:
    ///
    /// - Buttons: `button` elements, and `input` elements with the `button`, `submit`, `reset`,
    ///   or `image` type. A `button` is still reported when a role other than `button` is
    ///   assigned, because `disabled` continues to change its native behavior.
    /// - Fields that support the `readOnly` attribute: `textarea` elements, and `input` elements
    ///   whose type accepts typed input, such as `text`, `email`, `number`, or `date`. An `input`
    ///   with no `type`, or with a `type` that browsers don't recognize, is a text field.
    /// - Other form controls: `select` elements, and `input` elements with the `checkbox`,
    ///   `radio`, `range`, `color`, or `file` type.
    /// - `fieldset` elements, because a disabled `fieldset` disables every form control inside it.
    /// - Built-in elements that aren't form controls and whose first valid explicit role is
    ///   `button`. The `disabled` attribute has no effect on these elements.
    ///
    /// The rule doesn't report `option` and `optgroup` elements, `input` elements with the
    /// `hidden` type, or `input` elements whose `type` is a dynamic expression or can come from
    /// a spread attribute.
    ///
    /// ## Replacing `disabled`
    ///
    /// The unsafe fix depends on the reported element:
    ///
    /// - For fields that support it, the fix replaces `disabled` with `readOnly`. A read-only
    ///   field can receive focus and can't be edited. Unlike a disabled field, its value is
    ///   still submitted with the form, and browsers don't apply disabled styling to it.
    /// - For buttons, other form controls, and elements with the `button` role, the fix replaces
    ///   `disabled` with `aria-disabled`. The ARIA attribute only exposes the disabled state to
    ///   assistive technology. It does not prevent pointer or keyboard activation, change form
    ///   submission or validation, omit values from submitted data, or add disabled styling.
    ///   Implement those behaviors and provide disabled styling, including for forced-colors
    ///   mode.
    /// - A `fieldset` has no fix. Mark each form control inside it instead. Disabling a
    ///   `fieldset` can be appropriate when the whole group doesn't apply to the user, for
    ///   example the details of a second applicant when there is only one applicant. In that
    ///   case, suppress the diagnostic.
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
    /// <input type="email" disabled />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <select disabled><option>Monthly</option></select>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <fieldset disabled><input type="text" /></fieldset>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="button" disabled>Save</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///     <button aria-disabled="true">Save</button>
    ///     <input type="email" readOnly />
    ///     <select aria-disabled="true"><option>Monthly</option></select>
    ///     <input type="hidden" disabled />
    /// </>
    /// ```
    ///
    /// ## References
    ///
    /// - [MDN: `aria-disabled`](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-disabled)
    /// - [MDN: `readonly`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/readonly)
    /// - [Making Disabled Buttons More Inclusive](https://css-tricks.com/making-disabled-buttons-more-inclusive/)
    ///
    pub NoDisabledAttribute {
        version: "2.6.7",
        name: "noDisabledAttribute",
        language: "jsx",
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    attribute: JsxAttribute,
    kind: ControlKind,
}

/// What an element with a `disabled` attribute is, which decides the advice and the fix.
#[derive(Clone, Copy)]
enum ControlKind {
    /// A `button` element, or an `input` element with a button type.
    Button,
    /// A form control that supports the `readonly` attribute.
    EditableField,
    /// A form control that supports neither button activation nor `readonly`.
    FormControl,
    Fieldset,
    /// An element that isn't a form control and has the `button` role.
    ButtonRole,
}

impl Rule for NoDisabledAttribute {
    type Query = Ast<AnyJsxElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoDisabledAttributeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        if element.is_custom_component() || element.is_custom_element() {
            return None;
        }

        let attribute = element.find_attribute_by_name("disabled")?;
        if is_inactive_disabled_attribute(&attribute) || element.has_trailing_spread_prop(&attribute) {
            return None;
        }

        let kind = ControlKind::from_element(element)?;
        Some(RuleState { attribute, kind })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.attribute.range(),
                match state.kind {
                    ControlKind::ButtonRole => markup! {
                        "The "<Emphasis>"disabled"</Emphasis>" attribute has no effect on this element."
                    },
                    ControlKind::Button => markup! {
                        "Avoid the native "<Emphasis>"disabled"</Emphasis>" attribute on this button."
                    },
                    ControlKind::EditableField => markup! {
                        "Avoid the native "<Emphasis>"disabled"</Emphasis>" attribute on this field."
                    },
                    ControlKind::FormControl => markup! {
                        "Avoid the native "<Emphasis>"disabled"</Emphasis>" attribute on this form control."
                    },
                    ControlKind::Fieldset => markup! {
                        "Avoid the native "<Emphasis>"disabled"</Emphasis>" attribute on this fieldset."
                    },
                },
            )
            .note(match state.kind {
                ControlKind::ButtonRole => markup! {
                    "The "<Emphasis>"button"</Emphasis>" role changes accessibility semantics but does not add native button behavior."
                },
                ControlKind::Fieldset => markup! {
                    "A disabled fieldset disables every form control inside it, so none of them can be reached with the Tab key."
                },
                ControlKind::Button | ControlKind::EditableField | ControlKind::FormControl => markup! {
                    "A disabled form control can't receive keyboard focus, so it can't be reached with the Tab key."
                },
            })
            .note(match state.kind {
                ControlKind::EditableField => markup! {
                    "Replace it with "<Emphasis>"readOnly"</Emphasis>", which keeps the field focusable and prevents edits. Unlike a disabled field, a read-only field is still submitted with its form and doesn't get disabled styling."
                },
                ControlKind::Fieldset => markup! {
                    "Mark each form control inside the fieldset instead: use "<Emphasis>"readOnly"</Emphasis>" on fields that support it and "<Emphasis>"aria-disabled"</Emphasis>" on other controls."
                },
                ControlKind::Button | ControlKind::FormControl | ControlKind::ButtonRole => markup! {
                    "Replace it with "<Emphasis>"aria-disabled"</Emphasis>", which only exposes the disabled state to assistive technology. It does not suppress pointer or keyboard activation, form submission or validation, or apply disabled styling. Implement those behaviors, including forced-colors styling."
                },
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let element = ctx.query();
        // The static value is what replaces a `disabled` attribute that is always active.
        let (new_name, static_value) = match state.kind {
            ControlKind::Fieldset => return None,
            ControlKind::EditableField => {
                if element.find_attribute_by_name("readonly").is_some() {
                    return None;
                }
                ("readOnly", None)
            }
            ControlKind::Button | ControlKind::FormControl | ControlKind::ButtonRole => {
                ("aria-disabled", Some("true"))
            }
        };
        if element.find_attribute_by_name(new_name).is_some() {
            return None;
        }

        let old_name = state.attribute.name_value_token().ok()?;
        let preserves_initializer = state.attribute.initializer().is_some()
            && state.attribute.as_static_value().is_none();
        if !preserves_initializer && state.attribute.syntax().has_comments_descendants() {
            return None;
        }
        let new_name_token = make::jsx_ident(new_name)
            .with_leading_trivia_pieces(old_name.leading_trivia().pieces());
        let replacement = if preserves_initializer {
            let new_name_token =
                new_name_token.with_trailing_trivia_pieces(old_name.trailing_trivia().pieces());
            make::jsx_attribute(AnyJsxAttributeName::JsxName(make::jsx_name(new_name_token)))
                .with_initializer(state.attribute.initializer()?)
                .build()
        } else {
            let last_token = state.attribute.syntax().last_token()?;
            let trailing_trivia = last_token.trailing_trivia().pieces();
            match static_value {
                Some(static_value) => {
                    let value = make::jsx_string_literal(static_value)
                        .with_trailing_trivia_pieces(trailing_trivia);
                    make::jsx_attribute(AnyJsxAttributeName::JsxName(make::jsx_name(
                        new_name_token,
                    )))
                    .with_initializer(make::jsx_attribute_initializer_clause(
                        make::token(T![=]),
                        AnyJsxAttributeValue::JsxString(make::jsx_string(value)),
                    ))
                    .build()
                }
                None => make::jsx_attribute(AnyJsxAttributeName::JsxName(make::jsx_name(
                    new_name_token.with_trailing_trivia_pieces(trailing_trivia),
                )))
                .build(),
            }
        };

        let mut mutation = ctx.root().begin();
        mutation.replace_node(state.attribute.clone(), replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{new_name}</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

impl ControlKind {
    /// Classifies an element that has a `disabled` attribute.
    ///
    /// Returns `None` for elements the rule doesn't report, and for `input` elements whose
    /// `type` isn't a static value.
    fn from_element(element: &AnyJsxElement) -> Option<Self> {
        let name = element.name_value_token().ok()?;
        match name.text_trimmed() {
            "button" => Some(Self::Button),
            "input" => Self::from_input(element),
            "textarea" => Some(Self::EditableField),
            "select" => Some(Self::FormControl),
            "fieldset" => Some(Self::Fieldset),
            // These elements are never reached with the Tab key, whether disabled or not.
            "optgroup" | "option" => None,
            _ => {
                let role_attribute = element.find_attribute_by_name("role")?;
                if element.has_trailing_spread_prop(&role_attribute) {
                    return None;
                }
                let role = AriaRole::from_roles(role_attribute.as_static_value()?.text())?;
                (role == AriaRole::Button).then_some(Self::ButtonRole)
            }
        }
    }

    /// Classifies an `input` element by its `type` attribute.
    fn from_input(element: &AnyJsxElement) -> Option<Self> {
        let Some(type_attribute) = element.find_attribute_by_name("type") else {
            // An `input` without a `type` is a text field, unless a spread or shorthand
            // attribute sets one.
            let has_dynamic_type =
                element.has_spread_prop() || element.has_shorthand_attribute("type");
            return (!has_dynamic_type).then_some(Self::EditableField);
        };
        if element.has_trailing_spread_prop(&type_attribute) {
            return None;
        }
        let input_type = type_attribute.as_static_value()?;
        let input_type = input_type.text();
        if input_type.eq_ignore_ascii_case("hidden") {
            // A hidden input is never focusable. `disabled` only keeps it out of the submitted data.
            None
        } else if is_one_of(input_type, &BUTTON_INPUT_TYPES) {
            Some(Self::Button)
        } else if is_one_of(input_type, &INPUT_TYPES_WITHOUT_READONLY) {
            Some(Self::FormControl)
        } else {
            // Browsers treat every other type, including unknown ones, as a field that
            // supports `readonly`.
            Some(Self::EditableField)
        }
    }
}

const BUTTON_INPUT_TYPES: [&str; 4] = ["button", "submit", "reset", "image"];

/// The `input` types, other than the button types and `hidden`, that don't support `readonly`.
const INPUT_TYPES_WITHOUT_READONLY: [&str; 5] = ["checkbox", "radio", "range", "color", "file"];

/// Returns `true` if `input_type` is one of `types`, ignoring ASCII case.
fn is_one_of(input_type: &str, types: &[&str]) -> bool {
    types
        .iter()
        .any(|candidate| input_type.eq_ignore_ascii_case(candidate))
}

fn is_inactive_disabled_attribute(attribute: &JsxAttribute) -> bool {
    match attribute.as_static_value() {
        Some(StaticValue::Boolean(token)) => token.text_trimmed() == "false",
        Some(StaticValue::Null(_) | StaticValue::Undefined(_)) => true,
        _ => false,
    }
}
