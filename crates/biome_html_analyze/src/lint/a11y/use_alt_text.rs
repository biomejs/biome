use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::{fmt::Display, fmt::Formatter, markup};
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_alt_text::UseAltTextOptions;

use crate::a11y::{
    attribute_value_equals_ignore_case, has_non_empty_attribute, is_aria_hidden_true,
};

declare_lint_rule! {
    /// Enforce that all elements that require alternative text have meaningful information to relay back to the end user.
    ///
    /// This is a critical component of accessibility for screen reader users in order for them
    /// to understand the content's purpose on the page.
    /// By default, this rule checks for alternative text on the following elements:
    /// `<img>`, `<area>`, `<input type="image">`, and `<object>`.
    ///
    /// :::note
    /// In `.html` files, this rule matches element names case-insensitively (e.g., `<IMG>`, `<Img>`).
    ///
    /// In component-based frameworks (Vue, Svelte, Astro), only lowercase element names are checked.
    /// PascalCase variants like `<Img>` are assumed to be custom components and are ignored.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <img src="image.png" />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <input type="image" src="image.png" />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <area href="foo" />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <object data="foo"></object>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <img src="image.png" alt="A beautiful landscape" />
    /// ```
    ///
    /// ```html
    /// <input type="image" src="image.png" alt="Submit" />
    /// ```
    ///
    /// ```html
    /// <img src="image.png" aria-label="A beautiful landscape" />
    /// ```
    ///
    /// ```html
    /// <img src="image.png" aria-labelledby="image-description" />
    /// ```
    ///
    /// ```html
    /// <object data="foo" title="Embedded content"></object>
    /// ```
    ///
    /// ```html
    /// <!-- Decorative images can be hidden from assistive technologies -->
    /// <img src="decorative.png" alt="" />
    /// ```
    ///
    /// ```html
    /// <img src="decorative.png" aria-hidden="true" />
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 1.1.1](https://www.w3.org/WAI/WCAG21/Understanding/non-text-content.html)
    ///
    pub UseAltText {
        version: "2.4.0",
        name: "useAltText",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("alt-text").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

/// The type of element being validated
pub enum ValidatedElement {
    Object,
    Img,
    Area,
    Input,
}

impl Display for ValidatedElement {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self {
            Self::Object => fmt.write_markup(markup!(<Emphasis>"title"</Emphasis>)),
            _ => fmt.write_markup(markup!(<Emphasis>"alt"</Emphasis>)),
        }
    }
}

impl Rule for UseAltText {
    type Query = Ast<AnyHtmlElement>;
    type State = (ValidatedElement, TextRange);
    type Signals = Option<Self::State>;
    type Options = UseAltTextOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let file_source = ctx.source_type::<HtmlFileSource>();

        let element_name = element.name()?;
        let is_html_file = file_source.is_html();

        let has_alt = has_valid_alt_text(element);
        let has_aria_label = has_non_empty_attribute(element, "aria-label");
        let has_aria_labelledby = has_non_empty_attribute(element, "aria-labelledby");
        let aria_hidden = is_aria_hidden_true(element);

        let name_matches = |name: &str| -> bool {
            if is_html_file {
                element_name.eq_ignore_ascii_case(name)
            } else {
                element_name.text() == name
            }
        };

        if name_matches("object") {
            let has_title = has_non_empty_attribute(element, "title");

            if !has_title && !has_aria_label && !has_aria_labelledby && !aria_hidden {
                // For object elements, check if it has accessible child content
                // In HTML, we can't easily check for accessible children, so we flag all
                // object elements without title/aria-label/aria-labelledby
                return Some((
                    ValidatedElement::Object,
                    element.syntax().text_trimmed_range(),
                ));
            }
        } else if name_matches("img") {
            if !has_alt && !has_aria_label && !has_aria_labelledby && !aria_hidden {
                return Some((ValidatedElement::Img, element.syntax().text_trimmed_range()));
            }
        } else if name_matches("area") {
            if !has_alt && !has_aria_label && !has_aria_labelledby && !aria_hidden {
                return Some((
                    ValidatedElement::Area,
                    element.syntax().text_trimmed_range(),
                ));
            }
        } else if name_matches("input")
            && has_type_image_attribute(element)
            && !has_alt
            && !has_aria_label
            && !has_aria_labelledby
            && !aria_hidden
        {
            return Some((
                ValidatedElement::Input,
                element.syntax().text_trimmed_range(),
            ));
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (validated_element, range) = state;
        let message = markup!(
            "Provide a text alternative through the "{validated_element}", "<Emphasis>"aria-label"</Emphasis>", or "<Emphasis>"aria-labelledby"</Emphasis>" attribute."
        )
        .to_owned();
        Some(
            RuleDiagnostic::new(rule_category!(), range, message)
                .note(markup! {
                    "Meaningful alternative text on elements helps users relying on screen readers to understand content's purpose within a page."
                })
                .note(markup! {
                    "If the content is decorative, redundant, or obscured, consider hiding it from assistive technologies with the "<Emphasis>"aria-hidden"</Emphasis>" attribute."
                }),
        )
    }
}

/// Check if the element has a type="image" attribute
fn has_type_image_attribute(element: &AnyHtmlElement) -> bool {
    element
        .find_attribute_by_name("type")
        .is_some_and(|attr| attribute_value_equals_ignore_case(&attr, "image"))
}

/// Check if the element has a valid alt attribute
fn has_valid_alt_text(element: &AnyHtmlElement) -> bool {
    // The alt attribute exists - even an empty alt="" is valid for decorative images
    // If there's no initializer, it's treated as an empty string (valid)
    // If there's an initializer with a value, any value is valid
    element.find_attribute_by_name("alt").is_some()
}
