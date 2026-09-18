use biome_analyze::{AnalyzerSuppression, Suppression, to_analyzer_suppressions};
use biome_markdown_syntax::{MdQuote, MdRoot};
use biome_rowan::{AstNode, Direction, TextRange};
use biome_suppression::{
    SuppressionDiagnostic, parse_suppression_comment, parse_suppression_comment_with_line_prefix,
};

pub struct MarkdownSuppression {
    /// Tracks the ranges of comments inside quote blocks.
    /// Quote blocks require a special care because if we have a suppression comment that goes on multiple lines,
    /// we need to be able to handle this particular case
    ///
    /// ```md
    /// > <!--
    /// > biome-ignore lint: this is quoted text, not a directive
    /// > -->
    /// ```
    ///
    /// Where the second line must keep the quote `>`.
    ///
    /// Storing the range allows us to not strip the `>` altogether, because the trivia stores the quotes too.
    /// This is a limitation of how rowan works, and it represents data (CST).
    quoted_comments: Vec<TextRange>,
}

impl MarkdownSuppression {
    pub fn new(root: &MdRoot) -> Self {
        let mut quoted_comments = Vec::new();
        for token in root
            .syntax()
            .descendants_with_tokens(Direction::Next)
            .filter_map(|element| element.into_token())
        {
            let in_quote = token.ancestors().any(|node| MdQuote::can_cast(node.kind()));
            for comment in token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
                .filter_map(|piece| piece.as_comments())
            {
                let range = comment.text_range();
                if in_quote {
                    quoted_comments.push(range);
                }
            }
        }

        Self { quoted_comments }
    }
}

impl Suppression for MarkdownSuppression {
    type Diagnostic = SuppressionDiagnostic;

    fn parse_comment<'a>(
        &self,
        text: &'a str,
        range: TextRange,
    ) -> Vec<Result<AnalyzerSuppression<'a>, Self::Diagnostic>> {
        let mut result = Vec::new();
        let suppressions = if self.quoted_comments.contains(&range) {
            parse_suppression_comment_with_line_prefix(text, strip_quote_prefix).collect::<Vec<_>>()
        } else {
            parse_suppression_comment(text).collect()
        };
        for suppression in suppressions {
            match suppression {
                Ok(suppression) => result.extend(
                    to_analyzer_suppressions(suppression, range)
                        .into_iter()
                        .map(Ok),
                ),
                Err(error) => result.push(Err(error)),
            }
        }
        result
    }
}

fn strip_quote_prefix(mut line: &str) -> &str {
    loop {
        let trimmed = line.trim_start_matches([' ', '\t']);
        let Some(rest) = trimmed.strip_prefix('>') else {
            return line;
        };
        line = rest.strip_prefix([' ', '\t']).unwrap_or(rest);
    }
}
