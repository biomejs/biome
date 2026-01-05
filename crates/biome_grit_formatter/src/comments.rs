use crate::GritFormatContext;
use biome_diagnostics_categories::category;

use biome_formatter::comments::{CommentKind, CommentPlacement, DecoratedComment};
use biome_formatter::{
    FormatResult, FormatRule,
    comments::{CommentStyle, Comments, SourceComment, is_doc_comment},
    prelude::*,
    prelude::{Formatter, align, format_once, hard_line_break, text},
    write,
};
use biome_grit_syntax::{GritLanguage, GritRoot};
use biome_rowan::{AstNode, TextLen, TextSize};
use biome_suppression::{SuppressionKind, parse_suppression_comment};

pub type GritComments = Comments<GritLanguage>;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct GritCommentStyle;

impl CommentStyle for GritCommentStyle {
    type Language = GritLanguage;

    fn is_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .filter(|suppression| suppression.kind == SuppressionKind::Classic)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, ..)| key == category!("format"))
    }

    fn is_global_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .filter(|suppression| suppression.kind == SuppressionKind::All)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, ..)| key == category!("format"))
    }

    fn get_comment_kind(
        _comment: &biome_rowan::SyntaxTriviaPieceComments<Self::Language>,
    ) -> CommentKind {
        CommentKind::Line
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        handle_global_suppression(comment)
    }
}

#[derive(Default)]
pub struct FormatGritLeadingComment;

impl FormatRule<SourceComment<GritLanguage>> for FormatGritLeadingComment {
    type Context = GritFormatContext;
    // Copied and pasted this from the css formatter, not sure how much this needs to change.
    fn fmt(
        &self,
        comment: &SourceComment<GritLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        if is_doc_comment(comment.piece()) {
            let mut source_offset = comment.piece().text_range().start();

            let mut lines = comment.piece().text().lines();

            // SAFETY: Safe, `is_doc_comment` only returns `true` for multiline comments
            let first_line = lines.next().unwrap();
            write!(f, [text(first_line.trim_end(), source_offset)])?;

            source_offset += first_line.text_len();

            // Indent the remaining lines by one space so that all `*` are aligned.
            write!(
                f,
                [align(
                    1,
                    &format_once(|f| {
                        for line in lines {
                            write!(f, [hard_line_break(), text(line.trim(), source_offset)])?;

                            source_offset += line.text_len();
                        }

                        Ok(())
                    })
                )]
            )
        } else {
            write!(f, [comment.piece().as_piece()])
        }
    }
}

fn handle_global_suppression(
    comment: DecoratedComment<GritLanguage>,
) -> CommentPlacement<GritLanguage> {
    let node = comment.enclosing_node();

    if node.text_range_with_trivia().start() == TextSize::from(0) {
        let has_global_suppression = node.first_leading_trivia().is_some_and(|trivia| {
            trivia
                .pieces()
                .filter(|piece| piece.is_comments())
                .any(|piece| GritCommentStyle::is_global_suppression(piece.text()))
        });
        let root = node.ancestors().find_map(GritRoot::cast);
        if let Some(root) = root
            && has_global_suppression
        {
            return CommentPlacement::leading(root.syntax().clone(), comment);
        }
    }

    CommentPlacement::Default(comment)
}
