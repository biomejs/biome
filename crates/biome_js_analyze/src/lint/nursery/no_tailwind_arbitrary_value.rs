use crate::shared::any_class_string_like::AnyClassStringLike;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsSyntaxKind;
use biome_rowan::{TextRange, TextSize, TokenText};
use biome_rule_options::no_tailwind_arbitrary_value::NoTailwindArbitraryValueOptions;
use biome_tailwind_parser::parse_tailwind;
use biome_tailwind_syntax::lint_utils::arbitrary_ranges;

declare_lint_rule! {
    /// Disallow arbitrary values in Tailwind CSS utility classes.
    ///
    /// Arbitrary values (e.g. `w-[400px]`, `text-[#555]`) and arbitrary properties
    /// (e.g. `[color:red]`) bypass Tailwind's configured theme scales. This rule reports
    /// them so teams can keep styling constrained to named utilities from their Tailwind
    /// configuration.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="w-[400px]" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="text-[#555] bg-white" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="[color:red]" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div className="w-4 text-red-500 bg-white" />;
    /// ```
    ///
    /// ```jsx
    /// <div className="[&:nth-child(3)]:px-2" />;
    /// ```
    ///
    /// ## Options
    ///
    /// By default, this rule checks the `class` and `className` JSX attributes.
    /// The `attributes` option adds more JSX attributes to check, and `functions`
    /// enables checking string arguments and tagged templates in matching utilities.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "attributes": ["classList"],
    ///         "functions": ["clsx"]
    ///     }
    /// }
    /// ```
    ///
    /// ### attributes
    ///
    /// Additional JSX attribute names to check.
    ///
    /// Default: `[]` (the `class` and `className` attributes are always checked).
    ///
    /// ### functions
    ///
    /// Function or tagged template names whose classes will be checked for arbitrary values.
    ///
    /// Default: `[]`.
    ///
    /// ```jsx,use_options,expect_diagnostic
    /// <div className={clsx("w-[400px]")} />;
    /// ```
    ///
    pub NoTailwindArbitraryValue {
        version: "next",
        name: "noTailwindArbitraryValue",
        language: "jsx",
        sources: &[RuleSource::EslintTailwindcss("no-arbitrary-value").same()],
        domains: &[RuleDomain::Tailwind],
        recommended: false,
    }
}

impl Rule for NoTailwindArbitraryValue {
    type Query = Ast<AnyClassStringLike>;
    type State = TextRange;
    type Signals = Vec<TextRange>;
    type Options = NoTailwindArbitraryValueOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        if node.should_visit(options).is_none() {
            return vec![];
        }

        arbitrary_ranges_in_node(node)
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

struct ClassStringSource {
    text: TokenText,
    content_start: TextSize,
}

fn class_string_source(node: &AnyClassStringLike) -> Option<ClassStringSource> {
    match node {
        AnyClassStringLike::JsxString(jsx_string) => {
            let token = jsx_string.value_token().ok()?;
            Some(ClassStringSource {
                text: jsx_string.inner_string_text().ok()?,
                content_start: token.text_trimmed_range().start() + TextSize::from(1),
            })
        }
        AnyClassStringLike::JsStringLiteralExpression(string_literal) => {
            let token = string_literal.value_token().ok()?;
            Some(ClassStringSource {
                text: string_literal.inner_string_text().ok()?,
                content_start: token.text_trimmed_range().start() + TextSize::from(1),
            })
        }
        AnyClassStringLike::JsTemplateChunkElement(chunk) => {
            let token = chunk.template_chunk_token().ok()?;
            Some(ClassStringSource {
                text: token.token_text(),
                content_start: token.text_trimmed_range().start(),
            })
        }
        AnyClassStringLike::JsLiteralMemberName(member_name) => {
            let token = member_name.value().ok()?;
            let quote_offset = if token.kind() == JsSyntaxKind::JS_STRING_LITERAL {
                TextSize::from(1)
            } else {
                TextSize::from(0)
            };

            Some(ClassStringSource {
                text: member_name.name().ok()?,
                content_start: token.text_trimmed_range().start() + quote_offset,
            })
        }
    }
}

fn arbitrary_ranges_in_node(node: &AnyClassStringLike) -> Vec<TextRange> {
    let Some(source) = class_string_source(node) else {
        return vec![];
    };

    if !source.text.text().contains('[') {
        return vec![];
    }

    let parse = parse_tailwind(source.text.text());
    arbitrary_ranges(&parse.tree().candidates(), source.content_start)
}
