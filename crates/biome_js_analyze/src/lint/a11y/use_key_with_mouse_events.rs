use crate::services::semantic::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::{markup, MarkupBuf};
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Enforce `onMouseOver` / `onMouseOut` are accompanied by `onFocus` / `onBlur`.
    ///
    /// Coding for the keyboard is important for users with physical disabilities who cannot use a mouse, AT compatibility, and screenreader users.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div onMouseOver={() => {}} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div onMouseOut={() => {}} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///   <div onMouseOver={() => {}} onFocus={() => {}} />
    ///   <div onMouseOut={() => {}} onBlur={() => {}} />
    ///   <div onMouseOver={() => {}} {...otherProps} />
    ///   <div onMouseOut={() => {}} {...otherProps} />
    ///   <div onMouseOver={() => {}} onFocus={() => {}} {...otherProps} />
    ///   <div onMouseOut={() => {}} onBlur={() => {}} {...otherProps} />
    /// </>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)
    ///
    pub UseKeyWithMouseEvents {
        version: "1.0.0",
        name: "useKeyWithMouseEvents",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("mouse-events-have-key-events")],
        recommended: true,
        severity: Severity::Error,
    }
}

pub enum UseKeyWithMouseEventsState {
    MissingOnFocus,
    MissingOnBlur,
}

impl UseKeyWithMouseEventsState {
    fn message(&self) -> MarkupBuf {
        match self {
            UseKeyWithMouseEventsState::MissingOnBlur => {
                markup! {"onMouseOut must be accompanied by onBlur for accessibility."}.to_owned()
            }
            UseKeyWithMouseEventsState::MissingOnFocus => {
                markup! {"onMouseOver must be accompanied by onFocus for accessibility."}.to_owned()
            }
        }
    }
}

impl Rule for UseKeyWithMouseEvents {
    type Query = Semantic<AnyJsxElement>;
    type State = UseKeyWithMouseEventsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if !node.is_custom_component() {
            if !has_valid_focus_attributes(node) {
                return Some(UseKeyWithMouseEventsState::MissingOnFocus);
            }

            if !has_valid_blur_attributes(node) {
                return Some(UseKeyWithMouseEventsState::MissingOnBlur);
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let footer_note_text = markup! {"Actions triggered using mouse events should have corresponding events to account for keyboard-only navigation."};

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                state.message(),
            )
            .note(footer_note_text),
        )
    }
}

fn has_valid_focus_attributes(elem: &AnyJsxElement) -> bool {
    if let Some(on_mouse_over_attribute) = elem.find_attribute_by_name("onMouseOver") {
        if !elem.has_trailing_spread_prop(&on_mouse_over_attribute) {
            return elem.find_attribute_by_name("onFocus").is_some_and(|it| {
                !it.as_static_value()
                    .is_some_and(|value| value.is_null_or_undefined())
            });
        }
    }
    true
}

fn has_valid_blur_attributes(elem: &AnyJsxElement) -> bool {
    if let Some(on_mouse_attribute) = elem.find_attribute_by_name("onMouseOut") {
        if !elem.has_trailing_spread_prop(&on_mouse_attribute) {
            return elem.find_attribute_by_name("onBlur").is_some_and(|it| {
                !it.as_static_value()
                    .is_some_and(|value| value.is_null_or_undefined())
            });
        }
    }
    true
}
