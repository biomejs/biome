use biome_diagnostics_categories::category;
use biome_formatter::{
    Buffer, Format, FormatResult, FormatRule,
    comments::{
        CommentKind, CommentPlacement, CommentStyle, CommentSuppressionTarget, DecoratedComment,
        SourceComment,
    },
    prelude::{
        Formatter, dedent_to_root, empty_line, format_with, hard_line_break, located_token_text,
    },
};
use biome_markdown_syntax::{
    AnyMdBlock, MarkdownLanguage, MarkdownSyntaxKind, MdFencedCodeBlock, MdNewline, MdQuote, MdRoot,
};
use biome_rowan::{AstNode, AstNodeList, SyntaxTriviaPieceComments};
use biome_suppression::{SuppressionKind, parse_suppression_comment_with_line_prefix};

use crate::MarkdownFormatContext;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct MarkdownCommentStyle;

impl CommentStyle for MarkdownCommentStyle {
    type Language = MarkdownLanguage;

    fn suppression_kind(
        &self,
        comment: &DecoratedComment<Self::Language>,
    ) -> Option<SuppressionKind> {
        let token = comment.piece().as_piece().token();
        if token
            .ancestors()
            .any(|node| MdFencedCodeBlock::can_cast(node.kind()))
        {
            return None;
        }
        let strip_line_prefix: fn(&str) -> &str =
            if token.ancestors().any(|node| MdQuote::can_cast(node.kind())) {
                strip_quote_prefix
            } else {
                |line| line
            };
        let mut kind = None;
        for suppression in
            parse_suppression_comment_with_line_prefix(comment.piece().text(), strip_line_prefix)
                .filter_map(Result::ok)
        {
            if !suppression
                .categories
                .iter()
                .any(|(key, ..)| *key == category!("format"))
            {
                continue;
            }
            match suppression.kind {
                SuppressionKind::Classic => kind = Some(SuppressionKind::Classic),
                SuppressionKind::All => {
                    let Some(root) = comment.enclosing_node().ancestors().find_map(MdRoot::cast)
                    else {
                        continue;
                    };
                    // Markdown keeps blank lines as nodes; frontmatter is outside the body list.
                    let first_content_start =
                        root.value().iter().find(is_suppressible_block).map_or_else(
                            || {
                                root.eof_token().map_or_else(
                                    |_| root.range().end(),
                                    |token| token.text_trimmed_range().start(),
                                )
                            },
                            |block| block.range().start(),
                        );
                    if comment.piece().text_range().end() <= first_content_start {
                        return Some(SuppressionKind::All);
                    }
                }
                SuppressionKind::RangeStart | SuppressionKind::RangeEnd => {}
            }
        }
        kind
    }

    /// Targets the next block, skipping structural newlines, indentation, and quote prefixes.
    fn suppression_target(
        &self,
        comment: &DecoratedComment<Self::Language>,
    ) -> CommentSuppressionTarget<Self::Language> {
        let Some(root) = comment.enclosing_node().ancestors().find_map(MdRoot::cast) else {
            return CommentSuppressionTarget::CommentPlacement;
        };
        let comment_end = comment.piece().text_range().end();
        root.syntax()
            .descendants()
            .filter_map(AnyMdBlock::cast)
            .find(|block| block.range().start() >= comment_end && is_suppressible_block(block))
            .map_or(CommentSuppressionTarget::CommentPlacement, |block| {
                CommentSuppressionTarget::Node(block.into_syntax())
            })
    }

    fn get_comment_kind(comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        if has_following_newline(comment) {
            CommentKind::Line
        } else if comment.has_newline() {
            CommentKind::Block
        } else {
            CommentKind::InlineBlock
        }
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        let token = comment.piece().as_piece().token();
        if token.kind() == MarkdownSyntaxKind::MD_HTML_LITERAL
            && let Some(parent) = token.parent()
        {
            return CommentPlacement::leading(parent, comment);
        }
        if token.kind() == MarkdownSyntaxKind::NEWLINE
            && let Some(newline) = token.parent().and_then(MdNewline::cast)
        {
            return CommentPlacement::leading(newline.into_syntax(), comment);
        }
        CommentPlacement::Default(comment)
    }
}

fn has_following_newline(comment: &SyntaxTriviaPieceComments<MarkdownLanguage>) -> bool {
    let range = comment.text_range();
    let token = comment.as_piece().token();
    token
        .leading_trivia()
        .pieces()
        .chain(token.trailing_trivia().pieces())
        .skip_while(|piece| piece.text_range() != range)
        .skip(1)
        .find(|piece| !piece.is_whitespace())
        .map_or_else(
            || token.kind() == MarkdownSyntaxKind::NEWLINE,
            |piece| piece.is_newline(),
        )
}

fn is_suppressible_block(block: &AnyMdBlock) -> bool {
    !block.is_newline()
        && !block.is_continuation_indent()
        && block.as_md_quote_prefix().is_none()
        && !block.is_html_comment()
}

fn strip_quote_prefix(mut line: &str) -> &str {
    loop {
        let trimmed = line.trim_start_matches([' ', '\t']);
        let Some(rest) = trimmed.strip_prefix('>') else {
            return line;
        };
        line = rest.strip_prefix([' ', '\t']).unwrap_or(rest);
    }
}

#[derive(Default)]
pub struct FormatMarkdownLeadingComment;

impl FormatRule<SourceComment<MarkdownLanguage>> for FormatMarkdownLeadingComment {
    type Context = MarkdownFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<MarkdownLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        biome_formatter::write!(f, [comment.piece().as_piece()])
    }
}

/// Preserves the source gaps around comments because Markdown can store spaces
/// and line endings as syntax tokens, outside the comment's trivia.
pub(crate) struct FormatMarkdownLeadingComments<'a>(
    pub(crate) &'a [SourceComment<MarkdownLanguage>],
);

impl Format<MarkdownFormatContext> for FormatMarkdownLeadingComments<'_> {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        for comment in self.0 {
            let token = comment.piece().as_piece().token();
            let range = comment.piece().text_range();
            let leading = token.leading_trivia();
            let before = leading
                .pieces()
                .take_while(|piece| piece.text_range().end() <= range.start());
            if before.clone().all(|piece| piece.is_whitespace())
                && token
                    .prev_token()
                    .is_none_or(|previous| previous.text_trimmed().ends_with(['\n', '\r']))
            {
                biome_formatter::write!(
                    f,
                    [dedent_to_root(&format_with(|f| {
                        biome_formatter::write!(f, [hard_line_break()])?;
                        for piece in before.clone() {
                            biome_formatter::write!(
                                f,
                                [located_token_text(&token, piece.text_range())]
                            )?;
                        }
                        biome_formatter::write!(f, [comment.piece().as_piece()])
                    }))]
                )?;
            } else {
                biome_formatter::write!(f, [comment.piece().as_piece()])?;
            }

            for piece in token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
                .skip_while(|piece| piece.text_range().start() < range.end())
                .take_while(|piece| piece.is_whitespace())
            {
                biome_formatter::write!(f, [located_token_text(&token, piece.text_range())])?;
            }
            match comment.lines_after() {
                0 => {}
                1 => biome_formatter::write!(f, [hard_line_break()])?,
                _ => biome_formatter::write!(f, [empty_line()])?,
            }
            comment.mark_formatted();
        }
        Ok(())
    }
}
