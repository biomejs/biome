//! Provides builders for comments and skipped token trivia.

use crate::comments::is_doc_comment;
use crate::format_element::tag::VerbatimKind;
use crate::prelude::*;
use crate::{
    Argument, Arguments, CstFormatContext, FormatRefWithRule, GroupId, SourceComment, TextRange,
    comments::{CommentKind, CommentStyle},
    write,
};
use biome_rowan::{Language, SyntaxNode, SyntaxToken, TextSize};
#[cfg(debug_assertions)]
use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Sub;

/// Returns true if:
/// - `next_comment` is Some, and
/// - both comments are documentation comments, and
/// - both comments are multiline, and
/// - the two comments are immediately adjacent to each other, with no characters between them.
///
/// In this case, the comments are considered "nestled" - a pattern that JSDoc uses to represent
/// overloaded types, which get merged together to create the final type for the subject. The
/// comments must be kept immediately adjacent after formatting to preserve this behavior.
///
/// There isn't much documentation about this behavior, but it is mentioned on the JSDoc repo
/// for documentation: <https://github.com/jsdoc/jsdoc.github.io/issues/40>. Prettier also
/// implements the same behavior: <https://github.com/prettier/prettier/pull/13445/files#diff-3d5eaa2a1593372823589e6e55e7ca905f7c64203ecada0aa4b3b0cdddd5c3ddR160-R178>
pub fn should_nestle_adjacent_doc_comments<L: Language>(
    first_comment: &SourceComment<L>,
    second_comment: &SourceComment<L>,
) -> bool {
    let first = first_comment.piece();
    let second = second_comment.piece();

    first.has_newline()
        && second.has_newline()
        && (second.text_range().start()).sub(first.text_range().end()) == TextSize::from(0)
        && is_doc_comment(first)
        && is_doc_comment(second)
}

/// Formats the leading comments of `node`
pub const fn format_leading_comments<L: Language>(
    node: &SyntaxNode<L>,
) -> FormatLeadingComments<'_, L> {
    FormatLeadingComments {
        source: CommentSource::Node(node),
    }
}

/// Formats the leading comments from an explicit comment slice.
pub const fn format_leading_comments_from_slice<L: Language>(
    comments: &[SourceComment<L>],
) -> FormatLeadingComments<'_, L> {
    FormatLeadingComments {
        source: CommentSource::Comments(comments),
    }
}

/// Formats leading comments.
#[derive(Debug, Copy, Clone)]
pub struct FormatLeadingComments<'a, L: Language> {
    source: CommentSource<'a, L>,
}

/// Formats leading comments followed by caller-provided content.
#[derive(Debug, Copy, Clone)]
pub struct FormatLeadingCommentsWithContent<'a, L: Language, Context, Content> {
    source: CommentSource<'a, L>,
    content: Content,
    layout: LeadingCommentLayout,
    context: PhantomData<fn() -> Context>,
}

