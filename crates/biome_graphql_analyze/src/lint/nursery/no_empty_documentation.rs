use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{GraphqlDescription, GraphqlLanguage, GraphqlRoot};
use biome_rowan::{AstNode, SyntaxNode, SyntaxTriviaPiece, TextRange};
use biome_rule_options::no_empty_documentation::NoEmptyDocumentationOptions;

declare_lint_rule! {
    /// Disallow empty documentation.
    ///
    /// Enforces that comments are not empty. This helps maintain code quality by preventing meaningless
    /// or placeholder comments that don't provide any documentation value.
    ///
    /// Empty comments clutter the codebase and should be removed. This rule catches single-line comments (`#`),
    /// single-line descriptions (`" "`), and multi-line descriptions (`""" """`) that contain no meaningful content.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// #
    /// query {}
    /// ```
    ///
    /// ```graphql,expect_diagnostic
    /// type Example {
    ///   " "
    ///   something: Int!
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic
    /// type Example {
    ///   """
    ///
    ///   """
    ///   something: Int!
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql,ignore
    /// # Valid
    /// query {}
    /// ```
    ///
    /// ```graphql
    /// type Example {
    ///   "Valid"
    ///   something: Int!
    /// }
    /// ```
    ///
    /// ```graphql
    /// type Example {
    ///   """
    ///   Valid
    ///   """
    ///   something: Int!
    /// }
    /// ```
    ///
    pub NoEmptyDocumentation {
        version: "next",
        name: "noEmptyDocumentation",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::Stylelint("comment-no-empty").inspired()],
    }
}

impl Rule for NoEmptyDocumentation {
    type Query = Ast<GraphqlRoot>;
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

            if let Some(description) = GraphqlDescription::cast(descendant) {
                let value = description.to_trimmed_text();
                if is_empty_comment(value.text().trim()) {
                    found.push(description.range());
                }
            }
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
    syntax_node: &SyntaxNode<GraphqlLanguage>,
) -> Vec<SyntaxTriviaPiece<GraphqlLanguage>> {
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
    syntax_node: &SyntaxNode<GraphqlLanguage>,
) -> Vec<SyntaxTriviaPiece<GraphqlLanguage>> {
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

fn is_empty_comment(text: &str) -> bool {
    // Remove comment prefixes and check if anything remains
    let content = if let Some(stripped) = text.strip_prefix("#") {
        stripped.trim()
    } else if let Some(stripped) = text.strip_prefix("\"\"\"") {
        if let Some(stripped) = stripped.strip_suffix("\"\"\"") {
            stripped.trim()
        } else {
            stripped.trim()
        }
    } else if let Some(stripped) = text.strip_prefix("\"") {
        if let Some(stripped) = stripped.strip_suffix("\"") {
            stripped.trim()
        } else {
            stripped.trim()
        }
    } else {
        text
    };

    content.is_empty()
}

fn empty_comments(syntax_node: &SyntaxNode<GraphqlLanguage>) -> Vec<TextRange> {
    let mut comments = Vec::new();

    comments.append(&mut leading_comments(syntax_node));
    comments.append(&mut trailing_comments(syntax_node));

    comments
        .iter()
        .filter(|comment| is_empty_comment(comment.text().trim()))
        .map(|comment| comment.text_range())
        .collect()
}
