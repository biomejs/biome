#![deny(clippy::use_self)]
#![expect(
    clippy::disallowed_methods,
    reason = "Formatting Svelte expressions requires complete node text."
)]

use crate::prelude::*;
use biome_formatter::comments::Comments;
use biome_formatter::prelude::Tag::{EndEmbedded, StartEmbedded};
use biome_formatter::trivia::{FormatToken, format_skipped_token_trivia};
use biome_formatter::{CstFormatContext, FormatOwnedWithRule, FormatRefWithRule, prelude::*};
use biome_formatter::{FormatLanguage, FormatResult, Formatted, VecBuffer, write};
use biome_html_syntax::{HtmlLanguage, HtmlSyntaxNode, HtmlSyntaxToken};
use biome_rowan::{AstNode, SyntaxToken, TextRange, TokenText};
use comments::HtmlCommentStyle;
use context::HtmlFormatContext;
pub use context::HtmlFormatOptions;
use cst::FormatHtmlSyntaxNode;

mod angular;
mod astro;
mod comments;
pub mod context;
mod cst;
mod generated;
mod html;
pub(crate) mod prelude;
pub(crate) mod separated;
pub(crate) mod shared;
mod svelte;
mod trivia;
pub mod utils;
mod verbatim;
mod vue;

/// Formats a HTML file based on its features.
///
/// `embedded_node_ranges` contains the content ranges of the embedded snippets
/// whose formatting is delegated to the formatter of their language. The nodes
/// holding these snippets are emitted as embedded tags, which the caller fills
/// with [Formatted::format_embedded]. With no ranges, embedded content is
/// printed as it's written.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format_node(
    options: HtmlFormatOptions,
    root: &HtmlSyntaxNode,
    embedded_node_ranges: Vec<TextRange>,
) -> FormatResult<Formatted<HtmlFormatContext>> {
    let delegate_fmt_embedded_nodes = !embedded_node_ranges.is_empty();
    biome_formatter::format_node(
        root,
        HtmlFormatLanguage::new(options).with_embedded_node_ranges(embedded_node_ranges),
        delegate_fmt_embedded_nodes,
    )
}

/// The code of a node that is formatted by the formatter of another language.
///
/// It's printed as `content`, the node's own formatting, unless the caller of
/// [format_node] replaces it with [Formatted::format_embedded].
pub(crate) struct FormatEmbedded {
    range: TextRange,
    content: Interned,
}

impl Format<HtmlFormatContext> for FormatEmbedded {
    fn fmt(&self, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.write_elements([
            FormatElement::Tag(StartEmbedded(self.range)),
            FormatElement::Interned(self.content.clone()),
            FormatElement::Tag(EndEmbedded),
        ])
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

#[derive(Debug, Clone)]
pub struct HtmlFormatLanguage {
    options: HtmlFormatOptions,
    embedded_node_ranges: Vec<TextRange>,
    inline_embedded_expressions: Vec<HtmlInlineEmbeddedExpression>,
}

/// JavaScript formatting prepared for an expression inside a Svelte attribute.
#[derive(Debug, Clone)]
pub struct HtmlInlineEmbeddedExpression {
    /// The exact source range of the expression content between its braces.
    pub range: TextRange,
    /// The prepared output, or a request to preserve the original expression.
    pub content: HtmlInlineEmbeddedContent,
    /// The unwrapped identifier when shorthand is safe and the expression has no comments.
    pub shorthand_identifier: Option<TokenText>,
}

/// The formatting result for a Svelte attribute expression.
#[derive(Debug, Clone)]
pub enum HtmlInlineEmbeddedContent {
    /// A guest-language document to render inside the original attribute braces.
    Formatted(Document),
    /// Preserve the source expression when guest-language formatting fails.
    Verbatim,
}

impl HtmlFormatLanguage {
    pub fn new(options: HtmlFormatOptions) -> Self {
        Self {
            options,
            embedded_node_ranges: Vec::new(),
            inline_embedded_expressions: Vec::new(),
        }
    }

    pub fn with_embedded_node_ranges(mut self, embedded_node_ranges: Vec<TextRange>) -> Self {
        self.embedded_node_ranges = embedded_node_ranges;
        self
    }

    /// Supplies expressions indexed by their source ranges.
    pub fn with_inline_embedded_expressions(
        mut self,
        mut expressions: Vec<HtmlInlineEmbeddedExpression>,
    ) -> Self {
        expressions.sort_by_key(|expression| expression.range.start());
        self.inline_embedded_expressions = expressions;
        self
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
        let context = HtmlFormatContext::new(self.options, comments)
            .with_source_map(source_map)
            .with_inline_embedded_expressions(self.inline_embedded_expressions);
        if delegate_fmt_embedded_nodes {
            context.with_embedded_node_ranges(self.embedded_node_ranges)
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
        if self.is_suppressed(node, f) || self.is_global_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_node(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    /// Formats the node without comments. Ignores any suppression comments.
    fn fmt_node(&self, node: &N, f: &mut HtmlFormatter) -> FormatResult<()> {
        if f.context().should_delegate_fmt_embedded_nodes() {
            let range = node.range();
            if f.context().is_embedded_node_range(range) {
                let mut buffer = VecBuffer::new(f.state_mut());
                write!(buffer, [format_with(|f| self.fmt_fields(node, f))])?;
                let embedded = FormatEmbedded {
                    range,
                    content: Interned::new(buffer.into_vec()),
                };
                return self.wrap_embed(node, &embedded, f);
            }
        }
        self.fmt_fields(node, f)
    }

    /// Writes `embedded`, a piece of code written in another language, such as
    /// the JavaScript inside `{ }` or inside a `<script>` tag.
    ///
    /// By default, the code stays on the same line as the surrounding markup, like
    /// `<p>{name}</p>`. Override this method when the code must start on its own
    /// line, like the content of a `<script>` tag.
    fn wrap_embed(
        &self,
        _node: &N,
        embedded: &FormatEmbedded,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        embedded.fmt(f)
    }

    /// Formats the node's fields.
    fn fmt_fields(&self, item: &N, f: &mut HtmlFormatter) -> FormatResult<()>;

    /// Returns `true` if the node has a suppression comment and should use the same formatting as in the source document.
    fn is_suppressed(&self, node: &N, f: &HtmlFormatter) -> bool {
        f.context().comments().is_suppressed(node.syntax())
    }

    /// Returns `true` if the node has a global suppression comment and should use the same formatting as in the source document.
    fn is_global_suppressed(&self, node: &N, f: &HtmlFormatter) -> bool {
        f.context().comments().is_global_suppressed(node.syntax())
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
