use crate::MarkdownFormatContext;
use biome_formatter::{
    Buffer, Format, FormatResult,
    prelude::{Formatter, text},
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
                }
            }
        }

        // Write the original source text as-is
        text(
            &self.node.to_string(),
            self.node.text_trimmed_range().start(),
        )
        .fmt(f)
    }
}

pub fn format_bogus_node(
    node: &biome_rowan::SyntaxNode<biome_markdown_syntax::MarkdownLanguage>,
) -> FormatMarkdownVerbatimNode<'_> {
    FormatMarkdownVerbatimNode { node }
}
