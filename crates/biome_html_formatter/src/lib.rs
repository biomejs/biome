#![deny(clippy::use_self)]
#![expect(
    clippy::disallowed_methods,
    reason = "Formatting Svelte expressions requires complete node text."
)]

use crate::html::lists::element_list::FormatHtmlElementList;
use crate::prelude::*;
use biome_formatter::comments::Comments;
use biome_formatter::trivia::{FormatToken, format_skipped_token_trivia};
use biome_formatter::{
    CstFormatContext, FormatOwnedWithRule, FormatRefWithRule, rowan::prelude::*,
};
use biome_formatter::{FormatLanguage, FormatResult, Formatted, write};
use biome_html_syntax::{
    AnySvelteBlock, HtmlAttributeSingleTextExpression, HtmlBogusTextExpression, HtmlElement,
    HtmlElementList, HtmlLanguage, HtmlSingleTextExpression, HtmlSpreadAttribute,
    HtmlSyntaxElement, HtmlSyntaxKind, HtmlSyntaxNode, HtmlSyntaxToken, HtmlTextExpression,
    SvelteAttachAttribute, SvelteBindFunctionBindingExpression, SvelteBogusBlock, SvelteName,
    element_ext::AnyHtmlTagElement,
};
use biome_rowan::{AstNode, SyntaxToken, TextRange};
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
    format_node_with_verbatim_ranges(options, root, embedded_node_ranges, Vec::new())
}

/// Formats HTML while preserving failed embedded snippets in `verbatim_embedded_node_ranges`.
/// Successful snippets in `embedded_node_ranges` remain available for guest-language replacement.
pub fn format_node_with_verbatim_ranges(
    options: HtmlFormatOptions,
    root: &HtmlSyntaxNode,
    embedded_node_ranges: Vec<TextRange>,
    mut verbatim_embedded_node_ranges: Vec<TextRange>,
) -> FormatResult<Formatted<HtmlFormatContext>> {
    if options.file_source().is_svelte() {
        // Suppressed and verbatim ancestors skip their children's rules. Reject uncertain
        // boundaries before formatting anything the parser treated as a sibling.
        let recovery_kinds = AnySvelteBlock::KIND_SET
            .union(HtmlBogusTextExpression::KIND_SET)
            .union(HtmlElementList::KIND_SET);
        let mut opaque_contents = Vec::new();
        let mut events = root.preorder_of_kinds(recovery_kinds);
        while let Some(event) = events.next() {
            let biome_rowan::WalkEvent::Enter(node) = event else {
                continue;
            };
            if HtmlElementList::can_cast(node.kind()) {
                let Some(element) = node.parent().and_then(HtmlElement::cast) else {
                    continue;
                };
                let opening = element.opening_element()?;
                let opaque =
                    AnyHtmlTagElement::from(opening.clone()).requires_verbatim_svelte_content();
                if opaque || element.is_script_tag() || element.is_style_tag() {
                    opening.r_angle_token()?;
                    let closing = element.closing_element()?;
                    closing.name()?;
                    closing.r_angle_token()?;
                }
                if opaque {
                    // Only the body belongs to the preprocessor. Attribute expressions are host syntax.
                    opaque_contents.push(node.text_range_with_trivia());
                    events.skip_subtree();
                }
            } else if HtmlBogusTextExpression::can_cast(node.kind())
                || !has_svelte_closing_delimiter(&node)
            {
                return Err(biome_formatter::FormatError::SyntaxError);
            }
        }
        for range in &mut verbatim_embedded_node_ranges {
            let index = opaque_contents.partition_point(|body| body.end() <= range.start());
            if opaque_contents
                .get(index)
                .is_some_and(|body| body.contains_range(*range))
            {
                continue;
            }
            // Whitespace inside `{  expr }` can belong to the opening delimiter.
            // Protect the containing construct, not just the guest token.
            if root.text_range_with_trivia().contains_range(*range)
                && let Some(expression) = root.covering_element(*range).parent().filter(|node| {
                    HtmlTextExpression::can_cast(node.kind()) || SvelteName::can_cast(node.kind())
                })
                && let Some(delimited) = expression
                    .ancestors()
                    .take_while(|node| !HtmlElementList::can_cast(node.kind()))
                    .find(|node| {
                        HtmlSingleTextExpression::can_cast(node.kind())
                            || HtmlAttributeSingleTextExpression::can_cast(node.kind())
                            || HtmlSpreadAttribute::can_cast(node.kind())
                            || SvelteAttachAttribute::can_cast(node.kind())
                            || SvelteBindFunctionBindingExpression::can_cast(node.kind())
                            || is_svelte_header(node.kind())
                            || AnySvelteBlock::can_cast(node.kind())
                    })
            {
                if is_svelte_header(delimited.kind()) {
                    *range = svelte_header_range(&delimited)
                        .ok_or(biome_formatter::FormatError::SyntaxError)?;
                    continue;
                }
                if !has_svelte_closing_delimiter(&delimited) {
                    return Err(biome_formatter::FormatError::SyntaxError);
                }
                *range = delimited.text_trimmed_range();
            }
        }
    }
    let has_embedded_nodes =
        !embedded_node_ranges.is_empty() || !verbatim_embedded_node_ranges.is_empty();
    biome_formatter::format_node(
        root,
        HtmlFormatLanguage::new(options)
            .with_embedded_node_ranges(embedded_node_ranges)
            .with_verbatim_embedded_node_ranges(verbatim_embedded_node_ranges),
        has_embedded_nodes,
    )
}

