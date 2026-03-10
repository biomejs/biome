use crate::HtmlFormatter;
use crate::comments::HtmlCommentStyle;
use crate::context::HtmlFormatContext;
use biome_formatter::comments::{CommentKind, CommentStyle, SourceComment};
use biome_formatter::format_element::tag::VerbatimKind;
use biome_formatter::formatter::Formatter;
use biome_formatter::prelude::{
    Tag, empty_line, expand_parent, format_with, hard_line_break, line_suffix,
    should_nestle_adjacent_doc_comments, space, text,
};

use biome_formatter::{
    Buffer, CstFormatContext, Format, FormatContext, FormatElement, FormatRefWithRule,
    FormatResult, LINE_TERMINATORS, normalize_newlines,
};
use biome_html_syntax::{HtmlLanguage, HtmlSyntaxNode};
use biome_rowan::{Direction, SyntaxElement, TextRange};

/// "Formats" a node according to its original formatting in the source text. Being able to format
/// a node "as is" is useful if a node contains syntax errors. Formatting a node with syntax errors
/// has the risk that Biome misinterprets the structure of the code and formatting it could
/// "mess up" the developers, yet incomplete, work or accidentally introduce new syntax errors.
///
/// You may be inclined to call `node.text` directly. However, using `text` doesn't track the nodes
/// nor its children source mapping information, resulting in incorrect source maps for this subtree.
///
/// These nodes and tokens get tracked as [VerbatimKind::Verbatim], useful to understand
/// if these nodes still need to have their own implementation.
pub fn format_html_verbatim_node(node: &HtmlSyntaxNode) -> FormatHtmlVerbatimNode<'_> {
    FormatHtmlVerbatimNode {
        node,
        kind: VerbatimKind::Verbatim {
            length: node.text_range_with_trivia().len(),
        },
        format_comments: true,
    }
}

/// "Formats" a node according to its original formatting in the source text. It's functionally equal to
/// [`format_html_verbatim_node`], but it doesn't track the node as [VerbatimKind::Verbatim].
pub fn format_verbatim_skipped(node: &HtmlSyntaxNode) -> FormatHtmlVerbatimNode<'_> {
    FormatHtmlVerbatimNode {
        node,
        kind: VerbatimKind::Skipped,
        format_comments: true,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FormatHtmlVerbatimNode<'node> {
    node: &'node HtmlSyntaxNode,
    kind: VerbatimKind,
    format_comments: bool,
}

impl Format<HtmlFormatContext> for FormatHtmlVerbatimNode<'_> {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        for element in self.node.descendants_with_tokens(Direction::Next) {
            match element {
                SyntaxElement::Token(token) => f.state_mut().track_token(&token),
                SyntaxElement::Node(node) => {
                    let comments = f.context().comments();
                    comments.mark_suppression_checked(&node);

                    for comment in comments.leading_dangling_trailing_comments(&node) {
                        comment.mark_formatted();
                    }
                }
            }
        }

        // The trimmed range of a node is its range without any of its leading or trailing trivia.
        // Except for nodes that used to be parenthesized, the range than covers the source from the
        // `(` to the `)` (the trimmed range of the parenthesized expression, not the inner expression)
        let trimmed_source_range = f.context().source_map().map_or_else(
            || self.node.text_trimmed_range(),
            |source_map| source_map.trimmed_source_range(self.node),
        );

        f.write_element(FormatElement::Tag(Tag::StartVerbatim(self.kind)))?;

        fn source_range<Context>(f: &Formatter<Context>, range: TextRange) -> TextRange
        where
            Context: CstFormatContext,
        {
            f.context()
                .source_map()
                .map_or_else(|| range, |source_map| source_map.source_range(range))
        }

        // Format all leading comments that are outside of the node's source range.
        if self.format_comments {
            let comments = f.context().comments().clone();
            let leading_comments = comments.leading_comments(self.node);

            let outside_trimmed_range = leading_comments.partition_point(|comment| {
                comment.piece().text_range().end() <= trimmed_source_range.start()
            });

            let (outside_trimmed_range, in_trimmed_range) =
                leading_comments.split_at(outside_trimmed_range);

            biome_formatter::write!(f, [FormatLeadingCommentsSlice(outside_trimmed_range)])?;

            for comment in in_trimmed_range {
                comment.mark_formatted();
            }
        }

        // Find the first skipped token trivia, if any, and include it in the verbatim range because
        // the comments only format **up to** but not including skipped token trivia.
        let start_source = self
            .node
            .first_leading_trivia()
            .into_iter()
            .flat_map(|trivia| trivia.pieces())
            .filter(|trivia| trivia.is_skipped())
            .map(|trivia| source_range(f, trivia.text_range()).start())
            .take_while(|start| *start < trimmed_source_range.start())
            .next()
            .unwrap_or_else(|| trimmed_source_range.start());

        let original_source = f.context().source_map().map_or_else(
            || self.node.text_trimmed().to_string(),
            |source_map| {
                source_map
                    .source()
                    .text_slice(trimmed_source_range.cover_offset(start_source))
                    .to_string()
            },
        );

        text(
            &normalize_newlines(&original_source, LINE_TERMINATORS),
            self.node.text_trimmed_range().start(),
        )
        .fmt(f)?;

        for comment in f.context().comments().dangling_comments(self.node) {
            comment.mark_formatted();
        }

        // Format all trailing comments that are outside of the trimmed range.
        if self.format_comments {
            let comments = f.context().comments().clone();

            let trailing_comments = comments.trailing_comments(self.node);

            let outside_trimmed_range_start = trailing_comments.partition_point(|comment| {
                source_range(f, comment.piece().text_range()).end() <= trimmed_source_range.end()
            });

            let (in_trimmed_range, outside_trimmed_range) =
                trailing_comments.split_at(outside_trimmed_range_start);

            for comment in in_trimmed_range {
                comment.mark_formatted();
            }

            biome_formatter::write!(
                f,
                [FormatLHtmlTrailingComments::Comments(outside_trimmed_range)]
            )?;
        }

        f.write_element(FormatElement::Tag(Tag::EndVerbatim))
    }
}

