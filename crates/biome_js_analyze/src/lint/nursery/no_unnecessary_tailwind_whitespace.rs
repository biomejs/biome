use crate::JsRuleAction;
use crate::lint::nursery::use_sorted_classes::any_class_string_like::AnyClassStringLike;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_factory::make::{
    js_literal_member_name, js_string_literal, js_string_literal_expression,
    js_string_literal_single_quotes, js_template_chunk, js_template_chunk_element, jsx_string,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_sorted_classes::UseSortedClassesOptions;

declare_lint_rule! {
    /// Disallow unnecessary whitespace in Tailwind CSS class strings.
    ///
    /// Detects and removes unnecessary whitespace in CSS class strings, including:
    /// - Leading whitespace
    /// - Trailing whitespace
    /// - Multiple consecutive spaces between classes
    ///
    /// Clean class strings are easier to read and maintain.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="  flex p-4" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="flex p-4  " />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="flex    p-4" />;
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
    /// ## Options
    ///
    /// Use the same options as [`useSortedClasses`](/linter/rules/use-sorted-classes) to control
    /// which attributes and functions are checked.
    ///
    pub NoUnnecessaryTailwindWhitespace {
        version: "next",
        name: "noUnnecessaryTailwindWhitespace",
        language: "jsx",
        recommended: false,
        fix_kind: FixKind::Safe,
        issue_number: Some("1274"),
    }
}

impl Rule for NoUnnecessaryTailwindWhitespace {
    type Query = Ast<AnyClassStringLike>;
    type State = Box<str>;
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

        // Check for unnecessary whitespace
        let has_leading_whitespace = value_str.starts_with(char::is_whitespace);
        let has_trailing_whitespace = value_str.ends_with(char::is_whitespace);
        // Check for multiple consecutive whitespace characters (spaces, tabs, newlines)
        let has_multiple_whitespace = value_str
            .as_bytes()
            .windows(2)
            .any(|w| w[0].is_ascii_whitespace() && w[1].is_ascii_whitespace());

        if !has_leading_whitespace && !has_trailing_whitespace && !has_multiple_whitespace {
            return None;
        }

        // Normalize whitespace: split by whitespace and rejoin with single spaces
        let normalized: String = value_str.split_whitespace().collect::<Vec<_>>().join(" ");

        // Only report if something changed
        if normalized == value_str {
            return None;
        }

        Some(normalized.into())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let value = node.value()?;
        let value_str = value.text();

        let mut issues = Vec::new();
        if value_str.starts_with(char::is_whitespace) {
            issues.push("leading whitespace");
        }
        if value_str.ends_with(char::is_whitespace) {
            issues.push("trailing whitespace");
        }
        // Check for multiple consecutive whitespace characters
        let has_multiple_whitespace = value_str
            .as_bytes()
            .windows(2)
            .any(|w| w[0].is_ascii_whitespace() && w[1].is_ascii_whitespace());
        if has_multiple_whitespace {
            issues.push("multiple consecutive whitespace characters");
        }

        let issues_str = issues.join(", ");

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unnecessary whitespace in CSS class string: "{issues_str}"."
                },
            )
            .note(markup! {
                "Remove unnecessary whitespace for cleaner code."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let normalized = state;

        match ctx.query() {
            AnyClassStringLike::JsStringLiteralExpression(string_literal) => {
                let is_double_quote = string_literal
                    .value_token()
                    .map(|token| token.text_trimmed().starts_with('"'))
                    .unwrap_or(ctx.preferred_quote().is_double());
                let replacement = js_string_literal_expression(if is_double_quote {
                    js_string_literal(normalized)
                } else {
                    js_string_literal_single_quotes(normalized)
                });
                mutation.replace_node(string_literal.clone(), replacement);
            }
            AnyClassStringLike::JsLiteralMemberName(string_literal) => {
                let replacement = js_literal_member_name(if ctx.preferred_quote().is_double() {
                    js_string_literal(normalized)
                } else {
                    js_string_literal_single_quotes(normalized)
                });
                mutation.replace_node(string_literal.clone(), replacement);
            }
            AnyClassStringLike::JsxString(jsx_string_node) => {
                let is_double_quote = jsx_string_node
                    .value_token()
                    .map(|token| token.text_trimmed().starts_with('"'))
                    .unwrap_or(ctx.preferred_jsx_quote().is_double());
                let replacement = jsx_string(if is_double_quote {
                    js_string_literal(normalized)
                } else {
                    js_string_literal_single_quotes(normalized)
                });
                mutation.replace_node(jsx_string_node.clone(), replacement);
            }
            AnyClassStringLike::JsTemplateChunkElement(chunk) => {
                let replacement = js_template_chunk_element(js_template_chunk(normalized));
                mutation.replace_node(chunk.clone(), replacement);
            }
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Remove unnecessary whitespace."
            }
            .to_owned(),
            mutation,
        ))
    }
}
