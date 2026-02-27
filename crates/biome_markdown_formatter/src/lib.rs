use biome_markdown_syntax::{MarkdownLanguage, MarkdownSyntaxNode};

mod prelude;

mod comments;
pub mod context;
mod cst;
pub mod generated;
pub mod markdown;
pub mod verbatim;

use biome_formatter::{
    FormatContext, FormatLanguage, FormatResult, Formatted, TransformSourceMap, prelude::*,
};
use biome_rowan::AstNode;

pub(crate) use crate::context::MarkdownFormatContext;
use crate::{
    context::MarkdownFormatOptions, cst::FormatMarkdownSyntaxNode, verbatim::format_bogus_node,
};

pub(crate) type MarkdownFormatter<'buf> = Formatter<'buf, MarkdownFormatContext>;

#[derive(Debug, Clone, Default)]
pub struct MarkdownFormatLanguage {
    options: MarkdownFormatOptions,
}

impl MarkdownFormatLanguage {
    pub fn new(options: MarkdownFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for MarkdownFormatLanguage {
    type SyntaxLanguage = MarkdownLanguage;
    type Context = MarkdownFormatContext;
    type FormatRule = FormatMarkdownSyntaxNode;

    fn create_context(
        self,
        _root: &MarkdownSyntaxNode,
        source_map: Option<TransformSourceMap>,
        _delegate_fmt_embedded_nodes: bool,
    ) -> MarkdownFormatContext {
        MarkdownFormatContext::new(self.options.clone()).with_source_map(source_map)
    }

    fn options(&self) -> &<Self::Context as FormatContext>::Options {
        &self.options
    }
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
/// Allows to call format on optional AST fields without having to unwrap the field first.
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
/// Allows to call format on optional AST fields without having to unwrap the field first.
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

/// Used to get an object that knows how to format this object.
pub(crate) trait AsFormat<Context> {
    type Format<'a>: biome_formatter::Format<Context>
    where
        Self: 'a;

    /// Returns an object that is able to format this object.
    fn format(&self) -> Self::Format<'_>;
}

/// Rule for formatting an bogus nodes.
pub(crate) trait FormatBogusNodeRule<N>
where
    N: AstNode<Language = MarkdownLanguage>,
{
    fn fmt(&self, node: &N, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_bogus_node(node.syntax()).fmt(f)
    }
}

/// Format a [MarkdownSyntaxNode]
pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = MarkdownLanguage>,
{
    // this is the method that actually start the formatting
    fn fmt(&self, node: &N, f: &mut MarkdownFormatter) -> FormatResult<()> {
        self.fmt_fields(node, f)
    }

    fn fmt_fields(&self, node: &N, f: &mut MarkdownFormatter) -> FormatResult<()>;
}

/// Main entry point for formatting a Markdown file
pub fn format_node(
    options: MarkdownFormatOptions,
    root: &MarkdownSyntaxNode,
) -> FormatResult<Formatted<MarkdownFormatContext>> {
    biome_formatter::format_node(root, MarkdownFormatLanguage::new(options), false)
}
