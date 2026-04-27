use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlAttributeInitializer, HtmlAttribute, inner_string_text};
use biome_rowan::{AstNode, AstNodeList, TextRange, TextSize, TokenText};
use biome_rule_options::no_tailwind_arbitrary_value::NoTailwindArbitraryValueOptions;
use biome_tailwind_parser::parse_tailwind;
use biome_tailwind_syntax::{AnyTwCandidate, AnyTwFullCandidate, AnyTwModifier, AnyTwValue};

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
    /// ### attributes
    ///
    /// Additional HTML attribute names to check.
    ///
    /// Default: `[]` (the `class` attribute is always checked).
    ///
    pub NoTailwindArbitraryValue {
        version: "next",
        name: "noTailwindArbitraryValue",
        language: "html",
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
        let quote_offset = if matches!(token.text_trimmed().as_bytes().first(), Some(b'"' | b'\'')) {
            TextSize::from(1)
        } else {
            TextSize::from(0)
        };
        let content_start = token.text_trimmed_range().start() + quote_offset;

        arbitrary_ranges(text, content_start)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! { "Found an arbitrary value in a Tailwind CSS class." },
            )
            .note(markup! {
                "Use a named utility from your Tailwind configuration instead."
            }),
        )
    }
}

fn is_html_class_attribute(name: &str, options: &NoTailwindArbitraryValueOptions) -> bool {
    name == "class"
        || options
            .attributes
            .iter()
            .flatten()
            .any(|attribute| attribute.as_ref() == name)
}

fn class_ranges(text: &str) -> Vec<(usize, &str)> {
    let mut class_start = None;
    let mut classes = Vec::new();

    for (index, ch) in text.char_indices() {
        if ch.is_ascii_whitespace() {
            if let Some(start) = class_start.take() {
                classes.push((start, &text[start..index]));
            }
        } else if class_start.is_none() {
            class_start = Some(index);
        }
    }

    if let Some(start) = class_start {
        classes.push((start, &text[start..]));
    }

    classes
}

fn text_size(offset: usize) -> TextSize {
    TextSize::from(u32::try_from(offset).expect("class offset should fit into u32"))
}

fn push_arbitrary_value_range(
    results: &mut Vec<TextRange>,
    class_start: TextSize,
    value: Option<AnyTwValue>,
) {
    if let Some(AnyTwValue::TwArbitraryValue(value)) = value {
        let range = value.syntax().text_trimmed_range();
        results.push(TextRange::new(
            class_start + range.start(),
            class_start + range.end(),
        ));
    }
}

fn push_modifier_range(
    results: &mut Vec<TextRange>,
    class_start: TextSize,
    modifier: Option<AnyTwModifier>,
) {
    if let Some(AnyTwModifier::TwModifier(modifier)) = modifier {
        push_arbitrary_value_range(results, class_start, modifier.value().ok());
    }
}

fn arbitrary_ranges(text: TokenText, content_start: TextSize) -> Vec<TextRange> {
    let mut results = Vec::new();

    for (class_offset, class_name) in class_ranges(text.text()) {
        let parse = parse_tailwind(class_name);
        let class_start = content_start + text_size(class_offset);

        for candidate in parse.tree().candidates().iter() {
            let AnyTwFullCandidate::TwFullCandidate(candidate) = candidate else {
                continue;
            };

            match candidate.candidate() {
                Ok(AnyTwCandidate::TwArbitraryCandidate(candidate)) => {
                    let range = candidate.syntax().text_trimmed_range();
                    results.push(TextRange::new(
                        class_start + range.start(),
                        class_start + range.end(),
                    ));
                    push_modifier_range(&mut results, class_start, candidate.modifier());
                }
                Ok(AnyTwCandidate::TwFunctionalCandidate(candidate)) => {
                    push_arbitrary_value_range(&mut results, class_start, candidate.value().ok());
                    push_modifier_range(&mut results, class_start, candidate.modifier());
                }
                _ => {}
            }
        }
    }

    results
}
