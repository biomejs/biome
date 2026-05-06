use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlAttributeInitializer, HtmlAttribute, inner_string_text};
use biome_rowan::{TextRange, TextSize};
use biome_rule_options::no_tailwind_arbitrary_value::NoTailwindArbitraryValueOptions;
use biome_tailwind_parser::parse_tailwind;
use biome_tailwind_syntax::lint_utils::arbitrary_ranges;

declare_lint_rule! {
    /// Disallow arbitrary values in Tailwind CSS utility classes.
    ///
    /// Arbitrary values (e.g. `w-[400px]`, `text-[#555]`, `[color:red]`) bypass
    /// Tailwind's configured theme scales. This rule reports them so teams can
    /// keep styling constrained to named utilities from their Tailwind configuration.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div class="w-[400px]"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div class="text-[#555] bg-white"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div class="[color:red]"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="w-4 text-red-500 bg-white"></div>
    /// ```
    ///
    /// ```html
    /// <div class="[&:nth-child(3)]:px-2"></div>
    /// ```
    ///
    /// ## Options
    ///
    /// By default, this rule checks the `class` attribute. The `attributes`
    /// option adds more HTML attributes to check.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "attributes": ["classList"]
    ///     }
    /// }
    /// ```
    ///
    /// ```html,use_options,expect_diagnostic
    /// <div classList="w-[400px]"></div>
    /// ```
    ///
    /// ### attributes
    ///
    /// Additional HTML attribute names to check.
    ///
    /// Default: `[]` (the `class` attribute is always checked).
    ///
    /// ### functions
    ///
    /// This option has no effect for HTML. HTML attribute values are plain strings
    /// and cannot contain function calls, so configuring `functions` is a no-op for
    /// this rule. Use the `attributes` option instead to extend which attributes are
    /// checked.
    ///
    pub NoTailwindArbitraryValue {
        version: "next",
        name: "noTailwindArbitraryValue",
        language: "html",
        sources: &[RuleSource::EslintTailwindcss("no-arbitrary-value").same()],
        domains: &[RuleDomain::Tailwind],
        recommended: false,
    }
}

impl Rule for NoTailwindArbitraryValue {
    type Query = Ast<HtmlAttribute>;
    type State = TextRange;
    type Signals = Vec<TextRange>;
    type Options = NoTailwindArbitraryValueOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx.query();
        let options = ctx.options();

        let Some(name) = attribute
            .name()
            .ok()
            .and_then(|name| name.value_token().ok())
            .map(|token| token.token_text_trimmed())
        else {
            return vec![];
        };

        if !is_html_class_attribute(name.text(), options) {
            return vec![];
        }

        let Some(initializer) = attribute.initializer() else {
            return vec![];
        };
        let Ok(AnyHtmlAttributeInitializer::HtmlString(html_string)) = initializer.value() else {
            return vec![];
        };
        let Ok(token) = html_string.value_token() else {
            return vec![];
        };

        let text = inner_string_text(&token);
        if !text.text().contains('[') {
            return vec![];
        }

        // HTML attribute values are always quoted per spec
        let content_start = token.text_trimmed_range().start() + TextSize::from(1);

        let parse = parse_tailwind(text.text());
        arbitrary_ranges(&parse.tree().candidates(), content_start)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! { "Found an arbitrary value in a Tailwind CSS class." },
            )
            .note(markup! {
                "Arbitrary values bypass Tailwind's theme configuration, defeating design-system consistency and making styles harder to refactor."
            })
            .note(markup! {
                "Use a named utility from your Tailwind configuration instead."
            }),
        )
    }
}

fn is_html_class_attribute(name: &str, options: &NoTailwindArbitraryValueOptions) -> bool {
    name.eq_ignore_ascii_case("class")
        || options
            .attributes
            .iter()
            .flatten()
            .any(|attribute| attribute.as_ref().eq_ignore_ascii_case(name))
}

