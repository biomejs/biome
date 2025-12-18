use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, AnyHtmlElement, HtmlAttribute, HtmlString, inner_string_text,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, TokenText};
use biome_rule_options::use_tailwind_shorthand_classes::UseTailwindShorthandClassesOptions;
use biome_tailwind_logic::lint::use_tailwind_shorthand_classes::{
    TailwindShorthandViolation, analyze_tailwind_shorthand,
};

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce using less Tailwind utilities instead of multiple utilities that are functionally the same.
    ///
    /// This rule detects sequences of Tailwind CSS utility classes that can be replaced by a single
    /// shorter utility. Using shorthands reduces duplication, keeps class lists readable, and helps
    /// prevent drift where one side gets updated but the matching side does not.
    ///
    /// Notes:
    /// - Values must match to compress (for example, `ml-2 mr-3` is not compressed).
    /// - Variants must match to compress (for example, `hover:ml-2 mr-2` is not compressed).
    /// - If an equivalent shorthand already exists for the same key and value, the rule highlights the
    ///   redundant longhands without suggesting an additional shorthand.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div class="ml-2 mr-2"></div>
    /// ```
    /// ```html,expect_diagnostic
    /// <div class="pl-2 pr-2 pt-2 pb-2"></div>
    /// ```
    /// ```html,expect_diagnostic
    /// <div class="hover:w-4 hover:h-4"></div>
    /// ```
    /// ```html,expect_diagnostic
    /// <div class="border-x border-y"></div>
    /// ```
    /// ```html,expect_diagnostic
    /// <div class="overflow-hidden text-ellipsis whitespace-nowrap"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="mx-2 -my-2"></div>
    /// <div class="p-2 pl-4"></div>
    /// <div class="hover:size-4"></div>
    /// <div class="border"></div>
    /// <div class="truncate"></div>
    /// ```
    ///
    pub UseTailwindShorthandClasses {
        version: "next",
        name: "useTailwindShorthandClasses",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Tailwind],
        // Inspired because this rule is actually a little more intelligent than the original ESLint version.
        sources: &[RuleSource::EslintBetterTailwindcss("enforce-shorthand-classes").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseTailwindShorthandClasses {
    type Query = Ast<AnyHtmlElement>;
    type State = (HtmlString, TailwindShorthandViolation);
    type Signals = Box<[Self::State]>;
    type Options = UseTailwindShorthandClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        // Get attributes list from element (supports normal and self-closing tags)
        let attributes = match element {
            AnyHtmlElement::HtmlElement(html_element) => {
                let Ok(opening) = html_element.opening_element() else {
                    return [].into();
                };
                opening.attributes()
            }
            AnyHtmlElement::HtmlSelfClosingElement(self_closing) => self_closing.attributes(),
            _ => return [].into(),
        };

        // Find `class` attribute
        let Some(class_attr) = attributes.find_by_name("class") else {
            return [].into();
        };

        // Extract all HtmlString nodes inside the initializer (if any)
        let string_nodes = extract_html_string_nodes(&class_attr);

        let mut signals: Vec<Self::State> = Vec::new();

        for html_string in string_nodes {
            // Get the inner string text (without quotes) and its TokenText to compute ranges
            let Ok(value_token) = html_string.value_token() else {
                continue;
            };
            let inner_text: TokenText = inner_string_text(&value_token);

            let violations = analyze_tailwind_shorthand(&inner_text);
            for mut violation in violations {
                // Adjust ranges to be relative to the original attribute value token
                // The `inner_string_text` is a view over the token's content without quotes,
                // so we need to offset by the start of the inner text inside the token's full range.
                let inner_source_range = inner_text.source_range(value_token.text_range());
                let offset_start = inner_source_range.start();

                for range in &mut violation.original_ranges {
                    *range =
                        TextRange::new(range.start() + offset_start, range.end() + offset_start);
                }

                signals.push((html_string.clone(), violation));
            }
        }

        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (html_string, violation) = state;

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            html_string.range(),
            markup! {
                "Prefer Tailwind shorthand utilities over multiple longhand utilities."
            },
        );

        // Highlight each longhand class occurrence
        for (idx, range) in violation.original_ranges.iter().enumerate() {
            let original = violation
                .originals
                .get(idx)
                .map_or("this utility", |s| s.as_str());
            diagnostic = diagnostic.detail(
                *range,
                markup! {
                    "Longhand utility "<Emphasis>{original}</Emphasis>" used here."
                },
            );
        }

        // Suggest the shorthand replacement when available
        if let Some(replacement) = &violation.replacement {
            diagnostic = diagnostic.note(markup! {
                "You can replace them with the shorthand "<Emphasis>{replacement}</Emphasis>" to reduce duplication and improve readability."
            });
        }

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let (html_string, violation) = state;

        // Original token text (quoted, e.g. "ml-2 mr-2")
        let old_token = html_string.value_token().ok()?;
        let original_str = old_token.text_trimmed();

        // Extract inner content and quotes (if any).
        // HtmlString tokens use quotes; we handle both single and double quotes.
        let (content_start, content_end, quote_prefix, quote_suffix) =
            if original_str.starts_with('"') || original_str.starts_with('\'') {
                (
                    1usize,
                    original_str.len().saturating_sub(1),
                    original_str.chars().next().unwrap_or('"'),
                    original_str.chars().last().unwrap_or('"'),
                )
            } else {
                (0usize, original_str.len(), '\0', '\0')
            };

        let inner = &original_str[content_start..content_end];

        // Tokenize on ASCII whitespace to operate on class tokens
        let parts: Vec<&str> = inner.split_whitespace().collect();
        let originals: Vec<&str> = violation.originals.iter().map(|s| s.as_str()).collect();

        // Find insertion point for the replacement: the first occurrence among the originals
        let mut insert_at: Option<usize> = None;
        for (i, p) in parts.iter().enumerate() {
            if originals.iter().any(|o| o == p) {
                insert_at = Some(i);
                break;
            }
        }

        // Build the new list of class tokens:
        // - skip all originals (longhands)
        // - insert the replacement (if any) at the first original's position
        let mut new_parts: Vec<String> = Vec::with_capacity(parts.len().saturating_add(1));

        for (i, p) in parts.iter().enumerate() {
            if Some(i) == insert_at
                && let Some(repl) = &violation.replacement {
                    new_parts.push(repl.clone());
                }
            if !originals.iter().any(|o| o == p) {
                new_parts.push((*p).to_string());
            }
        }

        // If we didn't see any of the originals in the tokenized list (edge-case),
        // but we do have a replacement, append it at the end to avoid losing the fix.
        if insert_at.is_none()
            && let Some(repl) = &violation.replacement {
                new_parts.push(repl.clone());
            }

        let new_inner = new_parts.join(" ");

        // Recompose full token text with the original quotes (if present)
        let new_text = if content_start == 0 {
            new_inner.clone()
        } else {
            let mut s = String::with_capacity(new_inner.len() + 2);
            s.push(quote_prefix);
            s.push_str(&new_inner);
            s.push(quote_suffix);
            s
        };

        // If nothing changed, don't emit an action
        if new_text == original_str {
            return None;
        }

        // Create the new token and apply the mutation
        let new_token =
            biome_html_syntax::HtmlSyntaxToken::new_detached(old_token.kind(), &new_text, [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token(old_token, new_token);

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace longhand Tailwind utilities with the shorthand." }.to_owned(),
            mutation,
        ))
    }
}

fn extract_html_string_nodes(attr: &HtmlAttribute) -> Vec<HtmlString> {
    let mut strings: Vec<HtmlString> = Vec::new();

    let Some(initializer) = attr.initializer() else {
        return strings;
    };

    let Ok(value) = initializer.value() else {
        return strings;
    };

    match value {
        AnyHtmlAttributeInitializer::HtmlString(s) => strings.push(s),
        AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(expr) => {
            // Expressions may contain nested HtmlString tokens; collect descendants
            strings.extend(expr.syntax().descendants().filter_map(HtmlString::cast));
        }
    }

    strings
}
