use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
    trivia::LeadingCommentTriviaPieces,
};
use biome_console::markup;
use biome_json_syntax::JsonRoot;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_empty_documentation::NoEmptyDocumentationOptions;

declare_lint_rule! {
    /// Disallow empty documentation.
    ///
    /// Enforces that documentation cannot be empty.
    /// This helps maintain code quality by preventing meaningless or placeholder nodes that don't provide any value.
    ///
    /// Empty comments clutter the codebase and should be removed. This rule catches single-line comments (`//`)
    /// and multi-line comments (`/* */`) that contain no meaningful content.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsonc,expect_diagnostic
    /// {
    ///   //
    ///   "name": "Jane Doe"
    /// }
    /// ```
    ///
    /// ```jsonc,expect_diagnostic
    /// {
    ///   /* */
    ///   "name": "Jane Doe"
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsonc
    /// {
    ///   /* Valid */
    ///   "name": "Jane Doe",
    /// }
    /// ```
    ///
    /// ```jsonc
    /// {
    ///   // Valid
    ///   "name": "Jane Doe"
    /// }
    /// ```
    ///
    pub NoEmptyDocumentation {
        version: "next",
        name: "noEmptyDocumentation",
        language: "json",
        recommended: false,
        sources: &[RuleSource::Stylelint("comment-no-empty").inspired()],
    }
}

impl Rule for NoEmptyDocumentation {
    type Query = Ast<JsonRoot>;
    type State = Vec<TextRange>;
    type Signals = Option<Self::State>;
    type Options = NoEmptyDocumentationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let syntax_node = node.syntax();

        let mut found: Vec<_> = LeadingCommentTriviaPieces::new(syntax_node)
            .filter(|comment| is_empty_comment(comment.text().trim()))
            .map(|comment| comment.text_range())
            .collect();

        found.sort_unstable_by_key(|range| (range.start(), range.end()));
        found.dedup();

        if found.is_empty() {
            return None;
        }

        Some(found)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.first()?,
            markup! {
                "Unexpected empty documentation."
            },
        );

        for range in &state[1..] {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "More empty documentation."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "Empty documentation provides no value. Remove them or add meaningful content."
        }))
    }
}

fn is_empty_comment(text: &str) -> bool {
    // Remove comment prefixes and check if anything remains
    let content = if let Some(stripped) = text.strip_prefix("//") {
        stripped.trim()
    } else if let Some(stripped) = text.strip_prefix("/*") {
        if let Some(stripped) = stripped.strip_suffix("*/") {
            stripped.trim()
        } else {
            stripped.trim()
        }
    } else {
        text
    };

    content.is_empty()
}
