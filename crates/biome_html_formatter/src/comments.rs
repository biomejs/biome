use biome_diagnostics_categories::category;
use biome_formatter::{
    FormatRule,
    comments::{
        CommentKind, CommentPlacement, CommentStyle, Comments, DecoratedComment, SourceComment,
    },
    prelude::*,
    write,
};
use biome_html_syntax::HtmlLanguage;
use biome_rowan::SyntaxTriviaPieceComments;
use biome_suppression::parse_suppression_comment;

use crate::context::HtmlFormatContext;

pub type HtmlComments = Comments<HtmlLanguage>;

#[derive(Default)]
pub struct FormatHtmlComment;

impl FormatRule<SourceComment<HtmlLanguage>> for FormatHtmlComment {
    type Context = HtmlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<HtmlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        write!(f, [comment.piece().as_piece()])
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct HtmlCommentStyle;

impl CommentStyle for HtmlCommentStyle {
    type Language = HtmlLanguage;

    fn is_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, ..)| key == category!("format"))
    }

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<HtmlLanguage>) -> CommentKind {
        CommentKind::Block
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        CommentPlacement::Default(comment)
    }
}
