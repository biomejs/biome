use crate::MarkdownFormatContext;
use crate::comments::FormatMarkdownLeadingComments;
use biome_formatter::{
    Buffer, Format, FormatResult, LINE_TERMINATORS, normalize_newlines,
    prelude::{Formatter, text},
    trivia::format_trailing_comments_from_slice,
};
use biome_markdown_syntax::MarkdownSyntaxNode;
use biome_rowan::{Direction, SyntaxElement};

pub fn format_verbatim_node(node: &MarkdownSyntaxNode) -> FormatMarkdownVerbatimNode<'_> {
    FormatMarkdownVerbatimNode { node }
}

pub fn format_suppressed_node(node: &MarkdownSyntaxNode) -> FormatMarkdownVerbatimNode<'_> {
    FormatMarkdownVerbatimNode { node }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FormatMarkdownVerbatimNode<'node> {
    node: &'node MarkdownSyntaxNode,
}

impl Format<MarkdownFormatContext> for FormatMarkdownVerbatimNode<'_> {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        let comments = f.context().comments().clone();
        // Comment-only HTML tokens have no trimmed text; trimming can drop their comments.
        let (source, source_range) =
            if self.node.parent().is_none() || self.node.has_comments_descendants() {
                (
                    self.node.text_with_trivia(),
                    self.node.text_range_with_trivia(),
                )
            } else {
                (self.node.text_trimmed(), self.node.text_trimmed_range())
            };
        let leading_comments = comments.leading_comments(self.node);
        let outside_leading = leading_comments
            .partition_point(|comment| comment.piece().text_range().end() <= source_range.start());
        let (outside_leading, inside_leading) = leading_comments.split_at(outside_leading);
        Format::fmt(&FormatMarkdownLeadingComments(outside_leading), f)?;
        for comment in inside_leading {
            comment.mark_formatted();
        }

        // Track all tokens in the node so the formatter knows they've been seen
        for element in self.node.descendants_with_tokens(Direction::Next) {
            match element {
                SyntaxElement::Token(token) => {
                    // Bypass `assert_all_tracked` in
                    // https://github.com/biomejs/biome/blob/79d2e7b0f08b9f8ee4286ba15f9b4b8b1a5d1f52/crates/biome_formatter/src/printed_tokens.rs#L71-L81
                    f.state_mut().track_token(&token);
                }
                SyntaxElement::Node(node) => {
                    // Mark that we've checked suppression comments for this node
                    // to bypass `assert_checked_all_suppressions`
                    // in https://github.com/biomejs/biome/blob/79d2e7b0f08b9f8ee4286ba15f9b4b8b1a5d1f52/crates/biome_formatter/src/comments.rs#L965-L975
                    let comments = f.context().comments();
                    comments.mark_suppression_checked(&node);
                    for comment in comments.leading_dangling_trailing_comments(&node) {
                        comment.mark_formatted();
                    }
                }
            }
        }

        // Formatter text uses logical LF line endings; the printer applies the configured ending.
        text(
            &normalize_newlines(&source.to_string(), LINE_TERMINATORS),
            Some(source_range.start()),
        )
        .fmt(f)?;

        let trailing_comments = comments.trailing_comments(self.node);
        let outside_trailing = trailing_comments
            .partition_point(|comment| comment.piece().text_range().end() <= source_range.end());
        let (inside_trailing, outside_trailing) = trailing_comments.split_at(outside_trailing);
        for comment in inside_trailing {
            comment.mark_formatted();
        }
        Format::fmt(&format_trailing_comments_from_slice(outside_trailing), f)
    }
}

pub fn format_bogus_node(
    node: &biome_rowan::SyntaxNode<biome_markdown_syntax::MarkdownLanguage>,
) -> FormatMarkdownVerbatimNode<'_> {
    FormatMarkdownVerbatimNode { node }
}
