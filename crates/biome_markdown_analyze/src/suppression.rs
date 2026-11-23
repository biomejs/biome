use biome_analyze::{
    AnalyzerSuppression, Suppression, SuppressionComment, to_analyzer_suppressions,
};
use biome_languages::MdFileSource;
use biome_markdown_syntax::{MdQuote, MdRoot, MdxExpressionContent};
use biome_rowan::{AstNode, Direction, SyntaxElement, TextRange, WalkEvent};
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
    /// The literal range, literal text, and comment ranges of each MDX
    /// expression that holds only comments, in source order. Empty unless
    /// the file is MDX.
    /// `{/* biome-ignore ... */}` is the MDX form of a suppression comment.
    mdx_comments: Vec<(TextRange, String, Vec<TextRange>)>,
}

impl MarkdownSuppression {
    pub fn new(root: &MdRoot, file_source: MdFileSource) -> Self {
        let mut quoted_comments = Vec::new();
        let mut preorder = root.syntax().preorder_with_tokens(Direction::Next);
        while let Some(event) = preorder.next() {
            match event {
                WalkEvent::Enter(SyntaxElement::Node(node)) if !node.has_comments_descendants() => {
                    preorder.skip_subtree();
                }
                WalkEvent::Enter(SyntaxElement::Token(token))
                    if (token.has_leading_comments() || token.has_trailing_comments())
                        && token.ancestors().any(|node| MdQuote::can_cast(node.kind())) =>
                {
                    quoted_comments.extend(
                        token
                            .leading_trivia()
                            .pieces()
                            .chain(token.trailing_trivia().pieces())
                            .filter_map(|piece| piece.as_comments())
                            .map(|comment| comment.text_range()),
                    );
                }
                _ => {}
            }
        }

        // `preorder_of_kinds` only creates the nodes it yields, unlike a walk
        // over every descendant.
        let mdx_comments = if file_source.is_mdx() {
            root.syntax()
                .preorder_of_kinds(MdxExpressionContent::KIND_SET)
                .filter_map(|event| match event {
                    WalkEvent::Enter(node) => MdxExpressionContent::cast(node),
                    WalkEvent::Leave(_) => None,
                })
                .filter_map(|content| {
                    let ranges = content.comment_ranges().ok()??;
                    let token = content.value_token().ok()?;
                    // The lines of an expression in a block quote keep their
                    // `>` markers, like quoted HTML comments.
                    if content
                        .syntax()
                        .ancestors()
                        .any(|node| MdQuote::can_cast(node.kind()))
                    {
                        quoted_comments.extend(ranges.iter().copied());
                    }
                    Some((
                        token.text_trimmed_range(),
                        token.text_trimmed().to_string(),
                        ranges,
                    ))
                })
                .collect()
        } else {
            Vec::new()
        };

        Self {
            quoted_comments,
            mdx_comments,
        }
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

    fn parse_snippet(&self, range: TextRange) -> Vec<SuppressionComment<'_, Self::Diagnostic>> {
        let Some((_, text, comments)) = self.mdx_comment(range) else {
            return Vec::new();
        };
        comments
            .iter()
            .filter_map(|comment| {
                let text = &text[*comment - range.start()];
                let suppressions = self.parse_comment(text, *comment);
                (!suppressions.is_empty()).then_some(SuppressionComment {
                    range: *comment,
                    suppressions,
                })
            })
            .collect()
    }

    fn has_snippet_comments(&self) -> bool {
        !self.mdx_comments.is_empty()
    }

    fn is_comment_snippet(&self, range: TextRange) -> bool {
        self.mdx_comment(range).is_some()
    }
}

impl MarkdownSuppression {
    /// Returns the comment-only MDX expression whose literal spans `range`.
    fn mdx_comment(&self, range: TextRange) -> Option<&(TextRange, String, Vec<TextRange>)> {
        let index = self
            .mdx_comments
            .binary_search_by_key(&range.start(), |(literal, ..)| literal.start())
            .ok()?;
        let comment = &self.mdx_comments[index];
        (comment.0 == range).then_some(comment)
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