#[derive(Debug, Copy, Clone)]
enum CommentSource<'a, L: Language> {
    Node(&'a SyntaxNode<L>),
    Comments(&'a [SourceComment<L>]),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum LeadingCommentLayout {
    /// Writes content after the final leading comment.
    ///
    /// Example output: `/* note */ key: value`.
    FollowingContent,

    /// Writes content after the final leading comment and indents it on break.
    ///
    /// Example output: `/* note */ (value)` or `/* note */\n\t(value)`.
    IndentedFollowingContent,
}

impl<'a, L: Language> FormatLeadingComments<'a, L> {
    /// Formats leading comments before the following content.
    ///
    /// Example output: `/* note */ key: value`.
    pub fn with_following_content<Context, Content>(
        self,
        content: Content,
    ) -> FormatLeadingCommentsWithContent<'a, L, Context, Content>
    where
        Context: CstFormatContext<Language = L>,
        Content: Fn(&mut Formatter<Context>) -> FormatResult<()>,
    {
        self.with_content_layout(content, LeadingCommentLayout::FollowingContent)
    }

    /// Formats leading comments before indented following content.
    ///
    /// Example output: `/* note */ (value)` or `/* note */\n\t(value)`.
    pub fn with_indented_following_content<Context, Content>(
        self,
        content: Content,
    ) -> FormatLeadingCommentsWithContent<'a, L, Context, Content>
    where
        Context: CstFormatContext<Language = L>,
        Content: Fn(&mut Formatter<Context>) -> FormatResult<()>,
    {
        self.with_content_layout(content, LeadingCommentLayout::IndentedFollowingContent)
    }

    fn with_content_layout<Context, Content>(
        self,
        content: Content,
        layout: LeadingCommentLayout,
    ) -> FormatLeadingCommentsWithContent<'a, L, Context, Content>
    where
        Context: CstFormatContext<Language = L>,
        Content: Fn(&mut Formatter<Context>) -> FormatResult<()>,
    {
        FormatLeadingCommentsWithContent {
            source: self.source,
            content,
            layout,
            context: PhantomData,
        }
    }
}

impl<Context> Format<Context> for FormatLeadingComments<'_, Context::Language>
where
    Context: CstFormatContext,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let comments = f.context().comments().clone();

        let leading_comments = match self.source {
            CommentSource::Node(node) => comments.leading_comments(node),
            CommentSource::Comments(comments) => comments,
        };

        fmt_leading_comments_on_lines(leading_comments, f)
    }
}

impl<Context, Content> Format<Context>
    for FormatLeadingCommentsWithContent<'_, Context::Language, Context, Content>
where
    Context: CstFormatContext,
    Content: Fn(&mut Formatter<Context>) -> FormatResult<()>,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let comments = f.context().comments().clone();

        let leading_comments = match self.source {
            CommentSource::Node(node) => comments.leading_comments(node),
            CommentSource::Comments(comments) => comments,
        };

        self.layout.fmt(leading_comments, &self.content, f)
    }
}

impl LeadingCommentLayout {
    fn fmt<Context, Content>(
        self,
        leading_comments: &[SourceComment<Context::Language>],
        content: &Content,
        f: &mut Formatter<Context>,
    ) -> FormatResult<()>
    where
        Context: CstFormatContext,
        Content: Fn(&mut Formatter<Context>) -> FormatResult<()>,
    {
        if leading_comments.is_empty() {
            return content(f);
        }

        let mut leading_comments_iter = leading_comments.iter().peekable();

        while let Some(comment) = leading_comments_iter.next() {
            let is_last = leading_comments_iter.peek().is_none();
            let format_comment = FormatRefWithRule::new(comment, Context::CommentRule::default());
            write!(f, [format_comment])?;

            if is_last {
                self.fmt_content_after_last_comment(comment, content, f)?;
            } else {
                fmt_between_leading_comments(comment, f)?;
            }

            comment.mark_formatted();
        }

        Ok(())
    }

    fn fmt_content_after_last_comment<Context, Content>(
        self,
        comment: &SourceComment<Context::Language>,
        content: &Content,
        f: &mut Formatter<Context>,
    ) -> FormatResult<()>
    where
        Context: CstFormatContext,
        Content: Fn(&mut Formatter<Context>) -> FormatResult<()>,
    {
        match comment.kind() {
            CommentKind::Block | CommentKind::InlineBlock => match self {
                Self::FollowingContent => {
                    write!(f, [soft_line_break_or_space()])?;
                    content(f)
                }
                Self::IndentedFollowingContent => {
                    let indented_content = format_with(|f| {
                        write!(f, [soft_line_break_or_space()])?;
                        content(f)
                    });
                    write!(f, [indent(&indented_content)])
                }
            },
            CommentKind::Line => {
                write!(f, [hard_line_break()])?;
                content(f)
            }
        }
    }
}

