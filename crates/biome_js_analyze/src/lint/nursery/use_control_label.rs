use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::static_value::StaticValue;
use biome_js_syntax::{AnyJsxChild, JsxChildList, JsxElement};
use biome_rowan::AstNode;
use biome_rule_options::use_control_label::UseControlLabelOptions;

declare_lint_rule! {
    /// Enforce that interactive control elements have an accessible label.
    ///
    /// A control with no accessible label is announced by assistive technology
    /// as an anonymous control (e.g. just "button"), leaving its purpose
    /// unclear. A label can come from text content anywhere inside the
    /// control, `aria-label`, `aria-labelledby`, or `title` attribute.
    ///
    /// This rule checks native controls whose accessible name is expected to
    /// come from their own content or attributes (`button`, `menuitem`).
    /// Elements hidden from assistive technology with `aria-hidden` are
    /// skipped, as are elements that already require a text alternative under
    /// a dedicated rule (e.g. `area`, `img`, checked by `useAltText`).
    ///
    /// The search through the content of a control is permissive: anything
    /// whose rendered output cannot be determined statically, such as an
    /// expression, a spread, or a custom component, is assumed to provide a
    /// label.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <button />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <button></button>;
    /// ```
    ///
    /// An icon button whose content renders nothing announceable:
    ///
    /// ```jsx,expect_diagnostic
    /// <button><i className="icon-save" /></button>;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <button>Submit</button>;
    /// ```
    ///
    /// ```jsx
    /// <button aria-label="Close" />;
    /// ```
    ///
    /// ```jsx
    /// <button><Icon /><span>Delete</span></button>;
    /// ```
    ///
    /// ```jsx
    /// <button><img src="save.png" alt="Save" /></button>;
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 1.3.1](https://www.w3.org/WAI/WCAG21/Understanding/info-and-relationships)
    /// - [WCAG 3.3.2](https://www.w3.org/WAI/WCAG21/Understanding/labels-or-instructions)
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    pub UseControlLabel {
        version: "2.5.9",
        name: "useControlLabel",
        language: "jsx",
        recommended: false,
        sources: &[RuleSource::EslintJsxA11y("control-has-associated-label").inspired()],
        severity: Severity::Error,
    }
}

/// Native interactive elements whose accessible name comes from their own
/// content or labeling attributes (rather than an external `<label>` or a
/// dedicated alt-text rule).
const CONTROL_ELEMENTS: &[&str] = &["button", "menuitem"];

/// Attributes that supply an accessible name for these controls.
const LABEL_ATTRIBUTES: &[&str] = &["aria-label", "aria-labelledby", "title"];

/// Void elements that render nothing an assistive technology can announce, so
/// they never contribute to the accessible name of an ancestor control.
const EMPTY_CONTENT_ELEMENTS: &[&str] = &["br", "hr", "wbr", "meta", "link", "base", "col"];

impl Rule for UseControlLabel {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseControlLabelOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        if element.is_custom_component() {
            return None;
        }

        let name = element.name_value_token().ok()?;
        if !CONTROL_ELEMENTS.contains(&name.text_trimmed()) {
            return None;
        }

        // An element hidden from the accessibility tree does not need a label. A
        // bare `aria-hidden` counts as `true`, while a falsy value (`false`,
        // `null`, `undefined`, `""`) leaves the control exposed.
        if element.has_truthy_attribute("aria-hidden") {
            return None;
        }

        // A non-empty labeling attribute supplies the accessible name.
        if has_labeling_attribute(element) {
            return None;
        }

        // Props that can carry a label the rule cannot read leave the outcome
        // undecidable, so the control is left alone.
        if has_opaque_label_source(element) {
            return None;
        }

        // Otherwise the name must come from accessible child content. Only an
        // opening element can have children; a self-closing control cannot.
        let has_content = match element {
            AnyJsxElement::JsxOpeningElement(opening) => opening
                .parent::<JsxElement>()
                .is_some_and(|parent| has_accessible_content(&parent.children())),
            AnyJsxElement::JsxSelfClosingElement(_) => false,
        };
        if has_content {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This control element has no accessible label."
                },
            )
            .note(markup! {
                "Assistive technology announces it as an anonymous control, so its purpose is unclear to screen-reader users."
            })
            .note(markup! {
                "Add text content, or an "<Emphasis>"aria-label"</Emphasis>", "<Emphasis>"aria-labelledby"</Emphasis>", or "<Emphasis>"title"</Emphasis>" attribute."
            }),
        )
    }
}

