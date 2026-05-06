use crate::prelude::*;
use biome_diagnostics::category;
use biome_formatter::comments::{
    CommentKind, CommentPlacement, CommentStyle, CommentTextPosition, Comments, DecoratedComment,
    SourceComment, is_doc_comment,
};
use biome_formatter::formatter::Formatter;
use biome_formatter::{FormatResult, FormatRule, write};
use biome_graphql_syntax::{GraphqlLanguage, GraphqlRoot, TextLen};
use biome_rowan::{SyntaxTriviaPieceComments, TextSize};
use biome_suppression::{SuppressionKind, parse_suppression_comment};

pub type GraphqlComments = Comments<GraphqlLanguage>;

#[derive(Default)]
pub struct FormatGraphqlLeadingComment;

impl FormatRule<SourceComment<GraphqlLanguage>> for FormatGraphqlLeadingComment {
    type Context = GraphqlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<GraphqlLanguage>,
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

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct GraphqlCommentStyle;

impl CommentStyle for GraphqlCommentStyle {
    type Language = GraphqlLanguage;

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

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        CommentKind::Line
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        match comment.text_position() {
            CommentTextPosition::EndOfLine => handle_global_suppression(comment),
            CommentTextPosition::OwnLine => handle_global_suppression(comment),
            CommentTextPosition::SameLine => CommentPlacement::Default(comment),
        }
    }
}

fn handle_global_suppression(
    comment: DecoratedComment<GraphqlLanguage>,
) -> CommentPlacement<GraphqlLanguage> {
    let node = comment.enclosing_node();

    if node.text_range_with_trivia().start() == TextSize::from(0) {
        let has_global_suppression = node.first_leading_trivia().is_some_and(|trivia| {
            trivia
                .pieces()
                .filter(|piece| piece.is_comments())
                .any(|piece| GraphqlCommentStyle::is_global_suppression(piece.text()))
        });
        let root = node.ancestors().find_map(GraphqlRoot::cast);
        if let Some(root) = root
            && has_global_suppression
        {
            return CommentPlacement::leading(root.syntax().clone(), comment);
        }
    }
    CommentPlacement::Default(comment)
}
