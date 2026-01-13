use std::borrow::Cow;
use std::collections::HashSet;

use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_html_factory::make;
use biome_html_syntax::{
    HtmlAttribute, HtmlString, HtmlSyntaxKind, HtmlSyntaxToken, inner_string_text,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::HtmlRuleAction;

declare_source_rule! {
    /// Remove duplicate CSS classes.
    ///
    /// Detects and removes duplicate CSS classes in HTML `class` attributes.
    ///
    /// This action helps keep your class strings clean by detecting and removing duplicates.
    ///
    /// Duplicate classes are redundant and can indicate copy-paste errors or merge conflicts.
    ///
    /// ## Examples
    ///
    /// ```html,expect_diff
    /// <div class="flex flex"></div>
    /// ```
    ///
    /// ```html,expect_diff
    /// <div class="p-4 text-red-500 p-4 bg-white"></div>
    /// ```
    ///
    pub NoDuplicateClasses {
        version: "next",
        name: "noDuplicateClasses",
        language: "html",
        sources: &[RuleSource::EslintBetterTailwindcss("no-duplicate-classes").same()],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

/// State returned by the rule when duplicates are found.
pub struct DuplicateClassesState {
    /// The HtmlString node to replace.
    html_string: HtmlString,
    /// The deduplicated class string.
    deduplicated: Box<str>,
    /// The list of duplicate class names found.
    duplicates: Box<[Box<str>]>,
    /// Whether the original string used single quotes.
    is_single_quote: bool,
}

impl Rule for NoDuplicateClasses {
    type Query = Ast<HtmlAttribute>;
    type State = DuplicateClassesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let attribute = ctx.query();

        // Only check "class" attribute
        let name = attribute.name().ok()?;
        let name_token = name.value_token().ok()?;
        if name_token.text_trimmed() != "class" {
            return None;
        }

        // Get the attribute value
        let initializer = attribute.initializer()?;
        let value = initializer.value().ok()?;
        let html_string = value.as_html_string()?.clone();
        let value_token = html_string.value_token().ok()?;
        let value_text = value_token.text_trimmed();

        // Check if single-quoted
        let is_single_quote = value_text.starts_with('\'');

        // Get the inner string (without quotes) and parse into tokens,
        // preserving whitespace positions for minimal-change fixes.
        let inner_text = inner_string_text(&value_token);
        let value_str = inner_text.text();

        struct Token<'a> {
            prefix_start: usize,
            text_end: usize,
            class: &'a str,
        }

        let mut tokens: Vec<Token<'_>> = Vec::new();
        let mut pos = 0;

        while pos < value_str.len() {
            let prefix_start = pos;

            // Skip whitespace
            for c in value_str[pos..].chars() {
                if !c.is_whitespace() {
                    break;
                }
                pos += c.len_utf8();
            }

            if pos >= value_str.len() {
                break;
            }

            let class_start = pos;

            // Read class name
            for c in value_str[pos..].chars() {
                if c.is_whitespace() {
                    break;
                }
                pos += c.len_utf8();
            }

            tokens.push(Token {
                prefix_start,
                text_end: pos,
                class: &value_str[class_start..pos],
            });
        }

        // Identify duplicates and track which tokens to keep.
        // Use a Vec to track duplicates in order of first occurrence for deterministic output,
        // plus a HashSet for O(1) dedup checking.
        let mut seen: HashSet<&str> = HashSet::new();
        let mut duplicate_set: HashSet<&str> = HashSet::new();
        let mut duplicates_in_order: Vec<&str> = Vec::new();
        let mut kept_indices: Vec<usize> = Vec::new();

        for (idx, token) in tokens.iter().enumerate() {
            if seen.contains(token.class) {
                // Only add to the ordered list if this is the first time we see it as a duplicate
                if duplicate_set.insert(token.class) {
                    duplicates_in_order.push(token.class);
                }
            } else {
                seen.insert(token.class);
                kept_indices.push(idx);
            }
        }

        if duplicates_in_order.is_empty() {
            return None;
        }

        // Reconstruct the string, preserving original whitespace around kept classes
        let mut deduplicated = String::new();
        for &idx in &kept_indices {
            let token = &tokens[idx];
            deduplicated.push_str(&value_str[token.prefix_start..token.text_end]);
        }

        // Preserve trailing whitespace from the original string
        if let Some(last) = tokens.last() {
            deduplicated.push_str(&value_str[last.text_end..]);
        }

        let duplicates: Vec<Box<str>> = duplicates_in_order.into_iter().map(Into::into).collect();

        Some(DuplicateClassesState {
            html_string,
            deduplicated: deduplicated.into(),
            duplicates: duplicates.into_boxed_slice(),
            is_single_quote,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let diagnostic = if state.duplicates.len() == 1 {
            RuleDiagnostic::new(
                category!("assist/source/noDuplicateClasses"),
                node.range(),
                markup! {
                    "This class string contains a duplicate class."
                },
            )
            .note(markup! {
                "The class "<Emphasis>{&*state.duplicates[0]}</Emphasis>" appears multiple times."
            })
        } else {
            let duplicates_list = state
                .duplicates
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<&str>>()
                .join(", ");

            RuleDiagnostic::new(
                category!("assist/source/noDuplicateClasses"),
                node.range(),
                markup! {
                    "This class string contains duplicate classes."
                },
            )
            .note(markup! {
                "The classes "{duplicates_list}" appear multiple times."
            })
        };

        Some(diagnostic.note(markup! {
            "Duplicate classes are redundant and can indicate copy-paste errors or merge conflicts."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();

        // Create the new string token with proper quotes
        let new_token = if state.is_single_quote {
            html_string_literal_single_quotes(&state.deduplicated)
        } else {
            make::html_string_literal(&state.deduplicated)
        };

        let new_html_string = make::html_string(new_token);
        mutation.replace_node(state.html_string.clone(), new_html_string);

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! {
                "Remove the duplicate classes."
            }
            .to_owned(),
            mutation,
        ))
    }
}

/// Create a new string literal token with single quotes
fn html_string_literal_single_quotes(text: &str) -> HtmlSyntaxToken {
    HtmlSyntaxToken::new_detached(
        HtmlSyntaxKind::HTML_STRING_LITERAL,
        &format!("'{text}'"),
        [],
        [],
    )
}
