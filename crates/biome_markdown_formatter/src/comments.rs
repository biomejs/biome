use biome_formatter::{
    FormatResult, FormatRule,
    comments::{CommentKind, CommentPlacement, CommentStyle, SourceComment},
    prelude::Formatter,
};
use biome_markdown_syntax::MarkdownLanguage;
use biome_rowan::SyntaxTriviaPieceComments;

use crate::MarkdownFormatterContext;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct MarkdownCommentStyle;

impl CommentStyle for MarkdownCommentStyle {
    type Language = MarkdownLanguage;

    fn is_suppression(_: &str) -> bool {
        true
    }

    fn get_comment_kind(_: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        CommentKind::Line
    }

    fn place_comment(
        &self,
        _: biome_formatter::comments::DecoratedComment<Self::Language>,
    ) -> biome_formatter::comments::CommentPlacement<Self::Language> {
        todo!()
    }
}

#[derive(Default)]
pub struct FormatMarkdownLeadingComment;

impl FormatRule<SourceComment<MarkdownLanguage>> for FormatMarkdownLeadingComment {
    type Context = MarkdownFormatterContext;

    fn fmt(
        &self,
        _: &SourceComment<MarkdownLanguage>,
        __: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        todo!();
    }
}
