use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_json_syntax::{JsonLanguage, JsonRoot};
use biome_rowan::{AstNode, SyntaxNode, SyntaxTriviaPiece, TextRange};
use biome_rule_options::no_empty_documentation::NoEmptyDocumentationOptions;

declare_lint_rule! {
    /// Disallow empty documentation.
    ///
    /// Enforces that comments are not empty. This helps maintain code quality by preventing meaningless
    /// or placeholder comments that don't provide any documentation value.
    ///
    /// Empty comments clutter the codebase and should be removed. This rule catches single-line comments (`//`)
    /// and block comments (`/* */`) that contain no meaningful content.
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

        let mut found = Vec::new();
        found.append(&mut empty_comments(syntax_node));

        for descendant in syntax_node.descendants() {
            found.append(&mut empty_comments(&descendant));
        }

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

fn leading_comments(
    syntax_node: &SyntaxNode<JsonLanguage>,
) -> Vec<SyntaxTriviaPiece<JsonLanguage>> {
    if let Some(token) = syntax_node.first_token() {
        token
            .leading_trivia()
            .pieces()
            .filter(|piece| piece.is_comments())
            .collect()
    } else {
        Vec::new()
    }
}

fn trailing_comments(
    syntax_node: &SyntaxNode<JsonLanguage>,
) -> Vec<SyntaxTriviaPiece<JsonLanguage>> {
    if let Some(token) = syntax_node.first_token() {
        token
            .trailing_trivia()
            .pieces()
            .filter(|piece| piece.is_comments())
            .collect()
    } else {
        Vec::new()
    }
}

fn is_empty_comment(trivia_piece: &SyntaxTriviaPiece<JsonLanguage>) -> bool {
    let text = trivia_piece.text().trim();

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

fn empty_comments(syntax_node: &SyntaxNode<JsonLanguage>) -> Vec<TextRange> {
    let mut comments = Vec::new();

    comments.append(&mut leading_comments(syntax_node));
    comments.append(&mut trailing_comments(syntax_node));

    comments
        .iter()
        .filter(|comment| is_empty_comment(comment))
        .map(|comment| comment.text_range())
        .collect()
}
