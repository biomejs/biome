use biome_formatter::{
    FormatResult, FormatRule,
    comments::{CommentKind, CommentPlacement, CommentStyle, DecoratedComment, SourceComment},
    prelude::Formatter,
};
use biome_markdown_syntax::MarkdownLanguage;
use biome_rowan::SyntaxTriviaPieceComments;

use crate::MarkdownFormatContext;

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
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        CommentPlacement::Default(comment)
    }
}

#[derive(Default)]
pub struct FormatMarkdownLeadingComment;

impl FormatRule<SourceComment<MarkdownLanguage>> for FormatMarkdownLeadingComment {
    type Context = MarkdownFormatContext;

    fn fmt(
        &self,
        _: &SourceComment<MarkdownLanguage>,
        _: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        Ok(())
    }
}
