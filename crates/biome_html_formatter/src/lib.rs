use biome_formatter::comments::Comments;
use biome_formatter::{prelude::*, CstFormatContext, FormatOwnedWithRule, FormatRefWithRule};
use biome_formatter::{write, FormatLanguage, FormatResult, FormatToken, Formatted};
use biome_html_syntax::{HtmlLanguage, HtmlSyntaxNode, HtmlSyntaxToken};
use biome_rowan::AstNode;
use comments::HtmlCommentStyle;
use context::HtmlFormatContext;
pub use context::HtmlFormatOptions;
use cst::FormatHtmlSyntaxNode;

mod comments;
pub mod context;
mod cst;
mod generated;
mod html;
pub(crate) mod prelude;
pub mod utils;

/// Formats a Html file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format_node(
    options: HtmlFormatOptions,
    root: &HtmlSyntaxNode,
) -> FormatResult<Formatted<HtmlFormatContext>> {
    biome_formatter::format_node(root, HtmlFormatLanguage::new(options))
}

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

#[derive(Debug, Clone)]
pub struct HtmlFormatLanguage {
    options: HtmlFormatOptions,
}

impl HtmlFormatLanguage {
    pub fn new(options: HtmlFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for HtmlFormatLanguage {
    type SyntaxLanguage = HtmlLanguage;
    type Context = HtmlFormatContext;
    type FormatRule = FormatHtmlSyntaxNode;

    fn options(&self) -> &<Self::Context as biome_formatter::FormatContext>::Options {
        &self.options
    }

    fn create_context(
        self,
        root: &biome_rowan::SyntaxNode<Self::SyntaxLanguage>,
        source_map: Option<biome_formatter::TransformSourceMap>,
    ) -> Self::Context {
        let comments = Comments::from_node(root, &HtmlCommentStyle, source_map.as_ref());
        HtmlFormatContext::new(self.options, comments).with_source_map(source_map)
    }
}

pub(crate) type HtmlFormatter<'buf> = Formatter<'buf, HtmlFormatContext>;
pub(crate) type FormatHtmlSyntaxToken = FormatToken<HtmlFormatContext>;

// Rule for formatting a Html [AstNode].
pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = HtmlLanguage>,
{
    fn fmt(&self, node: &N, f: &mut HtmlFormatter) -> FormatResult<()> {
        if self.is_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_node(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    /// Formats the node without comments. Ignores any suppression comments.
    fn fmt_node(&self, node: &N, f: &mut HtmlFormatter) -> FormatResult<()> {
        self.fmt_fields(node, f)?;
        Ok(())
    }

    /// Formats the node's fields.
    fn fmt_fields(&self, item: &N, f: &mut HtmlFormatter) -> FormatResult<()>;

    /// Returns `true` if the node has a suppression comment and should use the same formatting as in the source document.
    fn is_suppressed(&self, node: &N, f: &HtmlFormatter) -> bool {
        f.context().comments().is_suppressed(node.syntax())
    }

    /// Formats the [leading comments](biome_formatter::comments#leading-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the leading comments.
    fn fmt_leading_comments(&self, node: &N, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_leading_comments(node.syntax()).fmt(f)
    }

    /// Formats the [dangling comments](biome_formatter::comments#dangling-comments) of the node.
    ///
    /// You should override this method if the node handled by this rule can have dangling comments because the
    /// default implementation formats the dangling comments at the end of the node, which isn't ideal but ensures that
    /// no comments are dropped.
    ///
    /// A node can have dangling comments if all its children are tokens or if all node childrens are optional.
    fn fmt_dangling_comments(&self, node: &N, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_dangling_comments(node.syntax())
            .with_soft_block_indent()
            .fmt(f)
    }

    /// Formats the [trailing comments](biome_formatter::comments#trailing-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the trailing comments.
    fn fmt_trailing_comments(&self, node: &N, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_trailing_comments(node.syntax()).fmt(f)
    }
}

/// Rule for formatting an bogus node.
pub(crate) trait FormatBogusNodeRule<N>
where
    N: AstNode<Language = HtmlLanguage>,
{
    fn fmt(&self, node: &N, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_bogus_node(node.syntax()).fmt(f)
    }
}

impl AsFormat<HtmlFormatContext> for HtmlSyntaxToken {
    type Format<'a> = FormatRefWithRule<'a, HtmlSyntaxToken, FormatHtmlSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatHtmlSyntaxToken::default())
    }
}

impl IntoFormat<HtmlFormatContext> for HtmlSyntaxToken {
    type Format = FormatOwnedWithRule<HtmlSyntaxToken, FormatHtmlSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatHtmlSyntaxToken::default())
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
