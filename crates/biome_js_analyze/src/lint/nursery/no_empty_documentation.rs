use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
    trivia::LeadingCommentTriviaPieces,
};
use biome_console::markup;
use biome_js_syntax::AnyJsRoot;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_empty_documentation::NoEmptyDocumentationOptions;

declare_lint_rule! {
    /// Disallow empty documentation.
    ///
    /// Enforces that documentation cannot be empty.
    /// This helps maintain code quality by preventing meaningless or placeholder nodes that don't provide any value.
    ///
    /// Empty documentation nodes clutter the codebase and should be removed. This rule catches single-line comments (`//`),
    /// multi-line comments (`/* */`), and JSDoc comments (`/** */`) that contain no meaningful content.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// //
    /// var a = 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /* */
    /// const b = 2;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /**
    ///  *
    ///  */
    /// const c = 3;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // Valid
    /// var a = 1;
    /// ```
    ///
    /// ```js
    /// /* Valid */
    /// const b = 2;
    /// ```
    ///
    /// ```js
    /// /**
    ///  * Valid
    ///  */
    /// const c = 3;
    /// ```
    ///
    pub NoEmptyDocumentation {
        version: "next",
        name: "noEmptyDocumentation",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Stylelint("comment-no-empty").inspired()],
    }
}

impl Rule for NoEmptyDocumentation {
    type Query = Ast<AnyJsRoot>;
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
    // Remove comment delimiters and normalize JSDoc stars before checking for content.
    if let Some(stripped) = text.strip_prefix("//") {
        return stripped.trim().is_empty();
    } else if let Some(stripped) = text.strip_prefix("/**") {
        if let Some(stripped) = stripped.strip_suffix("*/") {
            return is_empty_block_comment_content(stripped);
        } else {
            return is_empty_block_comment_content(stripped);
        }
    } else if let Some(stripped) = text.strip_prefix("/*") {
        if let Some(stripped) = stripped.strip_suffix("*/") {
            return is_empty_block_comment_content(stripped);
        } else {
            return is_empty_block_comment_content(stripped);
        }
    }

    text.is_empty()
}

fn is_empty_block_comment_content(content: &str) -> bool {
    content
        .lines()
        .map(|line| {
            line.trim_start()
                .strip_prefix('*')
                .unwrap_or(line.trim_start())
        })
        .all(|line| line.trim().is_empty())
}
