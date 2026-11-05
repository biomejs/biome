#![deny(clippy::use_self)]

//! Biome's TOML formatter.

mod comments;
pub mod context;
mod cst;
mod generated;
mod prelude;
mod separated;
mod toml;
mod trivia;
mod utils;
mod verbatim;

use crate::comments::{FormatTomlLeadingComment, TomlCommentStyle};
pub use crate::context::{TomlFormatContext, TomlFormatOptions};
use crate::cst::FormatTomlSyntaxNode;
pub(crate) use crate::trivia::*;
use crate::verbatim::{format_bogus_node, format_suppressed_node};
use biome_formatter::Formatted;
use biome_formatter::comments::{Comments, SourceComment};
use biome_formatter::rowan::Printed;
use biome_formatter::rowan::prelude::*;
use biome_formatter::trivia::{FormatToken, format_skipped_token_trivia};
use biome_formatter::{
    CstFormatContext, FormatContext, FormatLanguage, FormatOwnedWithRule, FormatRefWithRule,
    TransformSourceMap, normalize_newlines, write,
};
use biome_rowan::{AstNode, SyntaxNode, TextRange};
use biome_toml_syntax::{
    AnyTomlRootItem, AnyTomlValue, TomlLanguage, TomlSyntaxNode, TomlSyntaxToken,
};

/// Used to get an object that knows how to format this object.
pub(crate) trait AsFormat<Context> {
    type Format<'a>: biome_formatter::Format<Context>
    where
        Self: 'a;

    /// Returns an object that is able to format this object.
    fn format(&self) -> Self::Format<'_>;
}

/// Implement [AsFormat] for references to types that implement [AsFormat].
impl<T, C> AsFormat<C> for &T
where
    T: AsFormat<C>,
{
    type Format<'a>
        = T::Format<'a>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_> {
        AsFormat::format(&**self)
    }
}

/// Implement [AsFormat] for [SyntaxResult] where `T` implements [AsFormat].
///
/// Useful to format mandatory AST fields without having to unwrap the value first.
impl<T, C> AsFormat<C> for biome_rowan::SyntaxResult<T>
where
    T: AsFormat<C>,
{
    type Format<'a>
        = biome_rowan::SyntaxResult<T::Format<'a>>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_> {
        match self {
            Ok(value) => Ok(value.format()),
            Err(err) => Err(*err),
        }
    }
}

/// Implement [AsFormat] for [Option] when `T` implements [AsFormat]
///
/// Allows calling format on optional AST fields without having to unwrap the field first.
impl<T, C> AsFormat<C> for Option<T>
where
    T: AsFormat<C>,
{
    type Format<'a>
        = Option<T::Format<'a>>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_> {
        self.as_ref().map(|value| value.format())
    }
}

/// Used to convert this object into an object that can be formatted.
///
/// The difference to [AsFormat] is that this trait takes ownership of `self`.
pub(crate) trait IntoFormat<Context> {
    type Format: biome_formatter::Format<Context>;

    fn into_format(self) -> Self::Format;
}

impl<T, Context> IntoFormat<Context> for biome_rowan::SyntaxResult<T>
where
    T: IntoFormat<Context>,
{
    type Format = biome_rowan::SyntaxResult<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Implement [IntoFormat] for [Option] when `T` implements [IntoFormat]
///
/// Allows calling format on optional AST fields without having to unwrap the field first.
impl<T, Context> IntoFormat<Context> for Option<T>
where
    T: IntoFormat<Context>,
{
    type Format = Option<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Formatting specific [Iterator] extensions
// False positive
#[expect(dead_code)]
pub(crate) trait FormattedIterExt {
    /// Converts every item to an object that knows how to format it.
    fn formatted<Context>(self) -> FormattedIter<Self, Self::Item, Context>
    where
        Self: Iterator + Sized,
        Self::Item: IntoFormat<Context>,
    {
        FormattedIter {
            inner: self,
            options: std::marker::PhantomData,
        }
    }
}

impl<I> FormattedIterExt for I where I: std::iter::Iterator {}

pub(crate) struct FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item>,
{
    inner: Iter,
    options: std::marker::PhantomData<Context>,
}

impl<Iter, Item, Context> std::iter::Iterator for FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item>,
    Item: IntoFormat<Context>,
{
    type Item = Item::Format;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.into_format())
    }
}

