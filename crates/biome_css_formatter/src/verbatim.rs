use crate::context::CssFormatContext;
use biome_css_syntax::{CssLanguage, CssSyntaxNode, CssSyntaxToken};
use biome_formatter::format_element::tag::VerbatimKind;
use biome_formatter::formatter::Formatter;
use biome_formatter::prelude::{Tag, located_token_text, text};
use biome_formatter::trivia::{FormatLeadingComments, FormatTrailingComments};
use biome_formatter::{
    Buffer, CstFormatContext, Format, FormatContext, FormatElement, FormatError, FormatResult,
    FormatWithRule, LINE_TERMINATORS, normalize_newlines,
};
use biome_rowan::{AstNode, Direction, SyntaxElement, TextRange, TextSize};

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
pub fn format_css_verbatim_node(node: &CssSyntaxNode) -> FormatCssVerbatimNode<'_> {
    FormatCssVerbatimNode {
        node,
        kind: VerbatimKind::Verbatim {
            length: node.text_range_with_trivia().len(),
        },
        format_comments: true,
    }
}

/// Formats a source range from a subtree as verbatim source text.
///
/// Use this when a formatter must preserve source spacing but still replace
/// selected tokens. Example: SCSS string interpolation keeps `#{ get($map) }`
/// spacing while normalizing the string token in `#{".5"}`.
///
/// `range` must be inside `node.text_trimmed_range()`.
pub(crate) fn format_css_verbatim_range<FormatToken>(
    node: &CssSyntaxNode,
    range: TextRange,
    format_token: FormatToken,
) -> FormatCssVerbatimRange<'_, FormatToken>
where
    FormatToken: Fn(
        &CssSyntaxToken,
        TextRange,
        &mut Formatter<CssFormatContext>,
    ) -> FormatResult<CssVerbatimTokenFormat>,
{
    FormatCssVerbatimRange {
        node,
        range,
        format_token,
    }
}

pub(crate) struct FormatCssVerbatimRange<'node, FormatToken> {
    node: &'node CssSyntaxNode,
    range: TextRange,
    format_token: FormatToken,
}

/// Describes how a verbatim range formatter handled one token.
pub(crate) enum CssVerbatimTokenFormat {
    /// Copy the token text from the original source.
    Source,
    /// The caller already wrote a replacement for the token.
    Replacement,
}

