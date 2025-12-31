use crate::JsRuleAction;
use crate::lint::nursery::use_sorted_classes::any_class_string_like::AnyClassStringLike;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make::{
    js_literal_member_name, js_string_literal, js_string_literal_expression,
    js_string_literal_single_quotes, js_template_chunk, js_template_chunk_element, jsx_string,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_sorted_classes::UseSortedClassesOptions;
use rustc_hash::FxHashSet;

declare_lint_rule! {
    /// Disallow duplicate CSS classes.
    ///
    /// Detects and removes duplicate CSS classes in JSX `class` and `className` attributes,
    /// as well as in utility function calls like `clsx`, `cn`, `cva`, etc.
    ///
    /// Duplicate classes are redundant and can indicate copy-paste errors or merge conflicts.
    /// This rule helps keep your class strings clean by detecting and auto-fixing duplicates.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="flex flex" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="p-4 text-red-500 p-4 bg-white" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="hover:bg-blue-500 hover:bg-blue-500" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div class="flex p-4" />;
    /// ```
    ///
    /// ```jsx
    /// <div class="p-4 text-red-500 bg-white" />;
    /// ```
    ///
    /// ```jsx
    /// <div className="hover:bg-blue-500 focus:bg-blue-500" />;
    /// ```
    ///
    /// ## Options
    ///
    /// Use the same options as [`useSortedClasses`](/linter/rules/use-sorted-classes) to control
    /// which attributes and functions are checked.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "attributes": ["classList"],
    ///         "functions": ["clsx", "cva", "tw"]
    ///     }
    /// }
    /// ```
    ///
    pub NoDuplicateClasses {
        version: "next",
        name: "noDuplicateClasses",
        language: "jsx",
        sources: &[RuleSource::EslintBetterTailwindcss("no-duplicate-classes").same()],
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
    type Options = UseSortedClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let options = ctx.options();
        let node = ctx.query();

        if !node.should_visit(options)? {
            return None;
        }

        let value = node.value()?;
        let value_str = value.text();

        // Split by whitespace and track duplicates
        let mut seen: FxHashSet<&str> = FxHashSet::default();
        let mut duplicate_set: FxHashSet<&str> = FxHashSet::default();
        let mut deduplicated_parts: Vec<&str> = Vec::new();

        for class in value_str.split_whitespace() {
            if seen.contains(class) {
                // Found a duplicate - track it (HashSet ensures uniqueness)
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
        let duplicates: Vec<Box<str>> = duplicate_set.into_iter().map(Into::into).collect();

        Some(DuplicateClassesState {
            deduplicated: deduplicated.into(),
            duplicates: duplicates.into_boxed_slice(),
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let duplicates_str = state
            .duplicates
            .iter()
            .map(|s| format!("`{}`", s))
            .collect::<Vec<_>>()
            .join(", ");

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Duplicate CSS class"<Emphasis>{
                        if state.duplicates.len() > 1 { "es" } else { "" }
                    }</Emphasis>" detected: "{duplicates_str}
                },
            )
            .note(markup! {
                "Remove duplicate classes to improve readability."
            }),
        )
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
                let replacement = js_template_chunk_element(js_template_chunk(deduplicated));
                mutation.replace_node(chunk.clone(), replacement);
            }
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Remove duplicate classes."
            }
            .to_owned(),
            mutation,
        ))
    }
}
