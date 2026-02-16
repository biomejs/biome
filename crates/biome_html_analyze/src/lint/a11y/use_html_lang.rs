use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::AnyHtmlElement;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_html_lang::UseHtmlLangOptions;

use crate::a11y::has_non_empty_attribute;

declare_lint_rule! {
    /// Enforce that `html` element has `lang` attribute.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <html></html>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <html lang=""></html>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <html lang="en"></html>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 3.1.1](https://www.w3.org/WAI/WCAG21/Understanding/language-of-page)
    ///
    pub UseHtmlLang {
        version: "2.4.0",
        name: "useHtmlLang",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("html-has-lang").same(), RuleSource::HtmlEslint("require-lang").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseHtmlLang {
    type Query = Ast<AnyHtmlElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseHtmlLangOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        if !is_html_element(element) {
            return None;
        }

        if has_non_empty_attribute(element, "lang") {
            return None;
        }

        Some(element.syntax().text_trimmed_range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state,
            markup! {
                "Provide a "<Emphasis>"lang"</Emphasis>" attribute when using the "<Emphasis>"html"</Emphasis>" element."
            }
        ).note(
            markup! {
                "Setting a "<Emphasis>"lang"</Emphasis>" attribute on HTML document elements configures the language "
                "used by screen readers when no user default is specified."
            }
        ))
    }
}

fn is_html_element(element: &AnyHtmlElement) -> bool {
    element
        .name()
        .is_some_and(|token_text| token_text.eq_ignore_ascii_case("html"))
}