/// Formats bogus nodes. The difference between this method  and `format_verbatim` is that this method
/// doesn't track nodes/tokens as [VerbatimKind::Verbatim]. They are just printed as they are.
pub fn format_bogus_node(node: &HtmlSyntaxNode) -> FormatHtmlVerbatimNode<'_> {
    FormatHtmlVerbatimNode {
        node,
        kind: VerbatimKind::Bogus,
        format_comments: true,
    }
}

/// Format a node having formatter suppression comment applied to it
pub fn format_suppressed_node(node: &HtmlSyntaxNode) -> FormatHtmlVerbatimNode<'_> {
    FormatHtmlVerbatimNode {
        node,
        kind: VerbatimKind::Suppressed,
        format_comments: true,
    }
}

/// Formats the leading comments of `node`
pub const fn format_html_leading_comments(node: &HtmlSyntaxNode) -> FormatHtmlLeadingComments<'_> {
    FormatHtmlLeadingComments {
        node,
        is_block_element: false,
    }
}

/// Formats the leading comments of `node`, treating it as a block element.
/// For block elements, comments with no line break after them will still get a hard line break.
pub const fn format_html_leading_comments_for_block(
    node: &HtmlSyntaxNode,
) -> FormatHtmlLeadingComments<'_> {
    FormatHtmlLeadingComments {
        node,
        is_block_element: true,
    }
}

/// Formats the leading comments of a node.
#[derive(Debug, Copy, Clone)]
pub struct FormatHtmlLeadingComments<'a> {
    node: &'a HtmlSyntaxNode,
    /// Whether the node is a block element. If true, comments with no line break after
    /// them will still get a hard line break instead of a space.
    is_block_element: bool,
}

impl<'a> Format<HtmlFormatContext> for FormatHtmlLeadingComments<'a> {
    fn fmt(&self, f: &mut HtmlFormatter) -> FormatResult<()> {
        let comments = f.context().comments().clone();
        let leading_comments = comments.leading_comments(self.node);
        format_leading_comments_impl(leading_comments, self.is_block_element, f)
    }
}

/// Formats a slice of leading comments (used for verbatim node formatting).
#[derive(Debug, Copy, Clone)]
pub struct FormatLeadingCommentsSlice<'a>(&'a [SourceComment<HtmlLanguage>]);

impl<'a> Format<HtmlFormatContext> for FormatLeadingCommentsSlice<'a> {
    fn fmt(&self, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_leading_comments_impl(self.0, false, f)
    }
}