/// Formats leading comments before the owning node content.
fn fmt_leading_comments_on_lines<Context>(
    leading_comments: &[SourceComment<Context::Language>],
    f: &mut Formatter<Context>,
) -> FormatResult<()>
where
    Context: CstFormatContext,
{
    let mut leading_comments_iter = leading_comments.iter().peekable();
    while let Some(comment) = leading_comments_iter.next() {
        let format_comment = FormatRefWithRule::new(comment, Context::CommentRule::default());
        write!(f, [format_comment])?;

        match comment.kind() {
            CommentKind::Block | CommentKind::InlineBlock => match comment.lines_after() {
                0 => {
                    let should_nestle = leading_comments_iter.peek().is_some_and(|next_comment| {
                        should_nestle_adjacent_doc_comments(comment, next_comment)
                    });

                    write!(f, [maybe_space(!should_nestle)])?;
                }
                1 => {
                    if comment.lines_before() == 0 {
                        write!(f, [soft_line_break_or_space()])?;
                    } else {
                        write!(f, [hard_line_break()])?;
                    }
                }
                _ => write!(f, [empty_line()])?,
            },
            CommentKind::Line => match comment.lines_after() {
                0 | 1 => write!(f, [hard_line_break()])?,
                _ => write!(f, [empty_line()])?,
            },
        }

        comment.mark_formatted()
    }

    Ok(())
}

fn fmt_between_leading_comments<Context>(
    comment: &SourceComment<Context::Language>,
    f: &mut Formatter<Context>,
) -> FormatResult<()>
where
    Context: CstFormatContext,
{
    match comment.kind() {
        CommentKind::Block | CommentKind::InlineBlock => match comment.lines_after() {
            0 | 1 => write!(f, [soft_line_break_or_space()]),
            _ => write!(f, [empty_line()]),
        },
        CommentKind::Line => write!(f, [hard_line_break()]),
    }
}

/// Formats the trailing comments of `node`.
pub const fn format_trailing_comments<L: Language>(
    node: &SyntaxNode<L>,
) -> FormatTrailingComments<'_, L> {
    FormatTrailingComments {
        source: CommentSource::Node(node),
    }
}

/// Formats the trailing comments from an explicit comment slice.
pub const fn format_trailing_comments_from_slice<L: Language>(
    comments: &[SourceComment<L>],
) -> FormatTrailingComments<'_, L> {
    FormatTrailingComments {
        source: CommentSource::Comments(comments),
    }
}

/// Formats trailing comments.
#[derive(Debug, Clone, Copy)]
pub struct FormatTrailingComments<'a, L: Language> {
    source: CommentSource<'a, L>,
}

impl<Context> Format<Context> for FormatTrailingComments<'_, Context::Language>
where
    Context: CstFormatContext,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let comments = f.context().comments().clone();
        let trailing_comments = match self.source {
            CommentSource::Node(node) => comments.trailing_comments(node),
            CommentSource::Comments(comments) => comments,
        };

        let mut total_lines_before = 0;
        let mut previous_comment: Option<&SourceComment<Context::Language>> = None;

        for comment in trailing_comments {
            total_lines_before += comment.lines_before();

            let format_comment = FormatRefWithRule::new(comment, Context::CommentRule::default());

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
                write!(
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
                                        write!(f, [hard_line_break()])?;
                                    } else {
                                        write!(f, [space()])?;
                                    }
                                }
                                1 => write!(f, [hard_line_break()])?,
                                _ => write!(f, [empty_line()])?,
                            };

                            write!(f, [format_comment])
                        })),
                        expand_parent()
                    ]
                )?;
            } else {
                let content =
                    format_with(|f| write!(f, [maybe_space(!should_nestle), format_comment]));
                if comment.kind().is_line() {
                    write!(f, [line_suffix(&content), expand_parent()])?;
                } else {
                    write!(f, [content])?;
                }
            }

            previous_comment = Some(comment);
            comment.mark_formatted();
        }

        Ok(())
    }
}

