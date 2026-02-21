use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{AnyHtmlAttributeInitializer, HtmlFileSource};
use biome_rowan::AstNode;
use biome_rule_options::is_redundant_alt;
use biome_rule_options::no_redundant_alt::NoRedundantAltOptions;

declare_lint_rule! {
    /// Enforce `img` alt prop does not contain the word "image", "picture", or "photo".
    ///
    /// The rule will first check if `aria-hidden` is truthy to determine whether to enforce the rule. If the image is
    /// hidden, then the rule will always succeed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <img src="src" alt="photo content" />;
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <img alt="picture of cool person" aria-hidden="false" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div>
    /// 	<img src="src" alt="alt" />
    /// 	<img src="bar" aria-hidden alt="Picture of me taking a photo of an image" />
    /// </div>
    /// ```
    ///
    pub NoRedundantAlt {
        version: "2.4.0",
        name: "noRedundantAlt",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("img-redundant-alt").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoRedundantAlt {
    type Query = Ast<AnyHtmlTagElement>;
    type State = AnyHtmlAttributeInitializer;
    type Signals = Option<Self::State>;
    type Options = NoRedundantAltOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let file_source = ctx.source_type::<HtmlFileSource>();

        let name = node.name().ok()?.token_text_trimmed()?;
        if (file_source.is_html() && !name.eq_ignore_ascii_case("img"))
            || (!file_source.is_html() && name != "img")
        {
            return None;
        }

        // If aria-hidden is truthy (present and not "false"), skip the check
        if node.has_truthy_attribute("aria-hidden") {
            return None;
        }

        let alt = node
            .find_attribute_by_name("alt")?
            .initializer()?
            .value()
            .ok()?;

        match alt {
            AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(ref expression) => {
                let value = expression.expression().ok()?.html_literal_token().ok()?;

                is_redundant_alt(value.text_trimmed()).then_some(alt)
            }
            AnyHtmlAttributeInitializer::HtmlString(ref value) => {
                let inner_string_text = value.inner_string_text().ok()?;
                is_redundant_alt(inner_string_text.text()).then_some(alt)
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "Avoid the words \"image\", \"picture\", or \"photo\" in " <Emphasis>"img"</Emphasis>" element alt text."
                },
            )
            .note(markup! {
                "Screen readers announce img elements as \"images\", so it is not necessary to redeclare this in alternative text."
            }),
        )
    }
}
