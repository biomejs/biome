use biome_diagnostics::category;
use biome_formatter::comments::{
    CommentKind, CommentPlacement, CommentStyle, Comments, DecoratedComment, SourceComment,
    is_alignable_comment,
};
use biome_formatter::formatter::Formatter;
use biome_formatter::{FormatResult, FormatRule, write};
use biome_rowan::SyntaxTriviaPieceComments;
use biome_suppression::parse_suppression_comment;
use biome_yaml_syntax::{TextLen, YamlLanguage};

use crate::prelude::*;

pub type YamlComments = Comments<YamlLanguage>;

#[derive(Default)]
pub struct FormatYamlLeadingComment;

impl FormatRule<SourceComment<YamlLanguage>> for FormatYamlLeadingComment {
    type Context = YamlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<YamlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        write!(f, [comment.piece().as_piece()])
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct YamlCommentStyle;

impl CommentStyle for YamlCommentStyle {
    type Language = YamlLanguage;

    fn is_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, ..)| key == category!("format"))
    }

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        CommentKind::Line
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        CommentPlacement::Default(comment)
    }
}
