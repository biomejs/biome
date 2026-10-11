use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::VueDirective;
use biome_rowan::AstNode;
use biome_rule_options::no_vue_v_text::NoVueVTextOptions;

declare_lint_rule! {
    /// Disallow the use of Vue's `v-text` directive.
    ///
    /// The `v-text` directive fills an element with text from a value, and replaces anything written between the element's tags.
    /// Because the text is set from an attribute, someone reading the template cannot see where it ends up.
    ///
    /// Put the value between double curly braces (`{{ }}`) instead.
    /// Vue calls this text interpolation. It shows the text exactly where it appears in the template,
    /// and it can be mixed with other text in the same element.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div v-text="message"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div>{{ message }}</div>
    /// <p>Hello, {{ name }}!</p>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue `v-text` directive](https://vuejs.org/api/built-in-directives.html#v-text)
    /// - [Vue text interpolation](https://vuejs.org/guide/essentials/template-syntax.html#text-interpolation)
    pub NoVueVText {
        version: "next",
        name: "noVueVText",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-v-text").same()],
    }
}

impl Rule for NoVueVText {
    type Query = Ast<VueDirective>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueVTextOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let name = ctx.query().name_token().ok()?;
        (name.text_trimmed() == "v-text").then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Unexpected use of the "<Emphasis>"v-text"</Emphasis>" directive."
                },
            )
            .note(markup! {
                <Emphasis>"v-text"</Emphasis>" replaces everything inside the element, so the text it shows is not visible where it appears in the template."
            })
            .note(markup! {
                "Put the value between double curly braces inside the element instead, like "<Emphasis>"{{ message }}"</Emphasis>"."
            }),
        )
    }
}