fn is_svelte_header(kind: HtmlSyntaxKind) -> bool {
    matches!(
        kind,
        HtmlSyntaxKind::SVELTE_IF_OPENING_BLOCK
            | HtmlSyntaxKind::SVELTE_ELSE_IF_CLAUSE
            | HtmlSyntaxKind::SVELTE_AWAIT_OPENING_BLOCK
            | HtmlSyntaxKind::SVELTE_AWAIT_THEN_BLOCK
            | HtmlSyntaxKind::SVELTE_AWAIT_CATCH_BLOCK
            | HtmlSyntaxKind::SVELTE_SNIPPET_OPENING_BLOCK
            | HtmlSyntaxKind::SVELTE_EACH_OPENING_BLOCK
            | HtmlSyntaxKind::SVELTE_KEY_OPENING_BLOCK
    )
}

/// Ends at the header's own brace, even when its node also contains the body.
fn svelte_header_range(node: &HtmlSyntaxNode) -> Option<TextRange> {
    let close = node
        .children_with_tokens()
        .find(|child| child.kind() == HtmlSyntaxKind::R_CURLY)?;
    Some(TextRange::new(
        node.text_trimmed_range().start(),
        close.text_trimmed_range().end(),
    ))
}

/// A nested expression's closing brace does not delimit its containing block.
fn has_svelte_closing_delimiter(node: &HtmlSyntaxNode) -> bool {
    match node.last_child_or_token() {
        // Recovery stops at the first brace. Only an argument-free `{@name}`
        // rules out nested delimiters and the need for a later closing block.
        Some(HtmlSyntaxElement::Token(_)) if SvelteBogusBlock::can_cast(node.kind()) => {
            !node.has_skipped_descendants()
                && node.borrowed_tokens().map(|token| token.kind()).eq([
                    HtmlSyntaxKind::SV_CURLY_AT,
                    HtmlSyntaxKind::IDENT,
                    HtmlSyntaxKind::R_CURLY,
                ])
        }
        Some(HtmlSyntaxElement::Token(token)) => token.kind() == HtmlSyntaxKind::R_CURLY,
        Some(HtmlSyntaxElement::Node(closing)) => {
            matches!(
                closing.kind(),
                HtmlSyntaxKind::SVELTE_AWAIT_CLOSING_BLOCK
                    | HtmlSyntaxKind::SVELTE_EACH_CLOSING_BLOCK
                    | HtmlSyntaxKind::SVELTE_IF_CLOSING_BLOCK
                    | HtmlSyntaxKind::SVELTE_KEY_CLOSING_BLOCK
                    | HtmlSyntaxKind::SVELTE_SNIPPET_CLOSING_BLOCK
            ) && closing.slots().all(|slot| slot.into_token().is_some())
        }
        None => false,
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
    verbatim_embedded_node_ranges: Vec<TextRange>,
}

impl HtmlFormatLanguage {
    pub fn new(options: HtmlFormatOptions) -> Self {
        Self {
            options,
            embedded_node_ranges: Vec::new(),
            verbatim_embedded_node_ranges: Vec::new(),
        }
    }

    pub fn with_embedded_node_ranges(mut self, embedded_node_ranges: Vec<TextRange>) -> Self {
        self.embedded_node_ranges = embedded_node_ranges;
        self
    }

    pub fn with_verbatim_embedded_node_ranges(
        mut self,
        verbatim_embedded_node_ranges: Vec<TextRange>,
    ) -> Self {
        self.verbatim_embedded_node_ranges = verbatim_embedded_node_ranges;
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
        let context = HtmlFormatContext::new(self.options, comments).with_source_map(source_map);
        if delegate_fmt_embedded_nodes {
            context
                .with_embedded_node_ranges(self.embedded_node_ranges)
                .with_verbatim_embedded_node_ranges(self.verbatim_embedded_node_ranges)
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
        if is_svelte_header(node.syntax().kind())
            && f.context().has_embedded_node_ranges()
            && let Some(range) = svelte_header_range(node.syntax())
            && f.context().is_verbatim_embedded_node_range(range)
            && let Some(children) = node.syntax().last_child().and_then(HtmlElementList::cast)
        {
            FormatSvelteVerbatimHeader {
                node: node.syntax(),
                range,
            }
            .fmt(f)?;
            FormatHtmlElementList::default()
                .with_multiline()
                .fmt(&children, f)?;
            hard_line_break().fmt(f)?;
            return self.fmt_trailing_comments(node, f);
        }
        if f.options().file_source().is_svelte()
            && f.context().is_verbatim_embedded_node_range(node.range())
        {
            verbatim::format_html_source_node_preserving_line_endings(node.syntax())
                .without_comments()
                .fmt(f)?;
            return self.fmt_trailing_comments(node, f);
        }
        self.fmt_node(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    /// Formats the node without comments. Ignores any suppression comments.
    fn fmt_node(&self, node: &N, f: &mut HtmlFormatter) -> FormatResult<()> {
        if f.context().has_embedded_node_ranges() {
            let range = self.embedded_range(node);
            if f.context().is_verbatim_embedded_node_range(range) {
                return self.fmt_verbatim_embedded_node(node, f);
            }
            if f.context().should_delegate_fmt_embedded_nodes()
                && f.context().is_embedded_node_range(range)
            {
                let content = format_with(|f| self.fmt_fields(node, f));
                let embedded = FormatEmbedded::new(range, &content, f)?;
                return self.wrap_embed(node, &embedded, f);
            }
        }
        self.fmt_fields(node, f)
    }

    /// Formats a failed embedded snippet without normalizing its guest-language source.
    fn fmt_verbatim_embedded_node(&self, node: &N, f: &mut HtmlFormatter) -> FormatResult<()> {
        self.fmt_fields(node, f)
    }

    /// Returns the range of the embedded snippet, if applicable.
    ///
    /// This is useful in situations where the range the actual expression lives is smaller than the node itself,
    /// like in Vue, where expressions are contained in attribute values, inside the quotes.
    ///
    /// Override this method when the code doesn't start and end with the node.
    fn embedded_range(&self, node: &N) -> TextRange {
        node.range()
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
