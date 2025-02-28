mod comments;
pub mod context;
mod css;
mod cst;
mod generated;
mod prelude;
mod separated;
mod utils;

use std::borrow::Cow;

use crate::comments::CssCommentStyle;
pub(crate) use crate::context::CssFormatContext;
use crate::context::CssFormatOptions;
use crate::cst::FormatCssSyntaxNode;
use biome_css_syntax::{
    AnyCssDeclarationBlock, AnyCssRule, AnyCssRuleBlock, AnyCssValue, CssLanguage, CssSyntaxKind,
    CssSyntaxNode, CssSyntaxToken,
};
use biome_formatter::comments::Comments;
use biome_formatter::prelude::*;
use biome_formatter::trivia::format_skipped_token_trivia;
use biome_formatter::{
    write, CstFormatContext, FormatContext, FormatLanguage, FormatOwnedWithRule, FormatRefWithRule,
    TransformSourceMap,
};
use biome_formatter::{Formatted, Printed};
use biome_rowan::{AstNode, SyntaxNode, TextRange};
use biome_string_case::StrLikeExtension;

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

pub(crate) type CssFormatter<'buf> = Formatter<'buf, CssFormatContext>;

/// Format a [CssSyntaxNode]
pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = CssLanguage>,
{
    fn fmt(&self, node: &N, f: &mut CssFormatter) -> FormatResult<()> {
        if self.is_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_fields(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    fn fmt_fields(&self, node: &N, f: &mut CssFormatter) -> FormatResult<()>;

    /// Returns `true` if the node has a suppression comment and should use the same formatting as in the source document.
    fn is_suppressed(&self, node: &N, f: &CssFormatter) -> bool {
        f.context().comments().is_suppressed(node.syntax())
    }

    /// Formats the [leading comments](biome_formatter::comments#leading-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the leading comments.
    fn fmt_leading_comments(&self, node: &N, f: &mut CssFormatter) -> FormatResult<()> {
        format_leading_comments(node.syntax()).fmt(f)
    }

    /// Formats the [dangling comments](biome_formatter::comments#dangling-comments) of the node.
    ///
    /// You should override this method if the node handled by this rule can have dangling comments because the
    /// default implementation formats the dangling comments at the end of the node, which isn't ideal but ensures that
    /// no comments are dropped.
    ///
    /// A node can have dangling comments if all its children are tokens or if all node childrens are optional.
    fn fmt_dangling_comments(&self, node: &N, f: &mut CssFormatter) -> FormatResult<()> {
        format_dangling_comments(node.syntax())
            .with_soft_block_indent()
            .fmt(f)
    }

    /// Formats the [trailing comments](biome_formatter::comments#trailing-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the trailing comments.
    fn fmt_trailing_comments(&self, node: &N, f: &mut CssFormatter) -> FormatResult<()> {
        format_trailing_comments(node.syntax()).fmt(f)
    }
}

/// Rule for formatting an bogus nodes.
pub(crate) trait FormatBogusNodeRule<N>
where
    N: AstNode<Language = CssLanguage>,
{
    fn fmt(&self, node: &N, f: &mut CssFormatter) -> FormatResult<()> {
        format_bogus_node(node.syntax()).fmt(f)
    }
}

#[derive(Debug, Default, Clone)]
pub struct CssFormatLanguage {
    options: CssFormatOptions,
}

impl CssFormatLanguage {
    pub fn new(options: CssFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for CssFormatLanguage {
    type SyntaxLanguage = CssLanguage;
    type Context = CssFormatContext;
    type FormatRule = FormatCssSyntaxNode;

    // For CSS, range formatting allows:
    // - any block of rules or declarations
    // - any individual rule or declaration
    // - any individual value
    // - a complete value definition for a declaration
    fn is_range_formatting_node(&self, node: &SyntaxNode<Self::SyntaxLanguage>) -> bool {
        AnyCssDeclarationBlock::can_cast(node.kind())
            || AnyCssRuleBlock::can_cast(node.kind())
            || AnyCssValue::can_cast(node.kind())
            || AnyCssRule::can_cast(node.kind())
            || matches!(
                node.kind(),
                CssSyntaxKind::CSS_DECLARATION
                    | CssSyntaxKind::CSS_COMPONENT_VALUE_LIST
                    | CssSyntaxKind::CSS_SELECTOR_LIST
            )
    }

    fn options(&self) -> &<Self::Context as FormatContext>::Options {
        &self.options
    }

    fn create_context(
        self,
        root: &CssSyntaxNode,
        source_map: Option<TransformSourceMap>,
    ) -> Self::Context {
        let comments = Comments::from_node(root, &CssCommentStyle, source_map.as_ref());
        CssFormatContext::new(self.options, comments).with_source_map(source_map)
    }
}

/// Format implementation specific to CSS tokens.
///
/// This re-implementation of FormatToken allows the formatter to automatically
/// rewrite all keywords in lowercase, since they are case-insensitive. Other
/// tokens like identifiers handle lowercasing themselves.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct FormatCssSyntaxToken;

impl FormatRule<CssSyntaxToken> for FormatCssSyntaxToken {
    type Context = CssFormatContext;

    fn fmt(&self, token: &CssSyntaxToken, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
        f.state_mut().track_token(token);

        write!(f, [format_skipped_token_trivia(token)])?;

        if token.kind().is_contextual_keyword() {
            let original = token.text_trimmed();
            match original.to_ascii_lowercase_cow() {
                Cow::Borrowed(_) => write!(f, [format_trimmed_token(token)]),
                Cow::Owned(lowercase) => write!(
                    f,
                    [dynamic_text(&lowercase, token.text_trimmed_range().start())]
                ),
            }
        } else {
            write!(f, [format_trimmed_token(token)])
        }
    }
}

impl AsFormat<CssFormatContext> for CssSyntaxToken {
    type Format<'a> = FormatRefWithRule<'a, CssSyntaxToken, FormatCssSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatCssSyntaxToken)
    }
}

impl IntoFormat<CssFormatContext> for CssSyntaxToken {
    type Format = FormatOwnedWithRule<CssSyntaxToken, FormatCssSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatCssSyntaxToken)
    }
}

