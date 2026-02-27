use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlAttribute, HtmlElementList};
use biome_rowan::AstNode;
use biome_rule_options::no_svg_without_title::NoSvgWithoutTitleOptions;
use biome_string_case::StrLikeExtension;

use crate::a11y::is_aria_hidden_true;

const NAME_REQUIRED_ROLES: &[&str] = &["img", "image", "graphics-document", "graphics-symbol"];

declare_lint_rule! {
    /// Enforces the usage of the `title` element for the `svg` element.
    ///
    /// It is not possible to specify the `alt` attribute for the `svg` as for the `img`.
    /// To make svg accessible, the following methods are available:
    /// - provide the `title` element as the first child to `svg`
    /// - provide `role="img"` and `aria-label` or `aria-labelledby` to `svg`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <svg>foo</svg>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <svg>
    ///     <title></title>
    ///     <circle />
    /// </svg>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <svg role="img" title="title">
    ///     <span id="">foo</span>
    /// </svg>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <svg role="img" aria-labelledby="title">
    ///     <span id="title2">foo</span>
    /// </svg>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <svg>
    ///     <rect />
    ///     <rect />
    ///     <g>
    ///         <title>foo</title>
    ///         <circle />
    ///         <circle />
    ///     </g>
    /// </svg>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <svg role="graphics-symbol"><rect /></svg>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <svg>
    ///     <title>Pass</title>
    ///     <circle />
    /// </svg>
    /// ```
    ///
    /// ```html
    /// <svg role="img" aria-labelledby="title">
    ///     <span id="title">Pass</span>
    /// </svg>
    /// ```
    ///
    /// ```html
    /// <svg role="img" aria-label="title">
    ///     <span id="title">Pass</span>
    /// </svg>
    /// ```
    ///
    /// ```html
    /// <svg role="img" aria-label="">
    ///     <span id="">Pass</span>
    /// </svg>
    /// ```
    ///
    /// ```html
    /// <svg role="graphics-symbol">
    ///     <title>Pass</title>
    ///     <rect />
    /// </svg>
    /// ```
    ///
    /// ```html
    /// <svg aria-hidden="true"><rect /></svg>
    /// ```
    ///
    /// ```html
    /// <svg role="presentation">foo</svg>
    /// ```
    ///
    ///
    /// ## Accessibility guidelines
    /// [Document Structure – SVG 1.1 (Second Edition)](https://www.w3.org/TR/SVG11/struct.html#DescriptionAndTitleElements)
    /// [ARIA: img role - Accessibility | MDN](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/img_role)
    /// [Accessible SVGs | CSS-Tricks - CSS-Tricks](https://css-tricks.com/accessible-svgs/)
    /// [Contextually Marking up accessible images and SVGs | scottohara.me](https://www.scottohara.me/blog/2019/05/22/contextual-images-svgs-and-a11y.html)
    /// [Accessible SVGs](https://www.unimelb.edu.au/accessibility/techniques/accessible-svgs)
    ///
    pub NoSvgWithoutTitle {
        version: "2.4.0",
        name: "noSvgWithoutTitle",
        language: "html",
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoSvgWithoutTitle {
    type Query = Ast<AnyHtmlElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoSvgWithoutTitleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.name()? != "svg" {
            return None;
        }

        if is_aria_hidden_true(node) {
            return None;
        }

        // Checks if a `svg` element has a valid `title` element in a childlist
        let html_element = node.as_html_element()?;
        if html_element.opening_element().is_ok() {
            let has_valid_title = has_valid_title_element(&html_element.children());
            if has_valid_title.is_some_and(|bool| bool) {
                return None;
            }
        }

        // TODO: use `aria_roles.has_name_required_image_role` of aria crate
        // Checks if a `svg` element has role='img' and title/aria-label/aria-labelledby attribute
        let Some(role_attribute) = node.find_attribute_by_name("role") else {
            return Some(());
        };

        let role_attribute_value = role_attribute.initializer()?.value().ok()?;
        let Some(role_attribute_text) = role_attribute_value
            .as_html_string()?
            .inner_string_text()
            .ok()
        else {
            return Some(());
        };

        let role_text = role_attribute_text.to_ascii_lowercase_cow();
        if role_text.trim().is_empty() {
            return Some(());
        }

        // Check if any of the space-separated roles are valid
        let has_name_required_role = role_text
            .split_whitespace()
            .any(|role| NAME_REQUIRED_ROLES.contains(&role));

        if has_name_required_role {
            let aria_label = node.find_attribute_by_name("aria-label");
            let aria_labelledby = node.find_attribute_by_name("aria-labelledby");
            let is_valid_a11y_attribute = aria_label.is_some()
                || is_valid_attribute_value(aria_labelledby, &html_element.children())
                    .unwrap_or(false);
            if is_valid_a11y_attribute {
                return None;
            }
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let diagnostic = RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Alternative text "<Emphasis>"title"</Emphasis>" element cannot be empty"
                },
            )
            .note(markup! {
                "For accessibility purposes, "<Emphasis>"SVGs"</Emphasis>" should have an alternative text, provided via "<Emphasis>"title"</Emphasis>" element. If the svg element has role=\"img\", you should add the "<Emphasis>"aria-label"</Emphasis>" or "<Emphasis>"aria-labelledby"</Emphasis>" attribute."
            });
        Some(diagnostic)
    }
}

// Checks if the first element of the given `HtmlElementList` is a valid `title` element.
fn has_valid_title_element(html_child_list: &HtmlElementList) -> Option<bool> {
    let first_child = html_child_list.into_iter().next()?;
    let html_element = first_child.as_html_element()?;
    let opening_element = html_element.opening_element().ok()?;
    let name = opening_element.name().ok()?;
    let name_text = name.token_text_trimmed()?;
    let has_title_name = name_text == "title";
    if !has_title_name {
        return Some(false);
    }
    let has_child = html_element.children().into_iter().count() > 0;
    Some(has_child)
}

/// Checks if the given attribute is attached to the `svg` element and the attribute value is used by the `id` of the child element.
fn is_valid_attribute_value(
    attribute: Option<HtmlAttribute>,
    html_child_list: &HtmlElementList,
) -> Option<bool> {
    let attribute_value = attribute?.initializer()?.value().ok()?;
    let is_used_attribute = html_child_list
        .into_iter()
        .filter_map(|child| {
            let html_element = child.as_html_element()?;
            let maybe_attribute = html_element.find_attribute_by_name("id");
            let child_attribute_value = maybe_attribute?.initializer()?.value().ok()?;
            let is_valid = attribute_value.string_value() == child_attribute_value.string_value();
            Some(is_valid)
        })
        .any(|x| x);
    Some(is_used_attribute)
}
