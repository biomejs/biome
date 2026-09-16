use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_non_scalable_viewport::NoNonScalableViewportOptions;

declare_lint_rule! {
    /// Disallow disabling zoom with `user-scalable=no` in the `<meta name="viewport">` element.
    ///
    /// Disabling zoom can make page content difficult to read for people with low vision.
    ///
    /// See [WCAG 1.4.4](https://www.w3.org/WAI/WCAG21/Understanding/resize-text.html) and the
    /// [html-eslint rule](https://html-eslint.org/docs/rules/no-non-scalable-viewport) for details.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <meta name="viewport" content="width=device-width, user-scalable=no" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <meta name={"viewport"} content={"user-scalable=no"} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///   <meta name="viewport" content="width=device-width, user-scalable=yes" />
    ///   <meta name="viewport" content="width=device-width" />
    ///   <Meta name="viewport" content="user-scalable=no" />
    /// </>
    /// ```
    ///
    pub NoNonScalableViewport {
        version: "2.5.7",
        name: "noNonScalableViewport",
        language: "jsx",
        recommended: false,
        severity: Severity::Error,
        sources: &[RuleSource::HtmlEslint("no-non-scalable-viewport").inspired()],
    }
}

impl Rule for NoNonScalableViewport {
    type Query = Ast<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoNonScalableViewportOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let name = element.name_value_token().ok()?;

        if name.text_trimmed() != "meta" {
            return None;
        }

        let name_attribute = element.find_attribute_by_name("name")?;
        let content_attribute = element.find_attribute_by_name("content")?;
        let name_value = name_attribute.as_static_value()?;
        let content_value = content_attribute.as_static_value()?;

        if !name_value.text().eq_ignore_ascii_case("viewport") {
            return None;
        }

        has_non_scalable_viewport(content_value.text()).then_some(content_attribute.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "The viewport disables user scaling."
                },
            )
            .note(markup! {
                "Disabling zoom can make page content difficult to read for people with low vision."
            })
            .note(markup! {
                "Remove the "<Emphasis>"user-scalable=no"</Emphasis>" directive from the "<Emphasis>"content"</Emphasis>" attribute."
            }),
        )
    }
}

fn has_non_scalable_viewport(content: &str) -> bool {
    content.split(',').any(|directive| {
        let Some((property, value)) = directive.split_once('=') else {
            return false;
        };

        property.trim().eq_ignore_ascii_case("user-scalable")
            && value.trim().eq_ignore_ascii_case("no")
    })
}
