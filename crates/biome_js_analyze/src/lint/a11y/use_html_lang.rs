use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{TextRange, jsx_ext::AnyJsxElement};
use biome_rowan::AstNode;
use biome_rule_options::use_html_lang::UseHtmlLangOptions;

declare_lint_rule! {
    /// Enforce that `html` element has `lang` attribute.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <html></html>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <html lang={""}></html>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <html lang={null}></html>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <html lang={undefined}></html>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <html lang={true}></html>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <html lang="en"></html>
    /// ```
    ///
    /// ```jsx
    /// <html lang={language}></html>
    /// ```
    ///
    /// ```jsx
    /// <html {...props}></html>
    /// ```
    ///
    /// ```jsx
    /// <html lang={""} {...props}></html>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 3.1.1](https://www.w3.org/WAI/WCAG21/Understanding/language-of-page)
    ///
    pub UseHtmlLang {
        version: "1.0.0",
        name: "useHtmlLang",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("html-has-lang").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseHtmlLang {
    type Query = Ast<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseHtmlLangOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let name = element.name().ok()?.name_value_token().ok()?;

        if name.text_trimmed() == "html" {
            if let Some(lang_attribute) = element.find_attribute_by_name("lang") {
                if !lang_attribute
                    .as_static_value()
                    .is_none_or(|attribute| attribute.is_not_string_constant(""))
                    && !element.has_trailing_spread_prop(&lang_attribute)
                {
                    return Some(element.syntax().text_trimmed_range());
                }
            } else if !element.has_spread_prop() {
                return Some(element.syntax().text_trimmed_range());
            }
        }

        None
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
                "Setting a "<Emphasis>"lang"</Emphasis>" attribute on HTML document elements configures the language"
                "used by screen readers when no user default is specified."
            }
        ))
    }
}
