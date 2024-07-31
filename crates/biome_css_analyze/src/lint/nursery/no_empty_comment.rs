use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{CssLanguage, CssRoot};
use biome_rowan::{AstNode, Direction, SyntaxTriviaPiece, TextRange};

declare_lint_rule! {
    /// Disallow empty comments.
    ///
    /// Empty comments can confuse developers reading the code.
    /// They are often the result of unfinished or accidentally removed comments, which can decrease the overall quality of the code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// /**/
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// /* */
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// /*
    ///
    ///  */
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// width: 100px; /* */
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// /* comment */
    /// ```
    ///
    /// ```css
    /// /*
    ///  * Multi-line Comment
    /// **/
    /// ```
    ///
    pub NoEmptyComment {
        version: "next",
        name: "noEmptyComment",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Stylelint("comment-no-empty")],
    }
}

impl Rule for NoEmptyComment {
    type Query = Ast<CssRoot>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        get_comments(node)
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Empty comment found."
                },
            )
            .note(markup! {
                    "Replace the empty comment with Valid comment or remove it."
            }),
        )
    }
}

fn get_comments(node: &CssRoot) -> Vec<TextRange> {
    let syntax = node.syntax();
    let mut all_comment_token: Vec<TextRange> = vec![];

    let is_comment = |token: &SyntaxTriviaPiece<CssLanguage>| {
        if !token.is_comments() {
            return false;
        }
        let text = token.text();
        let trimmed = text.trim();

        // Ensure minimum length for a valid CSS comment (/**/),
        // and prevent index out of bounds error in subsequent slicing
        if trimmed.len() < 4 {
            return false;
        }

        let content = &trimmed[2..trimmed.len() - 2];
        content.trim().is_empty()
    };

    for token in syntax.descendants_tokens(Direction::Next) {
        let leading_trivia_pieces = token.leading_trivia().pieces();
        let trailing_trivia_pieces = token.trailing_trivia().pieces();

        for trivia in leading_trivia_pieces {
            if is_comment(&trivia) {
                all_comment_token.push(trivia.text_range());
            }
        }

        for trivia in trailing_trivia_pieces {
            if is_comment(&trivia) {
                all_comment_token.push(trivia.text_range());
            }
        }
    }

    all_comment_token
}
