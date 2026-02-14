use std::borrow::Cow;

use biome_analyze::shared::class_dedup::{duplicate_classes_diagnostic, find_duplicate_classes};
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
use biome_rule_options::no_duplicate_classes::NoDuplicateClassesOptions;

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
        sources: &[RuleSource::EslintBetterTailwindcss("no-duplicate-classes").inspired()],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

/// State returned by the rule when duplicates are found.
#[derive(Debug)]
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
    type Options = NoDuplicateClassesOptions;

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
        let value_str = inner_text.text();

        let result = find_duplicate_classes(value_str)?;

        Some(DuplicateClassesState {
            html_string,
            deduplicated: result.deduplicated.into(),
            duplicates: result.duplicates.into_iter().map(Into::into).collect(),
            is_single_quote,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(duplicate_classes_diagnostic(
            category!("assist/source/noDuplicateClasses"),
            ctx.query().range(),
            &state.duplicates,
        ))
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