impl<Iter, Item, Context> std::iter::FusedIterator for FormattedIter<Iter, Item, Context>
where
    Iter: std::iter::FusedIterator<Item = Item>,
    Item: IntoFormat<Context>,
{
}

impl<Iter, Item, Context> std::iter::ExactSizeIterator for FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item> + std::iter::ExactSizeIterator,
    Item: IntoFormat<Context>,
{
}

pub(crate) type TomlFormatter<'buf> = Formatter<'buf, TomlFormatContext>;

/// Format a [TomlSyntaxNode]
pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = TomlLanguage>,
{
    fn fmt(&self, node: &N, f: &mut TomlFormatter) -> FormatResult<()> {
        if self.is_suppressed(node, f) || self.is_global_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_fields(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    fn fmt_fields(&self, node: &N, f: &mut TomlFormatter) -> FormatResult<()>;

    /// Returns `true` if the node has a suppression comment and should use the same formatting as in the source document.
    fn is_suppressed(&self, node: &N, f: &TomlFormatter) -> bool {
        f.context().comments().is_suppressed(node.syntax())
    }

    /// Returns `true` if the node has a global suppression comment and should use the same formatting as in the source document.
    fn is_global_suppressed(&self, node: &N, f: &TomlFormatter) -> bool {
        f.context().comments().is_global_suppressed(node.syntax())
    }

    /// Formats the [leading comments](biome_formatter::comments#leading-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the leading comments.
    fn fmt_leading_comments(&self, node: &N, f: &mut TomlFormatter) -> FormatResult<()> {
        format_leading_comments(node.syntax()).fmt(f)
    }

    /// Formats the [dangling comments](biome_formatter::comments#dangling-comments) of the node.
    ///
    /// You should override this method if the node handled by this rule can have dangling comments because the
    /// default implementation formats the dangling comments at the end of the node, which isn't ideal but ensures that
    /// no comments are dropped.
    ///
    /// A node can have dangling comments if all its children are tokens or if all node childrens are optional.
    fn fmt_dangling_comments(&self, node: &N, f: &mut TomlFormatter) -> FormatResult<()> {
        format_dangling_comments(node.syntax())
            .with_soft_block_indent()
            .fmt(f)
    }

    /// Formats the [trailing comments](biome_formatter::comments#trailing-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the trailing comments.
    fn fmt_trailing_comments(&self, node: &N, f: &mut TomlFormatter) -> FormatResult<()> {
        let comments = f.context().comments().clone();
        FormatTomlTrailingComments::new(comments.trailing_comments(node.syntax())).fmt(f)
    }
}

/// Formats the trailing comments of a node.
///
/// Every TOML comment runs to the end of its line, so each trailing comment
/// after the first starts a new line, even if the source separated the two
/// comments by a token that the formatter moves, such as the comma in
/// `[1 # a` / `, # b` / `2]`.
pub(crate) struct FormatTomlTrailingComments<'a> {
    comments: &'a [SourceComment<TomlLanguage>],
}

impl<'a> FormatTomlTrailingComments<'a> {
    pub(crate) fn new(comments: &'a [SourceComment<TomlLanguage>]) -> Self {
        Self { comments }
    }
}

impl Format<TomlFormatContext> for FormatTomlTrailingComments<'_> {
    fn fmt(&self, f: &mut TomlFormatter) -> FormatResult<()> {
        for (index, comment) in self.comments.iter().enumerate() {
            let format_comment = FormatRefWithRule::new(comment, FormatTomlLeadingComment);
            let content = format_with(|f| {
                match comment.lines_before() {
                    0 if index == 0 => write!(f, [space()])?,
                    0 | 1 => write!(f, [hard_line_break()])?,
                    _ => write!(f, [empty_line()])?,
                }
                write!(f, [format_comment])
            });

            write!(f, [line_suffix(&content), expand_parent()])?;
            comment.mark_formatted();
        }

        Ok(())
    }
}

