use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
    trivia::LeadingCommentTriviaPiecesIterator,
};
use biome_console::markup;
use biome_html_syntax::HtmlRoot;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_empty_documentation::NoEmptyDocumentationOptions;

declare_lint_rule! {
    /// Disallow empty documentation.
    ///
    /// Enforces that documentation cannot be empty.
    /// This helps maintain code quality by preventing meaningless or placeholder nodes that don't provide any value.
    ///
    /// Empty documentation nodes clutter the codebase and should be removed. This rule catches comments (`<!-- -->`) that contain no meaningful content.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <!-- -->
    /// <input/>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <!-- Valid -->
    /// <input/>
    /// ```
    ///
    pub NoEmptyDocumentation {
        version: "next",
        name: "noEmptyDocumentation",
        language: "html",
        recommended: false,
        sources: &[RuleSource::Stylelint("comment-no-empty").inspired()],
    }
}

impl Rule for NoEmptyDocumentation {
    type Query = Ast<HtmlRoot>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = NoEmptyDocumentationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let syntax_node = node.syntax();

        let found: Vec<_> = LeadingCommentTriviaPiecesIterator::new(syntax_node)
            .filter(|comment| is_empty_comment(comment.text().trim()))
            .map(|comment| comment.text_range())
            .collect();

        if found.is_empty() {
            return found;
        }

        found
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Unexpected empty documentation."
                },
            )
            .note(markup! {
                "Empty documentation provides no value. Remove them or add meaningful content."
            }),
        )
    }
}

fn is_empty_comment(text: &str) -> bool {
    // Remove comment prefixes and check if anything remains
    let content = if let Some(stripped) = text.strip_prefix("<!--") {
        if let Some(stripped) = stripped.strip_suffix("-->") {
            stripped.trim()
        } else {
            stripped.trim()
        }
    } else {
        text
    };

    content.is_empty()
}
