use biome_formatter::comments::{CommentStyle, Comments};
use biome_grit_syntax::GritLanguage;

pub type GritComments = Comments<GritLanguage>;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct GritCommentStyle;

impl CommentStyle for GritCommentStyle {
    type Language = GritLanguage;

    fn is_suppression(_text: &str) -> bool {
        false
    }

    fn get_comment_kind(
        comment: &biome_rowan::SyntaxTriviaPieceComments<Self::Language>,
    ) -> biome_formatter::comments::CommentKind {
        todo!()
    }

    fn place_comment(
        &self,
        comment: biome_formatter::comments::DecoratedComment<Self::Language>,
    ) -> biome_formatter::comments::CommentPlacement<Self::Language> {
        biome_formatter::comments::CommentPlacement::Default(comment)
    }
}
