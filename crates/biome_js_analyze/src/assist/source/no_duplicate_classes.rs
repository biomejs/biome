use std::borrow::Cow;

use crate::JsRuleAction;
use crate::shared::any_class_string_like::AnyClassStringLike;
use biome_analyze::shared::class_dedup::{duplicate_classes_diagnostic, find_duplicate_classes};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make::{
    js_literal_member_name, js_string_literal, js_string_literal_expression,
    js_string_literal_single_quotes, js_template_chunk, js_template_chunk_element, jsx_string,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_duplicate_classes::NoDuplicateClassesOptions;

declare_source_rule! {
    /// Remove repeated CSS class names.
    ///
    /// The action checks JSX `class` and `className` attributes by default. It can also check
    /// additional attributes, function calls, and tagged templates configured by name. A tagged
    /// template is a template literal with a function name immediately before its opening backtick.
    /// Duplicate class names are redundant and often indicate a copy-and-paste mistake or merge
    /// conflict.
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
    /// ### `attributes`
    ///
    /// Adds JSX attributes to check alongside `class` and `className`. Defaults to an empty list.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "attributes": ["tw"]
    ///     }
    /// }
    /// ```
    ///
    /// ```jsx,use_options,expect_diff
    /// <div tw="flex flex" />
    /// ```
    ///
    /// ### `functions`
    ///
    /// Lists utility functions or tagged templates whose strings contain CSS classes. Defaults to
    /// an empty list.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "functions": ["clsx"]
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options,expect_diff
    /// const className = clsx("p-4 p-4");
    /// ```
    ///
    pub NoDuplicateClasses {
        version: "2.4.0",
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
    deduplicated: Box<str>,
    /// The list of duplicate class names found.
    duplicates: Box<[Box<str>]>,
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

        let result = find_duplicate_classes(value_str)?;

        Some(DuplicateClassesState {
            deduplicated: result.deduplicated.into(),
            duplicates: result.duplicates.into_iter().map(Into::into).collect(),
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(duplicate_classes_diagnostic(
            rule_category!(),
            ctx.query().range(),
            &state.duplicates,
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let deduplicated = &state.deduplicated;

        match ctx.query() {
            AnyClassStringLike::JsStringLiteralExpression(string_literal) => {
                let is_double_quote = string_literal
                    .value_token()
                    .map_or(ctx.preferred_quote().is_double(), |token| token.text_trimmed().starts_with('"'));
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
                    .map_or(ctx.preferred_quote().is_double(), |token| token.text_trimmed().starts_with('"'));
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
                    .map_or(ctx.preferred_jsx_quote().is_double(), |token| token.text_trimmed().starts_with('"'));
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
