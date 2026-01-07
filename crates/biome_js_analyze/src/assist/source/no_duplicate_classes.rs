use std::borrow::Cow;

use crate::JsRuleAction;
use crate::lint::nursery::use_sorted_classes::any_class_string_like::AnyClassStringLike;
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_js_factory::make::{
    js_literal_member_name, js_string_literal, js_string_literal_expression,
    js_string_literal_single_quotes, js_template_chunk, js_template_chunk_element, jsx_string,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_duplicate_classes::NoDuplicateClassesOptions;
use rustc_hash::FxHashSet;

declare_source_rule! {
    /// Remove duplicate CSS classes.
    ///
    /// Detects and removes duplicate CSS classes in JSX `class` and `className` attributes,
    /// as well as in utility function calls like `clsx`, `cn`, `cva`, etc.
    ///
    /// Duplicate classes are redundant and can indicate copy-paste errors or merge conflicts.
    ///
    /// ## Examples
    ///
    /// ```jsx,expect_diff
    /// <div class="flex flex" />;
    /// ```
    ///
    /// ```jsx,expect_diff
    /// <div class="p-4 text-red-500 p-4 bg-white" />;
    /// ```
    ///
    /// ## Options
    ///
    /// Uses the same options as [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/)
    /// to control which attributes and functions are checked.
    ///
    pub NoDuplicateClasses {
        version: "next",
        name: "noDuplicateClasses",
        language: "jsx",
        sources: &[RuleSource::EslintBetterTailwindcss("no-duplicate-classes").inspired()],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

/// State returned by the rule when duplicates are found.
#[derive(Debug)]
pub struct DuplicateClassesState {
    /// The deduplicated class string.
    pub deduplicated: Box<str>,
    /// The list of duplicate class names found.
    pub duplicates: Box<[Box<str>]>,
}

impl Rule for NoDuplicateClasses {
    type Query = Ast<AnyClassStringLike>;
    type State = DuplicateClassesState;
    type Signals = Option<Self::State>;
    type Options = NoDuplicateClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let options = ctx.options();
        let node = ctx.query();

        if !node.should_visit(options)? {
            return None;
        }

        let value = node.value()?;
        let value_str = value.text();

        // Parse the class string into tokens, preserving whitespace positions.
        // Each token tracks: where its preceding whitespace starts, where the class ends,
        // and the class name itself.
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
        let mut seen: FxHashSet<&str> = FxHashSet::default();
        let mut duplicate_set: FxHashSet<&str> = FxHashSet::default();
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
            deduplicated: deduplicated.into(),
            duplicates: duplicates.into_boxed_slice(),
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

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let deduplicated = &state.deduplicated;

        match ctx.query() {
            AnyClassStringLike::JsStringLiteralExpression(string_literal) => {
                let is_double_quote = string_literal
                    .value_token()
                    .map(|token| token.text_trimmed().starts_with('"'))
                    .unwrap_or(ctx.preferred_quote().is_double());
                let replacement = js_string_literal_expression(if is_double_quote {
                    js_string_literal(deduplicated)
                } else {
                    js_string_literal_single_quotes(deduplicated)
                });
                mutation.replace_node(string_literal.clone(), replacement);
            }
            AnyClassStringLike::JsLiteralMemberName(string_literal) => {
                let is_double_quote = string_literal
                    .value()
                    .map(|token| token.text_trimmed().starts_with('"'))
                    .unwrap_or(ctx.preferred_quote().is_double());
                let replacement = js_literal_member_name(if is_double_quote {
                    js_string_literal(deduplicated)
                } else {
                    js_string_literal_single_quotes(deduplicated)
                });
                mutation.replace_node(string_literal.clone(), replacement);
            }
            AnyClassStringLike::JsxString(jsx_string_node) => {
                let is_double_quote = jsx_string_node
                    .value_token()
                    .map(|token| token.text_trimmed().starts_with('"'))
                    .unwrap_or(ctx.preferred_jsx_quote().is_double());
                let replacement = jsx_string(if is_double_quote {
                    js_string_literal(deduplicated)
                } else {
                    js_string_literal_single_quotes(deduplicated)
                });
                mutation.replace_node(jsx_string_node.clone(), replacement);
            }
            AnyClassStringLike::JsTemplateChunkElement(chunk) => {
                // Whitespace is preserved by the deduplication logic, including
                // leading/trailing spaces needed for template expression boundaries
                let replacement = js_template_chunk_element(js_template_chunk(deduplicated));
                mutation.replace_node(chunk.clone(), replacement);
            }
        };

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
