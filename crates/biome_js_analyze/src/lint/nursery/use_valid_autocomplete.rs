use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, TextRange};

declare_lint_rule! {
    /// Use valid values for the `autocomplete` attribute on `input` elements.
    ///
    /// The HTML autocomplete attribute only accepts specific predefined values.
    /// This allows for more detailed purpose definitions compared to the `type` attribute.
    /// Using these predefined values, user agents and assistive technologies can present input purposes to users in different ways.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <input type="text" autocomplete="incorrect" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///   <input type="text" autocomplete="name" />
    ///   <MyInput autocomplete="incorrect" />
    /// </>
    /// ```
    ///
    /// ## Options
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "inputComponents": ["MyInput"]
    ///     }
    /// }
    /// ```
    ///
    /// ## Accessibility guidelines
    /// - [WCAG 1.3.5](https://www.w3.org/WAI/WCAG21/Understanding/identify-input-purpose)
    ///
    /// ### Resources
    /// - [HTML Living Standard autofill](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#autofill)
    /// - [HTML attribute: autocomplete - HTML: HyperText Markup Language | MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete)
    ///
    pub UseValidAutocomplete {
        version: "1.9.0",
        name: "useValidAutocomplete",
        language: "js",
        sources: &[RuleSource::EslintJsxA11y("autocomplete-valid")],
        recommended: false,
    }
}

// Sorted for binary search
const VALID_AUTOCOMPLETE_VALUES: [&str; 55] = [
    "additional-name",
    "address-level1",
    "address-level2",
    "address-level3",
    "address-level4",
    "address-line1",
    "address-line2",
    "address-line3",
    "bday",
    "bday-day",
    "bday-month",
    "bday-year",
    "cc-additional-name",
    "cc-csc",
    "cc-exp",
    "cc-exp-month",
    "cc-exp-year",
    "cc-family-name",
    "cc-given-name",
    "cc-name",
    "cc-number",
    "cc-type",
    "country",
    "country-name",
    "current-password",
    "email",
    "family-name",
    "given-name",
    "honorific-prefix",
    "honorific-suffix",
    "impp",
    "language",
    "name",
    "new-password",
    "nickname",
    "off",
    "on",
    "one-time-code",
    "organization",
    "organization-title",
    "photo",
    "postal-code",
    "sex",
    "street-address",
    "tel",
    "tel-area-code",
    "tel-country-code",
    "tel-extension",
    "tel-local",
    "tel-national",
    "transaction-amount",
    "transaction-currency",
    "url",
    "username",
    "webauthn",
];

// Sorted for binary search
const BILLING_AND_SHIPPING_ADDRESS: &[&str; 11] = &[
    "address-level1",
    "address-level2",
    "address-level3",
    "address-level4",
    "address-line1",
    "address-line2",
    "address-line3",
    "country",
    "country-name",
    "postal-code",
    "street-address",
];

#[derive(
    Clone, Debug, Default, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseValidAutocompleteOptions {
    /// `input` like custom components that should be checked.
    pub input_components: Box<[Box<str>]>,
}

impl Rule for UseValidAutocomplete {
    type Query = Ast<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = Box<UseValidAutocompleteOptions>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let input_components = &ctx.options().input_components;

        let elem_name = node.name().ok()?.name_value_token().ok()?;
        let elem_name = elem_name.text_trimmed();
        if elem_name != "input" && input_components.iter().all(|x| x.as_ref() != elem_name) {
            return None;
        }

        let autocomplete_attribute = node.attributes().find_by_name("autocomplete")?;
        let autocomplete_val = autocomplete_attribute.as_static_value()?;
        let autocompletes = autocomplete_val
            .text()
            .split_ascii_whitespace()
            .collect::<smallvec::SmallVec<[&str; 2]>>();
        if (autocompletes.len() == 1 && autocompletes[0] == "none")
            || is_valid_autocomplete(&autocompletes)
        {
            return None;
        }

        Some(autocomplete_attribute.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Use valid values for the "<Emphasis>"autocomplete"</Emphasis>" attribute."
                },
            )
            .note(markup! {
                "The autocomplete attribute only accepts a certain number of specific fixed values."
        }).note(markup!{
            "Follow the links for more information,
  "<Hyperlink href="https://www.w3.org/WAI/WCAG21/Understanding/identify-input-purpose">"WCAG 1.3.5"</Hyperlink>"
  "<Hyperlink href="https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#autofill">"HTML Living Standard autofill"</Hyperlink>"
  "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete">"HTML attribute: autocomplete - HTML: HyperText Markup Language | MDN"</Hyperlink>""
        })
    )
    }
}

/// Checks if the autocomplete attribute values are valid
fn is_valid_autocomplete(autocomplete_values: &[&str]) -> bool {
    match autocomplete_values.len() {
        0 => true,
        1 => {
            // SAFETY: the size of the slice is superior or equal to `1`
            let first = autocomplete_values[0];
            first.is_empty()
                || first.starts_with("section-")
                || VALID_AUTOCOMPLETE_VALUES.binary_search(&first).is_ok()
        }
        2.. => {
            // SAFETY: the size of the slice is superior or equal to `2`
            let first = autocomplete_values[0];
            let second = autocomplete_values[1];
            first.starts_with("section-")
                || ["billing", "shipping"].contains(&first)
                    && (BILLING_AND_SHIPPING_ADDRESS.contains(&second)
                        || VALID_AUTOCOMPLETE_VALUES.binary_search(&second).is_ok())
                || autocomplete_values
                    .iter()
                    .all(|val| VALID_AUTOCOMPLETE_VALUES.binary_search(val).is_ok())
        }
    }
}

#[test]
fn test_order() {
    assert!(VALID_AUTOCOMPLETE_VALUES.is_sorted());
    assert!(BILLING_AND_SHIPPING_ADDRESS.is_sorted());
}
