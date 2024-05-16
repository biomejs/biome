use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{AnyJsxAttribute, JsxOpeningElement};
use biome_rowan::AstNode;

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
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
    type Query = Ast<JsxOpeningElement>;
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
                return Some(attr);
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
