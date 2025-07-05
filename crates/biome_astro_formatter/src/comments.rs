use biome_formatter::comments::{CommentKind, CommentPlacement, CommentStyle, DecoratedComment};
use biome_astro_syntax::{AstroLanguage, AstroSyntaxKind, AstroSyntaxToken};
use biome_rowan::{SyntaxTriviaPieceComments, TextRange};

pub struct AstroCommentStyle;

impl CommentStyle for AstroCommentStyle {
    type Language = AstroLanguage;

    fn get_comment_kind(&self, comment: &AstroSyntaxToken) -> CommentKind {
        match comment.kind() {
            AstroSyntaxKind::ASTRO_COMMENT => {
                let text = comment.text();
                if text.starts_with("<!--") && text.ends_with("-->") {
                    CommentKind::Block
                } else {
                    CommentKind::Line
                }
            }
            _ => CommentKind::Block,
        }
    }

    fn is_suppression(&self, comment: &AstroSyntaxToken) -> bool {
        let text = comment.text();
        // Check for prettier-ignore comments in Astro
        text.contains("prettier-ignore") || text.contains("biome-ignore")
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        CommentPlacement::Default(comment)
    }
}