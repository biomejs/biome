use crate::prelude::*;
use biome_formatter::comments::{
    CommentKind, CommentPlacement, CommentStyle, Comments, DecoratedComment, SourceComment,
    is_doc_comment,
};
use biome_formatter::formatter::Formatter;
use biome_formatter::{FormatResult, FormatRule, write};
use biome_graphql_syntax::{GraphqlLanguage, GraphqlSyntaxKind, TextLen};
use biome_rowan::SyntaxTriviaPieceComments;

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
            write!(f, [text(first_line.trim_end(), Some(source_offset))])?;

            source_offset += first_line.text_len();

            // Indent the remaining lines by one space so that all `*` are aligned.
            write!(
                f,
                [align(
                    " ",
                    &format_once(|f| {
                        for line in lines {
                            write!(
                                f,
                                [hard_line_break(), text(line.trim(), Some(source_offset))]
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

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        CommentKind::Line
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        handle_comment_before_braces(comment)
    }
}

/// Move a comment that sits on its own line in front of a `{` into the braces.
///
/// Such a comment leads the braced node, and the definitions that own one write
/// it after a space, so it is printed on the line of the name:
///
/// ```graphql
/// type T # comment
/// {
///     a: a
/// }
/// ```
///
/// That is not where the comment was, and reformatting the result reads it as a
/// trailing comment of the name instead, so the output keeps changing from one
/// run to the next. Attaching it to the first member puts it where it was
/// written, inside the braces, and the placement no longer depends on how the
/// previous run laid the file out.
fn handle_comment_before_braces(
    comment: DecoratedComment<GraphqlLanguage>,
) -> CommentPlacement<GraphqlLanguage> {
    if !comment.text_position().is_own_line() {
        return CommentPlacement::Default(comment);
    }

    let Some(following) = comment.following_node() else {
        return CommentPlacement::Default(comment);
    };

    // The braces have to belong to the node the comment is enclosed by. A
    // comment standing between two definitions is enclosed by the document
    // instead, and leads the next definition at the outer indentation.
    if following.parent().as_ref() != Some(comment.enclosing_node()) {
        return CommentPlacement::Default(comment);
    }

    // Only the type system definitions. A selection set has the same problem,
    // but Prettier does not reach a fixed point on it either, so there is no
    // output to agree with yet.
    if !matches!(
        following.kind(),
        GraphqlSyntaxKind::GRAPHQL_FIELDS_DEFINITION
            | GraphqlSyntaxKind::GRAPHQL_INPUT_FIELDS_DEFINITION
            | GraphqlSyntaxKind::GRAPHQL_ENUM_VALUES_DEFINITION
            | GraphqlSyntaxKind::GRAPHQL_ROOT_OPERATION_TYPES
    ) {
        return CommentPlacement::Default(comment);
    }

    // The braced node holds a single list; its first element is the member the
    // comment belongs in front of. An empty list cannot be written in GraphQL,
    // so there is no member-less case to handle here.
    match following
        .children()
        .next()
        .and_then(|list| list.children().next())
    {
        Some(first_member) => CommentPlacement::leading(first_member, comment),
        None => CommentPlacement::Default(comment),
    }
}