impl<FormatToken> Format<CssFormatContext> for FormatCssVerbatimRange<'_, FormatToken>
where
    FormatToken: Fn(
        &CssSyntaxToken,
        TextRange,
        &mut Formatter<CssFormatContext>,
    ) -> FormatResult<CssVerbatimTokenFormat>,
{
    fn fmt(&self, f: &mut Formatter<CssFormatContext>) -> FormatResult<()> {
        debug_assert!(self.node.text_trimmed_range().contains_range(self.range));
        mark_css_verbatim_subtree(self.node, f);

        let mut last_end = self.range.start();

        for token in self.node.descendants_tokens(Direction::Next) {
            let Some(range) = token.text_trimmed_range().intersect(self.range) else {
                continue;
            };

            if range.is_empty() {
                continue;
            }

            if last_end < range.start() {
                write_verbatim_range_source(self.node, TextRange::new(last_end, range.start()), f)?;
            }

            f.state_mut().track_token(&token);

            match (self.format_token)(&token, range, f)? {
                CssVerbatimTokenFormat::Source => located_token_text(&token, range).fmt(f)?,
                CssVerbatimTokenFormat::Replacement => {}
            }

            last_end = range.end();
        }

        if last_end < self.range.end() {
            write_verbatim_range_source(self.node, TextRange::new(last_end, self.range.end()), f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FormatCssVerbatimNode<'node> {
    node: &'node CssSyntaxNode,
    kind: VerbatimKind,
    format_comments: bool,
}

impl Format<CssFormatContext> for FormatCssVerbatimNode<'_> {
    fn fmt(&self, f: &mut Formatter<CssFormatContext>) -> FormatResult<()> {
        fn source_range<Context>(f: &Formatter<Context>, range: TextRange) -> TextRange
        where
            Context: CstFormatContext,
        {
            f.context()
                .source_map()
                .map_or_else(|| range, |source_map| source_map.source_range(range))
        }

        let preserve_outer_trivia = self.node.parent().is_none();

        for element in self.node.descendants_with_tokens(Direction::Next) {
            match element {
                SyntaxElement::Token(token) => f.state_mut().track_token(&token),
                SyntaxElement::Node(node) => mark_css_verbatim_node(&node, f),
            }
        }

        // The trimmed range of a node is its range without any of its leading or trailing trivia.
        // Except for nodes that used to be parenthesized, the range than covers the source from the
        // `(` to the `)` (the trimmed range of the parenthesized expression, not the inner expression)
        let verbatim_source_range = if preserve_outer_trivia {
            source_range(f, self.node.text_range_with_trivia())
        } else {
            f.context().source_map().map_or_else(
                || self.node.text_trimmed_range(),
                |source_map| source_map.trimmed_source_range(self.node),
            )
        };

        let verbatim_text_start = if preserve_outer_trivia {
            self.node.text_range_with_trivia().start()
        } else {
            self.node.text_trimmed_range().start()
        };

        f.write_element(FormatElement::Tag(Tag::StartVerbatim(self.kind)))?;

        // Format all leading comments that are outside of the node's source range.
        if self.format_comments {
            let comments = f.context().comments().clone();
            let leading_comments = comments.leading_comments(self.node);

            let outside_trimmed_range = leading_comments.partition_point(|comment| {
                comment.piece().text_range().end() <= verbatim_source_range.start()
            });

            let (outside_trimmed_range, in_trimmed_range) =
                leading_comments.split_at(outside_trimmed_range);

            biome_formatter::write!(f, [FormatLeadingComments::Comments(outside_trimmed_range)])?;

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
            .take_while(|start| *start < verbatim_source_range.start())
            .next()
            .unwrap_or_else(|| verbatim_source_range.start());

        let original_source = f.context().source_map().map_or_else(
            || {
                if preserve_outer_trivia {
                    self.node.text_with_trivia().to_string()
                } else {
                    self.node.text_trimmed().to_string()
                }
            },
            |source_map| {
                source_map
                    .source()
                    .text_slice(verbatim_source_range.cover_offset(start_source))
                    .to_string()
            },
        );

        text(
            &normalize_newlines(&original_source, LINE_TERMINATORS),
            verbatim_text_start,
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
                source_range(f, comment.piece().text_range()).end() <= verbatim_source_range.end()
            });

            let (in_trimmed_range, outside_trimmed_range) =
                trailing_comments.split_at(outside_trimmed_range_start);

            for comment in in_trimmed_range {
                comment.mark_formatted();
            }

            biome_formatter::write!(f, [FormatTrailingComments::Comments(outside_trimmed_range)])?;
        }

        f.write_element(FormatElement::Tag(Tag::EndVerbatim))
    }
}

/// Marks comments and suppression checks for a subtree printed as source text.
///
/// Use this when a formatter writes verbatim source for a node and its child
/// formatters do not run, such as string interpolation `#{ get($map) }`.
fn mark_css_verbatim_subtree(node: &CssSyntaxNode, f: &Formatter<CssFormatContext>) {
    mark_css_verbatim_node(node, f);

    for element in node.descendants_with_tokens(Direction::Next) {
        let SyntaxElement::Node(node) = element else {
            continue;
        };

        mark_css_verbatim_node(&node, f);
    }
}

fn mark_css_verbatim_node(node: &CssSyntaxNode, f: &Formatter<CssFormatContext>) {
    let comments = f.context().comments();
    comments.mark_suppression_checked(node);

    for comment in comments.leading_dangling_trailing_comments(node) {
        comment.mark_formatted();
    }
}

fn write_verbatim_range_source(
    node: &CssSyntaxNode,
    range: TextRange,
    f: &mut Formatter<CssFormatContext>,
) -> FormatResult<()> {
    let raw = node.text_trimmed();
    let raw_start = node.text_trimmed_range().start();
    let relative_range = TextRange::new(range.start() - raw_start, range.end() - raw_start);
    let mut source = range.start();

    raw.slice(relative_range).try_for_each_chunk(|chunk| {
        text(chunk, source).fmt(f)?;
        source += TextSize::from(chunk.len() as u32);
        Ok(())
    })
}

/// Formats bogus nodes. The difference between this method  and `format_verbatim` is that this method
/// doesn't track nodes/tokens as [VerbatimKind::Verbatim]. They are just printed as they are.
pub fn format_bogus_node(node: &CssSyntaxNode) -> FormatCssVerbatimNode<'_> {
    FormatCssVerbatimNode {
        node,
        kind: VerbatimKind::Bogus,
        format_comments: true,
    }
}

/// Format a node having formatter suppression comment applied to it
pub fn format_suppressed_node(node: &CssSyntaxNode) -> FormatCssVerbatimNode<'_> {
    FormatCssVerbatimNode {
        node,
        kind: VerbatimKind::Suppressed,
        format_comments: true,
    }
}

/// Formats an object using its [`Format`] implementation but falls back to printing the object as
/// it is in the source document if formatting it returns an [`FormatError::SyntaxError`].
pub const fn format_or_verbatim<F>(inner: F) -> FormatNodeOrVerbatim<F> {
    FormatNodeOrVerbatim { inner }
}

/// Formats a node or falls back to verbatim printing if formatting this node fails.
#[derive(Copy, Clone)]
pub struct FormatNodeOrVerbatim<F> {
    inner: F,
}

impl<F, Item> Format<CssFormatContext> for FormatNodeOrVerbatim<F>
where
    F: FormatWithRule<CssFormatContext, Item = Item>,
    Item: AstNode<Language = CssLanguage>,
{
    fn fmt(&self, f: &mut Formatter<CssFormatContext>) -> FormatResult<()> {
        let snapshot = Formatter::state_snapshot(f);

        match self.inner.fmt(f) {
            Ok(result) => Ok(result),

            Err(FormatError::SyntaxError) => {
                f.restore_state_snapshot(snapshot);

                // Lists that yield errors are formatted as they were suppressed nodes.
                // Doing so, the formatter formats the nodes/tokens as is.
                format_suppressed_node(self.inner.item().syntax()).fmt(f)
            }
            Err(err) => Err(err),
        }
    }
}
