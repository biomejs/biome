use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_aria::AriaRoles;
use biome_console::markup;
use biome_js_syntax::{JsxAttribute, JsxOpeningElement};
use biome_rowan::TextRange;

use crate::services::aria::Aria;

declare_lint_rule! {
    /// It detects the use of `role` attributes in JSX elements and suggests using semantic elements instead.
    ///
    /// The `role` attribute is used to define the purpose of an element, but it should be used as a last resort. Using semantic elements like `<button>`, `<input>`, `<textarea>`, `<a>`, `<img>`, `<table>`, `<article>`, `<section>`, `<nav>`, `<aside>`, `<header>`, `<footer>`, `<main>`, `<figure>`, `<figcaption>`, `<details>`, `<summary>`, `<dialog>`, `<menu>`, `<menuitem>`, `<fieldset>`, `<legend>`, `<caption>`, `<colgroup>`, `<col>`, `<optgroup>`, `<option>`, `<select>`, `<datalist>`, `<output>`, `<progress>`, `<meter>`, `<time>`, `<audio>`, `<video>`, `<track>`, `<source>`, `<embed>`, `<object>`, `<param>`, `<iframe>`, `<canvas>`, `<map>`, `<area>`, `<svg>`, `<math>` are more accessible and provide better semantics.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="checkbox">
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="img">
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///  <div></div>
    ///  <header></header>
    ///  <img alt="" src="image.jpg" />
    /// </>
    /// ```
    ///
    pub UseSemanticElements {
        version: "1.8.0",
        name: "useSemanticElements",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("prefer-tag-over-role")],
        recommended: true,
    }
}

impl Rule for UseSemanticElements {
    type Query = Aria<JsxOpeningElement>;
    type State = JsxAttribute;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let role_attribute = node.find_attribute_by_name("role").unwrap();

        if let Some(attr) = role_attribute {
            // check is not interactive element
            let element = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
            let element_name = element.text_trimmed();

            let aria_roles = ctx.aria_roles();
            let extract_attributes = ctx.extract_attributes(&node.attributes());
            let extract_attributes = ctx.convert_all_attribute_values(extract_attributes);
            let is_not_interative =
                aria_roles.is_not_interactive_element(element_name, extract_attributes);

            if is_not_interative {
                return Some(attr);
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let role_attribute = state;

        let static_value = role_attribute.as_static_value()?;
        let role_value = static_value.as_string_constant()?;
        let candidate = AriaRoles.get_elements_by_role(role_value);

        let mut result_elements: Vec<&str> = vec![];
        let mut result_attributes: Vec<(&str, &str)> = vec![];
        if let Some(elements) = candidate {
            for element in elements {
                result_elements.push(element.0);

                // Get only first attribute
                let attributes = element.1.first();
                match attributes {
                    Some(attribute) => result_attributes.push((attribute.0, attribute.1)),
                    None => result_attributes.push(("", "")),
                }
            }
        }

        let mut error_message = if !result_elements.is_empty() {
            String::from(
                "The elements with the following roles can be changed to the following elements:\n",
            )
        } else {
            String::from("The element with this role can be changed to a DOM element that already this role.")
        };

        for (element, attribute) in result_elements.iter().zip(result_attributes.iter()) {
            if !attribute.0.is_empty() && !attribute.1.is_empty() {
                error_message.push_str(&format!(
                    "<{element} {key}=\"{value}\">\n",
                    element = element,
                    key = attribute.0,
                    value = attribute.1
                ));
            } else {
                error_message.push_str(&format!("<{element}>\n"));
            }
        }

        let span = TextRange::new(
            role_attribute.name_value_token()?.text_range().start(),
            role_attribute.as_static_value()?.range().end(),
        );

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                error_message,
            )
            .note(markup! {
                "For examples and more information, see " <Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles">"WAI-ARIA Roles"</Hyperlink>
            }),
        )
    }
}