/// Formats the dangling comments of `node`.
pub const fn format_dangling_comments<L: Language>(
    node: &SyntaxNode<L>,
) -> FormatDanglingComments<'_, L> {
    FormatDanglingComments {
        source: CommentSource::Node(node),
        indent: DanglingIndentMode::None,
        layout: DanglingCommentLayout::Multiline,
    }
}

/// Formats the dangling comments from an explicit comment slice.
pub const fn format_dangling_comments_from_slice<L: Language>(
    comments: &[SourceComment<L>],
) -> FormatDanglingComments<'_, L> {
    FormatDanglingComments {
        source: CommentSource::Comments(comments),
        indent: DanglingIndentMode::None,
        layout: DanglingCommentLayout::Multiline,
    }
}

/// Formats one dangling comment.
pub const fn format_dangling_comment<L: Language>(
    comment: &SourceComment<L>,
) -> FormatDanglingComments<'_, L> {
    format_dangling_comments_from_slice(std::slice::from_ref(comment))
}

/// Formats dangling comments.
pub struct FormatDanglingComments<'a, L: Language> {
    source: CommentSource<'a, L>,
    indent: DanglingIndentMode,
    layout: DanglingCommentLayout,
}

#[derive(Copy, Clone, Debug)]
pub enum DanglingIndentMode {
    /// Writes every comment on its own line and indents them with a block indent.
    ///
    /// # Examples
    /// ```ignore
    /// [
    ///     /* comment */
    /// ]
    ///
    /// [
    ///     /* comment */
    ///     /* multiple */
    /// ]
    /// ```
    Block,

    /// Writes every comment on its own line and indents them with a soft line indent.
    /// Guarantees to write a line break if the last formatted comment is a [line](CommentKind::Line) comment.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// [/* comment */]
    ///
    /// [
    ///     /* comment */
    ///     /* other */
    /// ]
    ///
    /// [
    ///     // line
    /// ]
    /// ```
    Soft,

    /// Writes every comment on its own line.
    None,
}

impl<'a, L: Language> FormatDanglingComments<'a, L> {
    /// Indents the comments with a [block](DanglingIndentMode::Block) indent.
    pub fn with_block_indent(self) -> Self {
        self.with_indent_mode(DanglingIndentMode::Block)
    }

    /// Indents the comments with a [soft block](DanglingIndentMode::Soft) indent.
    pub fn with_soft_block_indent(self) -> Self {
        self.with_indent_mode(DanglingIndentMode::Soft)
    }

    fn with_indent_mode(mut self, mode: DanglingIndentMode) -> Self {
        self.indent = mode;
        self
    }

    fn with_layout(mut self, layout: DanglingCommentLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Keeps adjacent block comments on one line.
    ///
    /// Use for punctuation-owned groups like `(a: b, /* one */ /* two */)`.
    pub fn with_block_comments_on_line(self) -> Self {
        self.with_layout(DanglingCommentLayout::InlineComments)
    }
}

impl<Context> Format<Context> for FormatDanglingComments<'_, Context::Language>
where
    Context: CstFormatContext,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let comments = f.context().comments().clone();
        let dangling_comments = match self.source {
            CommentSource::Node(node) => comments.dangling_comments(node),
            CommentSource::Comments(comments) => comments,
        };

        if dangling_comments.is_empty() {
            return Ok(());
        }

        match self.layout {
            DanglingCommentLayout::InlineComments
                if can_keep_dangling_comments_inline(dangling_comments) =>
            {
                self.fmt_inline_dangling_comments(dangling_comments, f)
            }
            DanglingCommentLayout::Multiline | DanglingCommentLayout::InlineComments => {
                self.fmt_multiline_dangling_comments(dangling_comments, f)
            }
        }
    }
}

