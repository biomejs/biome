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
    /// ```js,expect_diagnostic
    /// <div role="checkbox">
    /// <div role="img">
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
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
    type State = AnyJsxAttribute;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let attributes = node.attributes();
        for attr in attributes {
            let attr_value = attr.as_jsx_attribute().unwrap().name_value_token().unwrap();
            let attr_name = attr_value.text_trimmed();
            if attr_name == "role" {
                let aria_roles = AriaRoles::default();
                let extract_attributes = ctx.extract_attributes(&node.attributes());

                let element: biome_rowan::SyntaxToken<biome_js_syntax::JsLanguage> =
                    node.name().ok()?.as_jsx_name()?.value_token().ok()?;
                let element_name = element.text_trimmed();
                let is_not_interative =
                    aria_roles.is_not_interactive_element(element_name, extract_attributes);
                if is_not_interative {
                    return Some(attr);
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "This JSX element uses a `role` attribute. Use a semantic element instead."
                },
            )
            .note(markup! {
                "Semantic elements like `<button>`, `<input>`, `<textarea>`, `<a>`, `<img>`, `<table>`, `<article>`, `<section>`, `<nav>`, `<aside>`, `<header>`, `<footer>`, `<main>`, `<figure>`, `<figcaption>`, `<details>`, `<summary>`, `<dialog>`, `<menu>`, `<menuitem>`, `<fieldset>`, `<legend>`, `<caption>`, `<colgroup>`, `<col>`, `<optgroup>`, `<option>`, `<select>`, `<datalist>`, `<output>`, `<progress>`, `<meter>`, `<time>`, `<audio>`, `<video>`, `<track>`, `<source>`, `<embed>`, `<object>`, `<param>`, `<iframe>`, `<canvas>`, `<map>`, `<area>`, `<svg>`, `<math>` are more accessible and provide better semantics."
            }),
        )
    }
}
