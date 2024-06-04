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
    type State = AnyJsxAttribute;
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
                return Some(AnyJsxAttribute::from(attr));
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "The element with this role can be changed to a DOM element that already this role."
                },
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