impl<'a, L: Language> FormatDanglingComments<'a, L> {
    /// Formats dangling comments with normal line breaks.
    ///
    /// Example output: `(\n\t/* one */\n\t/* two */\n)`.
    fn fmt_multiline_dangling_comments<Context>(
        &self,
        dangling_comments: &[SourceComment<Context::Language>],
        f: &mut Formatter<Context>,
    ) -> FormatResult<()>
    where
        Context: CstFormatContext<Language = L>,
    {
        let format_dangling_comments = format_with(|f| {
            let mut previous_comment: Option<&SourceComment<Context::Language>> = None;

            for comment in dangling_comments {
                let format_comment =
                    FormatRefWithRule::new(comment, Context::CommentRule::default());

                let should_nestle = previous_comment.is_some_and(|previous_comment| {
                    should_nestle_adjacent_doc_comments(previous_comment, comment)
                });

                write!(
                    f,
                    [
                        (previous_comment.is_some() && !should_nestle).then_some(hard_line_break()),
                        format_comment
                    ]
                )?;

                previous_comment = Some(comment);
                comment.mark_formatted();
            }

            if matches!(self.indent, DanglingIndentMode::Soft)
                && dangling_comments
                    .last()
                    .is_some_and(|comment| comment.kind().is_line())
            {
                write!(f, [hard_line_break()])?;
            }

            Ok(())
        });

        self.indent.fmt_content(&format_dangling_comments, f)
    }

