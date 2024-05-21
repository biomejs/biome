use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_aria::AriaRoles;
use biome_console::markup;
use biome_js_syntax::{AnyJsxAttribute, JsxOpeningElement};
use biome_rowan::AstNode;

use crate::services::aria::Aria;

declare_rule! {
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
    /// <div>...</div>
    /// <header>...</header>
    /// <img alt="" src="image.jpg" />
    /// ```
    ///
    pub UseSemanticElements {
        version: "next",
        name: "useSemanticElements",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("prefer-tag-over-role")],
        recommended: true,
    }
}

impl Rule for UseSemanticElements {
    type Query = Aria<JsxOpeningElement>;
    // TODO: get element and attributes and AnyJsxAttribute
    type State = (
        AnyJsxAttribute,
        Option<Vec<String>>,
        Option<Vec<(String, String)>>,
    );
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let role_attributes = node.find_attribute_by_name("role").unwrap();

        if let Some(attr) = role_attributes {
            let extract_attributes = ctx.extract_attributes(&node.attributes());

            let element = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
            let element_name = element.text_trimmed();
            let is_not_interative =
                AriaRoles.is_not_interactive_element(element_name, extract_attributes);
            if is_not_interative {
                let static_value = attr.as_static_value().unwrap();
                let role_value = static_value.as_string_constant().unwrap();
                let candidates = AriaRoles.get_corresponding_element(role_value);

                if let Some(elements) = candidates {
                    let mut result_elements: Vec<String> = vec![];
                    let mut result_attributes: Vec<(String, String)> = vec![];

                    for element in elements {
                        // Get only first attribute
                        let attributes = element.1.first();

                        result_elements.push(element.0.to_string());

                        if let Some(attribute) = attributes {
                            result_attributes
                                .push((attribute.0.to_string(), attribute.1.to_string()));
                        } else {
                            result_attributes.push((String::new(), String::new()));
                        }
                    }

                    return Some((
                        AnyJsxAttribute::from(attr),
                        Some(result_elements),
                        Some(result_attributes),
                    ));
                } else {
                    return Some((AnyJsxAttribute::from(attr), None, None));
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let alternative_elements = state.clone().1;
        let alternative_attributes = state.clone().2;

        let mut error_message = if alternative_elements.is_some() {
            String::from(
                "The elements with the following roles can be changed to the following elements:\n",
            )
        } else {
            String::from("The element with this role can be changed to a DOM element that already this role.")
        };

        for (element, attribute) in alternative_elements
            .iter()
            .zip(alternative_attributes.iter())
        {
            if !attribute.first().unwrap().0.is_empty() && !attribute.first().unwrap().1.is_empty()
            {
                error_message.push_str(&format!(
                    "<{element} {key}=\"{value}\">\n",
                    element = element.first().unwrap(),
                    key = attribute.first().unwrap().0,
                    value = attribute.first().unwrap().1
                ));
            } else {
                error_message.push_str(&format!(
                    "<{element}>\n",
                    element = element.first().unwrap()
                ));
            }
        }

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.0.range(),
                error_message,
            )
            .footer_list(
                markup! {
                    "For examples and more information, see" <Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles">"WAI-ARIA Roles"</Hyperlink>
                },
                &["<button>", "<input>", "<textarea>", "<a>", "<img>", "<table>", "<article>", "<section>", "<nav>", "<aside>", "<header>", "<footer>", "<main>", "<figure>", "<figcaption>", "<details>", "<summary>", "<dialog>", "<menu>", "<menuitem>", "<fieldset>", "<legend>", "<caption>", "<colgroup>", "<col>", "<optgroup>", "<option>", "<select>", "<datalist>", "<output>", "<progress>", "<meter>", "<time>", "<audio>", "<video>", "<track>", "<source>", "<embed>", "<object>", "<param>", "<iframe>", "<canvas>", "<map>", "<area>", "<svg>", "<math>"]
            ),
        )
    }
}
