mod comments;
pub mod context;
mod cst;
mod generated;
mod grit;
mod prelude;
pub(crate) mod separated;

use biome_formatter::{
    comments::Comments,
    prelude::*,
    trivia::{format_dangling_comments, format_leading_comments, format_trailing_comments},
    write, CstFormatContext, Format, FormatLanguage, FormatResult, Formatted, Printed,
};
use biome_grit_syntax::{GritLanguage, GritSyntaxNode};
use comments::GritCommentStyle;

pub(crate) use crate::context::GritFormatContext;

use biome_rowan::{AstNode, TextRange};
use context::GritFormatOptions;
use cst::FormatGritSyntaxNode;

pub(crate) type GritFormatter<'buf> = Formatter<'buf, GritFormatContext>;

/// Formats a GritQL file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format_node(
    options: GritFormatOptions,
    root: &GritSyntaxNode,
) -> FormatResult<Formatted<GritFormatContext>> {
    biome_formatter::format_node(root, GritFormatLanguage::new(options))
}

/// Formats a range within a file, supported by Biome
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [GritFormatOptions], which
/// must match the current indentation of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Printed] result with a range corresponding to the
/// range of the input that was effectively overwritten by the formatter
pub fn format_range(
    options: GritFormatOptions,
    root: &GritSyntaxNode,
    range: TextRange,
) -> FormatResult<Printed> {
    biome_formatter::format_range(root, range, GritFormatLanguage::new(options))
}

/// Formats a single node within a file, supported by Biome.
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [GritFormatOptions], which
/// must match the current indentation of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// Returns the [Printed] code.
pub fn format_sub_tree(options: GritFormatOptions, root: &GritSyntaxNode) -> FormatResult<Printed> {
    biome_formatter::format_sub_tree(root, GritFormatLanguage::new(options))
}

pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = GritLanguage>,
{
    // this is the method that actually start the formatting
    fn fmt(&self, node: &N, f: &mut GritFormatter) -> FormatResult<()> {
        if self.is_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_fields(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    fn fmt_fields(&self, node: &N, f: &mut GritFormatter) -> FormatResult<()>;

    /// Returns `true` if the node has a suppression comment and should use the same formatting as in the source document.
    fn is_suppressed(&self, node: &N, f: &GritFormatter) -> bool {
        f.context().comments().is_suppressed(node.syntax())
    }

    /// Formats the [leading comments](biome_formatter::comments#leading-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the leading comments.
    fn fmt_leading_comments(&self, node: &N, f: &mut GritFormatter) -> FormatResult<()> {
        format_leading_comments(node.syntax()).fmt(f)
    }

    /// Formats the [dangling comments](biome_formatter::comments#dangling-comments) of the node.
    ///
    /// You should override this method if the node handled by this rule can have dangling comments because the
    /// default implementation formats the dangling comments at the end of the node, which isn't ideal but ensures that
    /// no comments are dropped.
    ///
    /// A node can have dangling comments if all its children are tokens or if all node childrens are optional.
    fn fmt_dangling_comments(&self, node: &N, f: &mut GritFormatter) -> FormatResult<()> {
        format_dangling_comments(node.syntax())
            .with_soft_block_indent()
            .fmt(f)
    }

    /// Formats the [trailing comments](biome_formatter::comments#trailing-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the trailing comments.
    fn fmt_trailing_comments(&self, node: &N, f: &mut GritFormatter) -> FormatResult<()> {
        format_trailing_comments(node.syntax()).fmt(f)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GritFormatLanguage {
    options: GritFormatOptions,
}

impl GritFormatLanguage {
    pub fn new(options: GritFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for GritFormatLanguage {
    type SyntaxLanguage = GritLanguage;

    type Context = GritFormatContext;

    type FormatRule = FormatGritSyntaxNode;

    fn transform(
        &self,
        _root: &biome_rowan::SyntaxNode<Self::SyntaxLanguage>,
    ) -> Option<(
        biome_rowan::SyntaxNode<Self::SyntaxLanguage>,
        biome_formatter::TransformSourceMap,
    )> {
        None
    }

    fn is_range_formatting_node(
        &self,
        _node: &biome_rowan::SyntaxNode<Self::SyntaxLanguage>,
    ) -> bool {
        true
    }

    fn options(&self) -> &<Self::Context as biome_formatter::FormatContext>::Options {
        &self.options
    }

    fn create_context(
        self,
        root: &biome_rowan::SyntaxNode<Self::SyntaxLanguage>,
        source_map: Option<biome_formatter::TransformSourceMap>,
    ) -> Self::Context {
        let comments: Comments<GritLanguage> =
            Comments::from_node(root, &GritCommentStyle, source_map.as_ref());

        GritFormatContext::new(self.options, comments).with_source_map(source_map)
    }
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

/// Formatting specific [Iterator] extensions
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

/// Rule for formatting an bogus nodes.
pub(crate) trait FormatBogusNodeRule<N>
where
    N: AstNode<Language = GritLanguage>,
{
    fn fmt(&self, node: &N, f: &mut GritFormatter) -> FormatResult<()> {
        format_bogus_node(node.syntax()).fmt(f)
    }
}