    /// Formats dangling comments on one line.
    ///
    /// Example output: `(/* one */ /* two */)`.
    fn fmt_inline_dangling_comments<Context>(
        &self,
        dangling_comments: &[SourceComment<Context::Language>],
        f: &mut Formatter<Context>,
    ) -> FormatResult<()>
    where
        Context: CstFormatContext<Language = L>,
    {
        let format_dangling_comments = format_with(|f| {
            let mut is_first = true;

            for comment in dangling_comments {
                if is_first {
                    is_first = false;
                } else {
                    write!(f, [space()])?;
                }

                let format_comment =
                    FormatRefWithRule::new(comment, Context::CommentRule::default());
                write!(f, [format_comment])?;
                comment.mark_formatted();
            }

            Ok(())
        });

        self.indent.fmt_content(&format_dangling_comments, f)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DanglingCommentLayout {
    /// Writes each dangling comment on its own line.
    Multiline,

    /// Keeps adjacent comments on one line.
    ///
    /// Falls back to `Multiline` when any dangling comment is a `//` comment.
    InlineComments,
}

/// Returns `true` when every dangling comment can stay inline.
///
/// Example: `(/* one */ /* two */)` can stay inline, but `// one`
/// must fall back to the normal dangling-comment line layout.
fn can_keep_dangling_comments_inline<L: Language>(dangling_comments: &[SourceComment<L>]) -> bool {
    dangling_comments
        .iter()
        .all(|comment| !comment.kind().is_line())
}

impl DanglingIndentMode {
    fn fmt_content<Context, Content>(
        self,
        content: &Content,
        f: &mut Formatter<Context>,
    ) -> FormatResult<()>
    where
        Content: Format<Context>,
    {
        match self {
            Self::Block => write!(f, [block_indent(content)]),
            Self::Soft => write!(f, [group(&soft_block_indent(content))]),
            Self::None => write!(f, [content]),
        }
    }
}

pub trait FormatToken<L, C>
where
    L: Language,
    C: CstFormatContext<Language = L>,
{
    fn has_skipped(&self, token: &SyntaxToken<L>, f: &mut Formatter<C>) -> bool {
        f.comments().has_skipped(token)
    }
    fn format_removed(&self, token: &SyntaxToken<L>, f: &mut Formatter<C>) -> FormatResult<()> {
        f.state_mut().track_token(token);

        self.format_skipped_token_trivia(token, f)
    }

    /// Formats the skipped token trivia of `token`.
    fn format_skipped_token_trivia(
        &self,
        token: &SyntaxToken<L>,
        f: &mut Formatter<C>,
    ) -> FormatResult<()> {
        if f.comments().has_skipped(token) {
            self.fmt_skipped(token, f)
        } else {
            Ok(())
        }
    }

    /// Formats a token without its skipped token trivia
    ///
    /// ## Warning
    /// It's your responsibility to format any skipped trivia.
    fn format_trimmed_token_trivia(
        &self,
        token: &SyntaxToken<L>,
        f: &mut Formatter<C>,
    ) -> FormatResult<()> {
        let trimmed_range = token.text_trimmed_range();
        located_token_text(token, trimmed_range).fmt(f)
    }

    /// Print out a `token` from the original source with a different `content`.
    ///
    /// This will print the skipped token trivia that belong to `token` to `content`;
    /// `token` is then marked as consumed by the formatter.
    fn format_replaced(
        &self,
        token: &SyntaxToken<L>,
        content: &impl Format<C>,
        f: &mut Formatter<C>,
    ) -> FormatResult<()> {
        f.state_mut().track_token(token);
        self.format_skipped_token_trivia(token, f)?;
        f.write_fmt(Arguments::from(&Argument::new(content)))
    }

    #[cold]
    fn fmt_skipped(&self, token: &SyntaxToken<L>, f: &mut Formatter<C>) -> FormatResult<()> {
        fmt_skipped_token_trivia_impl(token, f)
    }
}

/// Formats the given token only if the group does break and otherwise retains the token's skipped token trivia.
pub fn format_only_if_breaks<'a, 'content, L, Content, Context>(
    token: &'a SyntaxToken<L>,
    content: &'content Content,
    ok_skipped: fn(&'a SyntaxToken<L>, &mut Formatter<Context>) -> FormatResult<()>,
) -> FormatOnlyIfBreaks<'a, 'content, L, Context>
where
    L: Language,
    Content: Format<Context>,
{
    FormatOnlyIfBreaks {
        token,
        content: Argument::new(content),
        group_id: None,
        ok_skipped,
    }
}

/// Formats a token with its skipped token trivia that only gets printed if its enclosing
/// group does break but otherwise gets omitted from the formatted output.
pub struct FormatOnlyIfBreaks<'a, 'content, L, C>
where
    L: Language,
{
    token: &'a SyntaxToken<L>,
    content: Argument<'content, C>,
    group_id: Option<GroupId>,
    ok_skipped: fn(&'a SyntaxToken<L>, &mut Formatter<C>) -> FormatResult<()>,
}

impl<L, C> FormatOnlyIfBreaks<'_, '_, L, C>
where
    L: Language,
{
    pub fn with_group_id(mut self, group_id: Option<GroupId>) -> Self {
        self.group_id = group_id;
        self
    }
}

impl<L, C> Format<C> for FormatOnlyIfBreaks<'_, '_, L, C>
where
    L: Language + 'static,
    C: CstFormatContext<Language = L>,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        write!(
            f,
            [if_group_breaks(&Arguments::from(&self.content)).with_group_id(self.group_id),]
        )?;

        let skipped = format_with(|f| (self.ok_skipped)(self.token, f));

        if f.comments().has_skipped(self.token) {
            // Print the trivia otherwise
            write!(
                f,
                [if_group_fits_on_line(&skipped).with_group_id(self.group_id)]
            )?;
        }

        Ok(())
    }
}

/// Formats the skipped token trivia of `token`.
pub const fn format_skipped_token_trivia<L: Language>(
    token: &SyntaxToken<L>,
) -> FormatSkippedTokenTrivia<'_, L> {
    FormatSkippedTokenTrivia { token }
}

/// Formats the skipped token trivia of `token`.
pub struct FormatSkippedTokenTrivia<'a, L: Language> {
    token: &'a SyntaxToken<L>,
}

