use crate::services::aria::Aria;
use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsxAttribute, JsxChildList, JsxElement, jsx_ext::AnyJsxElement};
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_svg_without_title::NoSvgWithoutTitleOptions;

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
    /// ```jsx,expect_diagnostic
    /// <svg>foo</svg>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <svg>
    ///     <title></title>
    ///     <circle />
    /// </svg>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <svg>foo</svg>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
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
    /// ```jsx,expect_diagnostic
    /// <svg role="graphics-symbol"><rect /></svg>
    /// ```
    ///
    /// ### Valid
    ///
    ///
    /// ```jsx
    /// <svg>
    ///     <title>Pass</title>
    ///     <circle />
    /// </svg>
    /// ```
    ///
    /// ```jsx
    /// <svg role="img" aria-labelledby="title">
    ///     <span id="title">Pass</span>
    /// </svg>
    /// ```
    ///
    /// ```jsx
    /// <svg role="img" aria-label="title">
    ///     <span id="title">Pass</span>
    /// </svg>
    /// ```
    ///
    /// ```jsx
    /// <svg role="graphics-symbol">
    ///     <title>Pass</title>
    ///     <rect />
    /// </svg>
    /// ```
    ///
    /// ```jsx
    /// <svg aria-hidden="true"><rect /></svg>
    /// ```
    ///
    /// ```jsx
    /// <svg role="img" aria-label="">
    ///     <span id="">Pass</span>
    /// </svg>
    /// ```
    ///
    /// ```jsx
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
        version: "1.0.0",
        name: "noSvgWithoutTitle",
        language: "jsx",
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoSvgWithoutTitle {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoSvgWithoutTitleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_roles = ctx.aria_roles();

        if node.name_value_token().ok()?.text_trimmed() != "svg" {
            return None;
        }

        if let Some(aria_hidden_attr) = node.find_attribute_by_name("aria-hidden")
            && let Some(attr_static_val) = aria_hidden_attr.as_static_value()
        {
            let attr_text = attr_static_val.text();
            if attr_text == "true" {
                return None;
            }
        }

        // Checks if a `svg` element has a valid `title` element is in a childlist
        let jsx_element = node.parent::<JsxElement>()?;
        if let AnyJsxElement::JsxOpeningElement(_) = node {
            let has_valid_title = has_valid_title_element(&jsx_element.children());
            if has_valid_title.is_some_and(|bool| bool) {
                return None;
            }
        }

        let has_name_required_role = aria_roles.has_name_required_image_role(node);

        if has_name_required_role {
            let [aria_label, aria_labelledby] = node
                .attributes()
                .find_by_names(["aria-label", "aria-labelledby"]);
            let is_valid_a11y_attribute = aria_label.is_some()
                || is_valid_attribute_value(aria_labelledby, &jsx_element.children())
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

/// Checks if the given attribute is attached to the `svg` element and the attribute value is used by the `id` of the child element.
fn is_valid_attribute_value(
    attribute: Option<JsxAttribute>,
    jsx_child_list: &JsxChildList,
) -> Option<bool> {
    let attribute_value = attribute?.initializer()?.value().ok()?;
    let is_used_attribute = jsx_child_list
        .iter()
        .filter_map(|child| {
            let jsx_element = child.as_jsx_element()?;
            let opening_element = jsx_element.opening_element().ok()?;
            let maybe_attribute = opening_element.find_attribute_by_name("id");
            let child_attribute_value = maybe_attribute?.initializer()?.value().ok()?;
            let is_valid = attribute_value.as_static_value()?.text()
                == child_attribute_value.as_static_value()?.text();
            Some(is_valid)
        })
        .any(|x| x);
    Some(is_used_attribute)
}

/// Checks if the first element of the given `JsxChildList` is a valid `title` element.
fn has_valid_title_element(jsx_child_list: &JsxChildList) -> Option<bool> {
    let first_child = jsx_child_list.iter().nth(1)?;
    let jsx_element = first_child.as_jsx_element()?;
    let opening_element = jsx_element.opening_element().ok()?;
    let name = opening_element.name().ok()?;
    let name = name.as_jsx_name()?.value_token().ok()?;
    let has_title_name = name.text_trimmed() == "title";
    if !has_title_name {
        return Some(false);
    }
    let is_empty_child = jsx_element.children().is_empty();
    Some(!is_empty_child)
}
