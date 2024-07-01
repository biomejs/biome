use crate::prelude::*;
use biome_diagnostics::category;
use biome_formatter::comments::{
    is_doc_comment, CommentKind, CommentStyle, Comments, SourceComment,
};
use biome_formatter::formatter::Formatter;
use biome_formatter::{write, FormatResult, FormatRule};
use biome_graphql_syntax::{GraphqlLanguage, TextLen};
use biome_rowan::SyntaxTriviaPieceComments;
use biome_suppression::parse_suppression_comment;

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
            write!(f, [dynamic_text(first_line.trim_end(), source_offset)])?;

            source_offset += first_line.text_len();

            // Indent the remaining lines by one space so that all `*` are aligned.
            write!(
                f,
                [align(
                    1,
                    &format_once(|f| {
                        for line in lines {
                            write!(
                                f,
                                [hard_line_break(), dynamic_text(line.trim(), source_offset)]
                            )?;

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
            .flat_map(|suppression| suppression.categories)
            .any(|(key, _)| key == category!("format"))
    }

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        CommentKind::Line
    }
}