impl<Context> Format<Context> for FormatSkippedTokenTrivia<'_, Context::Language>
where
    Context: CstFormatContext,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        if f.comments().has_skipped(self.token) {
            fmt_skipped_token_trivia_impl(self.token, f)
        } else {
            Ok(())
        }
    }
}

#[cold]
fn fmt_skipped_token_trivia_impl<L, Context>(
    token: &SyntaxToken<L>,
    f: &mut Formatter<Context>,
) -> FormatResult<()>
where
    L: Language,
    Context: CstFormatContext<Language = L>,
{
    // Lines/spaces before the next token/comment
    let (mut lines, mut spaces) = match token.prev_token() {
        Some(token) => {
            let mut lines = 0u32;
            let mut spaces = 0u32;
            for piece in token.trailing_trivia().pieces().rev() {
                if piece.is_whitespace() {
                    spaces += 1;
                } else if piece.is_newline() {
                    spaces = 0;
                    lines += 1;
                } else {
                    break;
                }
            }

            (lines, spaces)
        }
        None => (0, 0),
    };

    // The comments between the last skipped token trivia and the token
    let mut dangling_comments = Vec::new();
    let mut skipped_range: Option<TextRange> = None;

    // Iterate over the remaining pieces to find the full range from the first to the last skipped token trivia.
    // Extract the comments between the last skipped token trivia and the token.
    for piece in token.leading_trivia().pieces() {
        if piece.is_whitespace() {
            spaces += 1;
            continue;
        }

        if piece.is_newline() {
            lines += 1;
            spaces = 0;
        } else if let Some(comment) = piece.as_comments() {
            let source_comment = SourceComment {
                kind: Context::Style::get_comment_kind(&comment),
                lines_before: lines,
                lines_after: 0,
                piece: comment,
                #[cfg(debug_assertions)]
                formatted: Cell::new(true),
            };

            dangling_comments.push(source_comment);

            lines = 0;
            spaces = 0;
        } else if piece.is_skipped() {
            skipped_range = Some(match skipped_range {
                Some(range) => range.cover(piece.text_range()),
                None => {
                    if dangling_comments.is_empty() {
                        match lines {
                            0 if spaces == 0 => {
                                // Token had no space to previous token nor any preceding comment. Keep it that way
                            }
                            0 => write!(f, [space()])?,
                            _ => write!(f, [hard_line_break()])?,
                        };
                    } else {
                        match lines {
                            0 => write!(f, [space()])?,
                            1 => write!(f, [hard_line_break()])?,
                            _ => write!(f, [empty_line()])?,
                        };
                    }

                    piece.text_range()
                }
            });

            lines = 0;
            spaces = 0;
            dangling_comments.clear();
        }
    }

    let skipped_range =
        skipped_range.unwrap_or_else(|| TextRange::empty(token.text_range().start()));

    f.write_element(FormatElement::Tag(Tag::StartVerbatim(
        VerbatimKind::Verbatim {
            length: skipped_range.len(),
        },
    )))?;
    write!(f, [located_token_text(token, skipped_range)])?;
    f.write_element(FormatElement::Tag(Tag::EndVerbatim))?;

    // Write whitespace separator between skipped/last comment and token
    if dangling_comments.is_empty() {
        match lines {
            0 if spaces == 0 => {
                // Don't write a space if there was none in the source document.
                Ok(())
            }
            0 => write!(f, [space()]),
            _ => write!(f, [hard_line_break()]),
        }
    } else {
        match dangling_comments
            .first()
            .map_or(0, SourceComment::lines_before)
        {
            0 => write!(f, [space()])?,
            1 => write!(f, [hard_line_break()])?,
            _ => write!(f, [empty_line()])?,
        }

        write!(f, [format_dangling_comments_from_slice(&dangling_comments)])?;

        match lines {
            0 => write!(f, [space()]),
            _ => write!(f, [hard_line_break()]),
        }
    }
}
