use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_factory::make;
use biome_html_syntax::{
    HtmlAttribute, HtmlString, HtmlSyntaxKind, HtmlSyntaxToken, inner_string_text,
};
use biome_rowan::{AstNode, BatchMutationExt};
use rustc_hash::FxHashSet;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Disallow duplicate CSS classes.
    ///
    /// Detects and removes duplicate CSS classes in HTML `class` attributes.
    ///
    /// Duplicate classes are redundant and can indicate copy-paste errors or merge conflicts.
    /// This rule helps keep your class strings clean by detecting and auto-fixing duplicates.
    ///
    /// Note that this rule collapses all whitespace (including newlines) into single spaces,
    /// consistent with [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div class="flex flex"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div class="p-4 text-red-500 p-4 bg-white"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="flex p-4"></div>
    /// ```
    ///
    /// ```html
    /// <div class="p-4 text-red-500 bg-white"></div>
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
    /// The list of duplicate class names found (sorted for deterministic output).
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

        // Get the inner string (without quotes) and find duplicates
        let inner_text = inner_string_text(&value_token);
        let mut seen: FxHashSet<&str> = FxHashSet::default();
        let mut duplicate_set: FxHashSet<&str> = FxHashSet::default();
        let mut deduplicated_parts: Vec<&str> = Vec::new();

        for class in inner_text.text().split_whitespace() {
            if seen.contains(class) {
                duplicate_set.insert(class);
            } else {
                seen.insert(class);
                deduplicated_parts.push(class);
            }
        }

        if duplicate_set.is_empty() {
            return None;
        }

        let deduplicated = deduplicated_parts.join(" ");
        // Sort duplicate names alphabetically for deterministic diagnostic messages.
        // Note: This does NOT affect the fix output - class order is always preserved.
        let mut duplicates: Vec<Box<str>> = duplicate_set.into_iter().map(Into::into).collect();
        duplicates.sort();

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
                rule_category!(),
                node.range(),
                markup! {
                    "This "<Emphasis>"class"</Emphasis>" attribute contains a duplicate class."
                },
            )
            .note(markup! {
                "The class "<Emphasis>{&*state.duplicates[0]}</Emphasis>" appears multiple times."
            })
        } else {
            let duplicates_list = state
                .duplicates
                .iter()
                .map(|s| format!("{}", s))
                .collect::<Vec<_>>()
                .join(", ");

            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This "<Emphasis>"class"</Emphasis>" attribute contains duplicate classes."
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

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
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