/// Formats a range within a file, supported by Biome
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [CssFormatOptions], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Printed] result with a range corresponding to the
/// range of the input that was effectively overwritten by the formatter
pub fn format_range(
    options: CssFormatOptions,
    root: &CssSyntaxNode,
    range: TextRange,
) -> FormatResult<Printed> {
    biome_formatter::format_range(root, range, CssFormatLanguage::new(options))
}

/// Formats a CSS syntax tree.
///
/// It returns the [Formatted] document that can be printed to a string.
pub fn format_node(
    options: CssFormatOptions,
    root: &CssSyntaxNode,
) -> FormatResult<Formatted<CssFormatContext>> {
    biome_formatter::format_node(root, CssFormatLanguage::new(options))
}

/// Formats a single node within a file, supported by Biome.
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [CssFormatOptions], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// Returns the [Printed] code.
pub fn format_sub_tree(options: CssFormatOptions, root: &CssSyntaxNode) -> FormatResult<Printed> {
    biome_formatter::format_sub_tree(root, CssFormatLanguage::new(options))
}

#[cfg(test)]
mod tests {
    use crate::context::CssFormatOptions;
    use crate::format_node;
    use biome_css_parser::{parse_css, CssParserOptions};

    #[test]
    fn smoke_test() {
        let src = r#"html {}"#;
        let parse = parse_css(src, CssParserOptions::default());
        let options = CssFormatOptions::default();
        let formatted = format_node(options, &parse.syntax()).unwrap();
        assert_eq!(formatted.print().unwrap().as_code(), "html {\n}\n");
    }
}
