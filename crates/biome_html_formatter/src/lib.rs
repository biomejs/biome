#![deny(clippy::use_self)]

use crate::prelude::*;
use biome_formatter::comments::Comments;
use biome_formatter::prelude::Tag::{EndEmbedded, StartEmbedded};
use biome_formatter::trivia::{FormatToken, format_skipped_token_trivia};
use biome_formatter::{CstFormatContext, FormatOwnedWithRule, FormatRefWithRule, prelude::*};
use biome_formatter::{FormatLanguage, FormatResult, Formatted, write};
use biome_html_syntax::{HtmlLanguage, HtmlSyntaxNode, HtmlSyntaxToken};
use biome_rowan::{AstNode, SyntaxToken, TextRange};
use comments::HtmlCommentStyle;
use context::HtmlFormatContext;
pub use context::HtmlFormatOptions;
use cst::FormatHtmlSyntaxNode;

mod astro;
mod comments;
pub mod context;
mod cst;
mod generated;
mod html;
pub(crate) mod prelude;
mod trivia;
pub mod utils;
mod verbatim;

/// Formats a Html file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format_node(
    options: HtmlFormatOptions,
    root: &HtmlSyntaxNode,
    delegate_fmt_embedded_nodes: bool,
) -> FormatResult<Formatted<HtmlFormatContext>> {
    biome_formatter::format_node(
        root,
        HtmlFormatLanguage::new(options),
        delegate_fmt_embedded_nodes,
    )
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
        delegate_fmt_embedded_nodes: bool,
    ) -> Self::Context {
        let comments = Comments::from_node(root, &HtmlCommentStyle, source_map.as_ref());
        let context = HtmlFormatContext::new(self.options, comments).with_source_map(source_map);
        if delegate_fmt_embedded_nodes {
            context.with_fmt_embedded_nodes()
        } else {
            context
        }
    }
}

pub(crate) type HtmlFormatter<'buf> = Formatter<'buf, HtmlFormatContext>;

#[derive(Debug, Default)]
pub(crate) struct FormatHtmlSyntaxToken;

impl FormatRule<SyntaxToken<HtmlLanguage>> for FormatHtmlSyntaxToken {
    type Context = HtmlFormatContext;

    fn fmt(&self, token: &HtmlSyntaxToken, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
        f.state_mut().track_token(token);

        self.format_skipped_token_trivia(token, f)?;
        self.format_trimmed_token_trivia(token, f)?;

        Ok(())
    }
}

impl FormatToken<HtmlLanguage, HtmlFormatContext> for FormatHtmlSyntaxToken {
    fn format_skipped_token_trivia(
        &self,
        token: &HtmlSyntaxToken,
        f: &mut Formatter<HtmlFormatContext>,
    ) -> FormatResult<()> {
        format_skipped_token_trivia(token).fmt(f)
    }
}

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
        if let Some(range) = self.embedded_node_range(node, f) {
            // Tokens that belong to embedded nodes are formatted later on,
            // so we track them, even though they aren't formatted now during this pass.
            let state = f.state_mut();
            for token in node.syntax().tokens() {
                state.track_token(&token);
            }

            f.write_elements(vec![
                FormatElement::Tag(StartEmbedded(range)),
                FormatElement::Tag(EndEmbedded),
            ])?;
        } else {
            self.fmt_fields(node, f)?;
        }
        Ok(())
    }

    /// Whether this node contains content that needs to be formatted by an external formatter.
    /// If so, the function must return the range of the nodes that will be formatted in the second phase.
    fn embedded_node_range(&self, _node: &N, _f: &mut HtmlFormatter) -> Option<TextRange> {
        None
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
        format_html_leading_comments(node.syntax()).fmt(f)
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
        format_html_trailing_comments(node.syntax()).fmt(f)
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
    type Format<'a> = FormatRefWithRule<'a, Self, FormatHtmlSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatHtmlSyntaxToken)
    }
}

impl IntoFormat<HtmlFormatContext> for HtmlSyntaxToken {
    type Format = FormatOwnedWithRule<Self, FormatHtmlSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatHtmlSyntaxToken)
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
