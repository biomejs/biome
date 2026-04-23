use crate::shared::any_class_string_like::AnyClassStringLike;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::JsSyntaxKind;
use biome_rowan::{AstNode, AstNodeList, TextRange, TextSize, TokenText};
use biome_rule_options::no_arbitrary_value::NoArbitraryValueOptions;
use biome_rule_options::use_sorted_classes::UseSortedClassesOptions;
use biome_tailwind_parser::parse_tailwind;
use biome_tailwind_syntax::{AnyTwCandidate, AnyTwFullCandidate, AnyTwModifier, AnyTwValue};

declare_lint_rule! {
    /// Disallow arbitrary values in Tailwind CSS utility classes.
    ///
    /// Arbitrary values (e.g. `w-[400px]`, `text-[#555]`, `[color:red]`) bypass
    /// the design system. This rule reports them so teams can enforce a constraint-based approach.
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
    /// ```json,options
    /// {
    ///     "options": {
    ///         "attributes": ["classList"],
    ///         "functions": ["clsx", "cn", "classnames", "tw", "tw.*"]
    ///     }
    /// }
    /// ```
    ///
    /// ### attributes
    ///
    /// Additional JSX attribute names to check (beyond the default `class` and `className`).
    ///
    /// ### functions
    ///
    /// Function or tagged template names whose classes will be checked for arbitrary values.
    ///
    pub NoArbitraryValue {
        version: "next",
        name: "noArbitraryValue",
        language: "jsx",
        recommended: false,
        fix_kind: FixKind::None,
    }
}

impl Rule for NoArbitraryValue {
    type Query = Ast<AnyClassStringLike>;
    type State = TextRange;
    type Signals = Vec<TextRange>;
    type Options = NoArbitraryValueOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        let visit_options = UseSortedClassesOptions {
            attributes: options.attributes.clone(),
            functions: options.functions.clone(),
        };

        if node.should_visit(&visit_options).is_none() {
            return vec![];
        }

        arbitrary_ranges_in_node(node)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! { "Avoid arbitrary values in Tailwind CSS classes." },
            )
            .note(markup! {
                "Arbitrary values bypass the design system. Use a design token instead."
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
    TextSize::from(u32::try_from(offset).unwrap())
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

fn arbitrary_ranges_in_node(node: &AnyClassStringLike) -> Vec<TextRange> {
    let Some(source) = class_string_source(node) else {
        return vec![];
    };

    let mut results = Vec::new();

    for (class_offset, class_name) in class_ranges(source.text.text()) {
        let parse = parse_tailwind(class_name);
        let class_start = source.content_start + text_size(class_offset);

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
