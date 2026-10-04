use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::VueDirective;
use biome_rowan::AstNode;
use biome_rule_options::no_vue_v_html::NoVueVHtmlOptions;

declare_lint_rule! {
    /// Disallow the use of Vue's `v-html` directive.
    ///
    /// `v-html` inserts its value into the page as real HTML. Any tags in the value become
    /// part of the page, including tags that run code in the browser. If the value contains
    /// text from users or another source you don't control, an attacker can use it to run
    /// their own code on your page. This attack is called cross-site scripting (XSS).
    ///
    /// To show a value as text, put it inside double curly braces (`{{ }}`) instead. Vue then
    /// displays any HTML tags in the value as plain text.
    ///
    /// If you need to render HTML, first remove any dangerous tags from it with a library
    /// such as [DOMPurify](https://github.com/cure53/DOMPurify). Then
    /// [suppress this rule](https://biomejs.dev/analyzer/suppressions/) with a comment that
    /// explains why the value is safe.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-html="content"></div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div>{{ content }}</div>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue `v-html` directive](https://vuejs.org/api/built-in-directives.html#v-html)
    /// - [Vue security guide: HTML injection](https://vuejs.org/guide/best-practices/security.html#html-injection)
    pub NoVueVHtml {
        version: "next",
        name: "noVueVHtml",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-v-html").same()],
    }
}

impl Rule for NoVueVHtml {
    type Query = Ast<VueDirective>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueVHtmlOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query().is_html().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The "<Emphasis>"v-html"</Emphasis>" directive inserts its value into the page as HTML."
                },
            )
            .note(markup! {
                "If the value contains text from users, an attacker can use it to run their own code on your page. This attack is called cross-site scripting (XSS)."
            })
            .note(markup! {
                "To show the value as text, use "<Emphasis>"{{ }}"</Emphasis>" instead. If you need to render HTML, remove any dangerous tags from it before using "<Emphasis>"v-html"</Emphasis>"."
            }),
        )
    }
}
