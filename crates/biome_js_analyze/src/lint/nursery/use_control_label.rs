use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;
use biome_rule_options::use_control_label::UseControlLabelOptions;

declare_lint_rule! {
    /// Enforce that interactive control elements have an accessible label.
    ///
    /// A control with no accessible label is announced by assistive technology
    /// as an anonymous control (e.g. just "button"), leaving its purpose
    /// unclear. A label can come from text content, or from an `aria-label`,
    /// `aria-labelledby`, `title`, or `alt` attribute.
    ///
    /// This rule checks self-labeled native controls (`button`, `area`,
    /// `menuitem`). Elements hidden from assistive technology with
    /// `aria-hidden` are skipped.
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
    pub UseControlLabel {
        version: "next",
        name: "useControlLabel",
        language: "jsx",
        recommended: false,
        sources: &[RuleSource::EslintJsxA11y("control-has-associated-label").same()],
    }
}

/// Native interactive elements whose accessible name comes from their own
/// content or labeling attributes (rather than an external `<label>`).
const CONTROL_ELEMENTS: &[&str] = &["button", "area", "menuitem"];

/// Attributes that supply an accessible name.
const LABEL_ATTRIBUTES: &[&str] = &["aria-label", "aria-labelledby", "title", "alt"];

impl Rule for UseControlLabel {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseControlLabelOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        let name = element.name_value_token().ok()?;
        if !CONTROL_ELEMENTS.contains(&name.text_trimmed()) {
            return None;
        }

        // An element hidden from the accessibility tree does not need a label.
        if element.find_attribute_by_name("aria-hidden").is_some() {
            return None;
        }

        // A labeling attribute supplies the accessible name.
        if LABEL_ATTRIBUTES
            .iter()
            .any(|attribute| element.find_attribute_by_name(attribute).is_some())
        {
            return None;
        }

        // Otherwise the name must come from accessible child content. Only an
        // opening element can have children; a self-closing control cannot.
        let has_content = match element {
            AnyJsxElement::JsxOpeningElement(opening) => opening.has_accessible_child(),
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