/// Rule for formatting an bogus nodes.
pub(crate) trait FormatBogusNodeRule<N>
where
    N: AstNode<Language = TomlLanguage>,
{
    fn fmt(&self, node: &N, f: &mut TomlFormatter) -> FormatResult<()> {
        format_bogus_node(node.syntax()).fmt(f)
    }
}

#[derive(Debug, Default, Clone)]
pub struct TomlFormatLanguage {
    options: TomlFormatOptions,
}

impl TomlFormatLanguage {
    pub fn new(options: TomlFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for TomlFormatLanguage {
    type SyntaxLanguage = TomlLanguage;
    type Context = TomlFormatContext;
    type FormatRule = FormatTomlSyntaxNode;

    fn is_range_formatting_node(&self, node: &SyntaxNode<Self::SyntaxLanguage>) -> bool {
        AnyTomlRootItem::can_cast(node.kind()) || AnyTomlValue::can_cast(node.kind())
    }

    fn options(&self) -> &<Self::Context as FormatContext>::Options {
        &self.options
    }

    fn create_context(
        self,
        root: &TomlSyntaxNode,
        source_map: Option<TransformSourceMap>,
        _delegate_fmt_embedded_nodes: bool,
    ) -> Self::Context {
        let comments = Comments::from_node(root, &TomlCommentStyle, source_map.as_ref());
        TomlFormatContext::new(self.options, comments).with_source_map(source_map)
    }
}

/// Format implementation specific to TOML tokens.

#[derive(Debug, Default)]
pub(crate) struct FormatTomlSyntaxToken;

impl FormatRule<TomlSyntaxToken> for FormatTomlSyntaxToken {
    type Context = TomlFormatContext;

    fn fmt(&self, token: &TomlSyntaxToken, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
        f.state_mut().track_token(token);

        self.format_skipped_token_trivia(token, f)?;
        self.format_trimmed_token_trivia(token, f)?;

        Ok(())
    }
}

impl FormatToken<TomlLanguage, TomlFormatContext> for FormatTomlSyntaxToken {
    /// Prints the token text with LF line endings, which the printer converts
    /// to the configured line ending. Only multi-line strings contain line
    /// breaks, and TOML lets parsers normalize those.
    fn format_trimmed_token_trivia(
        &self,
        token: &TomlSyntaxToken,
        f: &mut Formatter<TomlFormatContext>,
    ) -> FormatResult<()> {
        syntax_token_cow_slice(
            normalize_newlines(token.text_trimmed(), ['\r']),
            token,
            token.text_trimmed_range().start(),
        )
        .fmt(f)
    }

    fn format_skipped_token_trivia(
        &self,
        token: &TomlSyntaxToken,
        f: &mut Formatter<TomlFormatContext>,
    ) -> FormatResult<()> {
        format_skipped_token_trivia(token).fmt(f)
    }
}

impl AsFormat<TomlFormatContext> for TomlSyntaxToken {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatTomlSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatTomlSyntaxToken)
    }
}

impl IntoFormat<TomlFormatContext> for TomlSyntaxToken {
    type Format = FormatOwnedWithRule<Self, FormatTomlSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatTomlSyntaxToken)
    }
}

/// Formats a range within a file, supported by Biome
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [TomlFormatOptions], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Printed] result with a range corresponding to the
/// range of the input that was effectively overwritten by the formatter
pub fn format_range(
    options: TomlFormatOptions,
    root: &TomlSyntaxNode,
    range: TextRange,
) -> FormatResult<Printed> {
    biome_formatter::format_range(root, range, TomlFormatLanguage::new(options))
}

/// Formats a TOML syntax tree.
///
/// It returns the [Formatted] document that can be printed to a string.
pub fn format_node(
    options: TomlFormatOptions,
    root: &TomlSyntaxNode,
) -> FormatResult<Formatted<TomlFormatContext>> {
    biome_formatter::format_node(root, TomlFormatLanguage::new(options), false)
}

/// Formats a single node within a file, supported by Biome.
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [TomlFormatOptions], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// Returns the [Printed] code.
pub fn format_sub_tree(options: TomlFormatOptions, root: &TomlSyntaxNode) -> FormatResult<Printed> {
    biome_formatter::format_sub_tree(root, TomlFormatLanguage::new(options))
}