/// Whether any descendant of a control supplies part of its accessible name.
///
/// The walk is unbounded and deliberately permissive: anything whose rendered
/// output cannot be determined statically (an expression, a spread, a custom
/// component) counts as a label, so only a subtree that is provably silent
/// reports.
fn has_accessible_content(children: &JsxChildList) -> bool {
    children.into_iter().any(|child| match child {
        AnyJsxChild::JsxText(text) => text
            .value_token()
            .is_ok_and(|token| !token.text_trimmed().trim().is_empty()),
        AnyJsxChild::JsxExpressionChild(expression) => {
            expression
                .expression()
                .is_some_and(|expression| match expression.as_static_value() {
                    None => true,
                    Some(value) => renders_content(&value),
                })
        }
        AnyJsxChild::JsxFragment(fragment) => has_accessible_content(&fragment.children()),
        AnyJsxChild::JsxElement(element) => {
            let Ok(opening) = element.opening_element() else {
                return true;
            };
            names_itself(&AnyJsxElement::from(opening))
                .unwrap_or_else(|| has_accessible_content(&element.children()))
        }
        AnyJsxChild::JsxSelfClosingElement(element) => {
            names_itself(&AnyJsxElement::from(element)).unwrap_or(false)
        }
        _ => true,
    })
}

/// Whether a descendant element names itself, independently of its children.
///
/// `Some(true)` when it supplies a name, `Some(false)` when it can never
/// supply one, and `None` when the answer depends on its own children.
fn names_itself(element: &AnyJsxElement) -> Option<bool> {
    // A hidden subtree is skipped whole: it contributes nothing even if it
    // holds text.
    if element.has_truthy_attribute("aria-hidden") {
        return Some(false);
    }

    if element.is_custom_component()
        || has_opaque_label_source(element)
        || has_labeling_attribute(element)
    {
        return Some(true);
    }

    match element.name_value_token().ok()?.text_trimmed() {
        // An image is named by its alt text; an empty `alt` marks it decorative.
        "img" if has_non_empty_attribute(element, "alt") => Some(true),
        "img" => Some(false),
        // A hidden input renders nothing; any other input is a control whose
        // own label this rule does not resolve.
        "input" => Some(!is_hidden_input(element)),
        name if EMPTY_CONTENT_ELEMENTS.contains(&name) => Some(false),
        _ => None,
    }
}

/// Whether a statically known value renders text an assistive technology can
/// announce.
///
/// This is about rendered output, not truthiness: JSX omits both Boolean
/// values and nullish values, and whitespace renders nothing announceable,
/// while every number renders as its own text, `0` and `0n` included.
fn renders_content(value: &StaticValue) -> bool {
    match value {
        StaticValue::Boolean(_)
        | StaticValue::Null(_)
        | StaticValue::Undefined(_)
        | StaticValue::EmptyString(_) => false,
        StaticValue::String(_) => !value.text().trim().is_empty(),
        StaticValue::Number(_) | StaticValue::BigInt(_) => true,
    }
}

/// Whether the element is an `<input type="hidden">`, which renders nothing.
fn is_hidden_input(element: &AnyJsxElement) -> bool {
    element
        .find_attribute_by_name("type")
        .and_then(|attribute| attribute.as_static_value())
        .is_some_and(|value| value.text().eq_ignore_ascii_case("hidden"))
}

/// Whether the element carries a prop that can supply content or a labeling
/// attribute the rule cannot inspect: a prop that injects children
/// (`dangerouslySetInnerHTML`, `innerHTML`, a `children` prop that renders
/// something), or a spread whose members are unknown (`{...props}`).
fn has_opaque_label_source(element: &AnyJsxElement) -> bool {
    element
        .find_attribute_by_name("dangerouslySetInnerHTML")
        .is_some()
        || element.find_attribute_by_name("innerHTML").is_some()
        || element
            .find_attribute_by_name("children")
            .is_some_and(|attribute| {
                // A valueless `children` is `true`, which renders nothing.
                if attribute.initializer().is_none() {
                    return false;
                }
                attribute
                    .as_static_value()
                    .is_none_or(|value| renders_content(&value))
            })
        || element.has_spread_prop()
}

/// Whether the element carries a labeling attribute with a non-empty value.
fn has_labeling_attribute(element: &AnyJsxElement) -> bool {
    LABEL_ATTRIBUTES
        .iter()
        .any(|name| has_non_empty_attribute(element, name))
}

/// Whether the named attribute is present and carries a usable value. A
/// dynamic value (not statically known, e.g. `aria-label={label}`) is assumed
/// to provide one; an empty or whitespace-only literal (`aria-label=""`,
/// `aria-label={``}`, `aria-label="  "`) does not, nor does a literal `null`
/// or `undefined` (e.g. `aria-label={null}`).
fn has_non_empty_attribute(element: &AnyJsxElement, name: &str) -> bool {
    let Some(attribute) = element.find_attribute_by_name(name) else {
        return false;
    };
    match attribute.as_static_value() {
        None => true,
        Some(value) => match value {
            StaticValue::String(_) => !value.text().trim().is_empty(),
            StaticValue::EmptyString(_) => false,
            StaticValue::Null(_) | StaticValue::Undefined(_) => false,
            _ => true,
        },
    }
}