/// Shared implementation for formatting leading comments.
fn format_leading_comments_impl(
    leading_comments: &[SourceComment<HtmlLanguage>],
    is_block_element: bool,
    f: &mut HtmlFormatter,
) -> FormatResult<()> {
    let leading_comments_iter = leading_comments.iter().peekable();
    for comment in leading_comments_iter {
        let format_comment = FormatRefWithRule::new(
            comment,
            <HtmlFormatContext as CstFormatContext>::CommentRule::default(),
        );
        biome_formatter::write!(f, [format_comment])?;

        // Check if this is a suppression comment
        let is_suppression = HtmlCommentStyle::is_suppression(comment.piece().text());

        match comment.kind() {
            CommentKind::Block | CommentKind::InlineBlock => {
                // HTML comments are block comments (<!-- ... -->)
                match comment.lines_after() {
                    0 => {
                        // No newline after comment in source
                        if is_suppression {
                            // Don't add trailing whitespace after suppression comments -
                            // the suppressed content should directly follow
                        } else if is_block_element {
                            // Block elements always get a line break after leading comments
                            biome_formatter::write!(f, [hard_line_break()])?;
                        } else {
                            // Inline elements get a space
                            biome_formatter::write!(f, [space()])?;
                        }
                    }
                    1 => {
                        biome_formatter::write!(f, [hard_line_break()])?;
                    }
                    _ => biome_formatter::write!(f, [empty_line()])?,
                }
            }

            CommentKind::Line => {
                // `//` line comments always require a hard line break after them because
                // everything after `//` to end-of-line is part of the comment. Using
                // `soft_line_break_or_space` would collapse the comment with the next
                // attribute onto a single line, turning the `>` into part of the comment.
                match comment.lines_after() {
                    0 => {}
                    _ => {
                        biome_formatter::write!(f, [hard_line_break()])?;
                    }
                }
            }
        }

        comment.mark_formatted()
    }

    Ok(())
}

/// Formats the leading comments of `node`
pub const fn format_html_trailing_comments(
    node: &HtmlSyntaxNode,
) -> FormatLHtmlTrailingComments<'_> {
    FormatLHtmlTrailingComments::Node(node)
}

/// Formats the leading comments of a node.
#[derive(Debug, Copy, Clone)]
pub enum FormatLHtmlTrailingComments<'a> {
    Node(&'a HtmlSyntaxNode),
    Comments(&'a [SourceComment<HtmlLanguage>]),
}

impl<'a> Format<HtmlFormatContext> for FormatLHtmlTrailingComments<'a> {
    fn fmt(&self, f: &mut HtmlFormatter) -> FormatResult<()> {
        let comments = f.context().comments().clone();
        let trailing_comments = match self {
            FormatLHtmlTrailingComments::Node(node) => comments.trailing_comments(node),
            FormatLHtmlTrailingComments::Comments(comments) => comments,
        };
        let mut total_lines_before = 0;
        let mut previous_comment: Option<
            &SourceComment<<HtmlFormatContext as CstFormatContext>::Language>,
        > = None;

        for comment in trailing_comments {
            total_lines_before += comment.lines_before();

            let format_comment = FormatRefWithRule::new(
                comment,
                <HtmlFormatContext as CstFormatContext>::CommentRule::default(),
            );

            let should_nestle = previous_comment.is_some_and(|previous_comment| {
                should_nestle_adjacent_doc_comments(previous_comment, comment)
            });

            // This allows comments at the end of nested structures:
            // {
            //   x: 1,
            //   y: 2
            //   // A comment
            // }
            // Those kinds of comments are almost always leading comments, but
            // here it doesn't go "outside" the block and turns it into a
            // trailing comment for `2`. We can simulate the above by checking
            // if this a comment on its own line; normal trailing comments are
            // always at the end of another expression.
            if total_lines_before > 0 {
                biome_formatter::write!(
                    f,
                    [
                        line_suffix(&format_with(|f| {
                            match comment.lines_before() {
                                _ if should_nestle => {}
                                0 => {
                                    // If the comment is immediately following a block-like comment,
                                    // then it can stay on the same line with just a space between.
                                    // Otherwise, it gets a hard break.
                                    //
                                    //   /** hello */ // hi
                                    //   /**
                                    //    * docs
                                    //   */ /* still on the same line */
                                    if previous_comment.is_some_and(|previous_comment| {
                                        previous_comment.kind().is_line()
                                    }) {
                                        biome_formatter::write!(f, [hard_line_break()])?;
                                    } else {
                                        biome_formatter::write!(f, [space()])?;
                                    }
                                }
                                1 => biome_formatter::write!(f, [hard_line_break()])?,
                                _ => biome_formatter::write!(f, [empty_line()])?,
                            };

                            biome_formatter::write!(f, [format_comment])
                        })),
                        expand_parent()
                    ]
                )?;
            } else {
                let content = format_with(|f| biome_formatter::write!(f, [format_comment]));
                if comment.kind().is_line() {
                    biome_formatter::write!(f, [line_suffix(&content), expand_parent()])?;
                } else {
                    biome_formatter::write!(f, [content])?;
                }
            }

            previous_comment = Some(comment);
            comment.mark_formatted();
        }

        Ok(())
    }
}
