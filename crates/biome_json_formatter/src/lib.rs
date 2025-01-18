mod comments;
pub mod context;
mod cst;
mod format_string;
mod generated;
mod json;
mod prelude;
mod separated;

use crate::comments::JsonCommentStyle;
pub(crate) use crate::context::JsonFormatContext;
use crate::context::JsonFormatOptions;
use crate::cst::FormatJsonSyntaxNode;
use biome_formatter::comments::Comments;
use biome_formatter::prelude::*;
use biome_formatter::{
    write, CstFormatContext, FormatContext, FormatLanguage, FormatOwnedWithRule, FormatRefWithRule,
    FormatToken, TransformSourceMap,
};
use biome_formatter::{Formatted, Printed};
use biome_json_syntax::{AnyJsonValue, JsonLanguage, JsonSyntaxNode, JsonSyntaxToken};
use biome_rowan::{AstNode, SyntaxNode, TextRange};

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

pub(crate) type JsonFormatter<'buf> = Formatter<'buf, JsonFormatContext>;

/// Format a [JsonSyntaxNode]
pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = JsonLanguage>,
{
    fn fmt(&self, node: &N, f: &mut JsonFormatter) -> FormatResult<()> {
        if self.is_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_fields(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    fn fmt_fields(&self, node: &N, f: &mut JsonFormatter) -> FormatResult<()>;

    /// Returns `true` if the node has a suppression comment and should use the same formatting as in the source document.
    fn is_suppressed(&self, node: &N, f: &JsonFormatter) -> bool {
        f.context().comments().is_suppressed(node.syntax())
    }

    /// Formats the [leading comments](biome_formatter::comments#leading-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the leading comments.
    fn fmt_leading_comments(&self, node: &N, f: &mut JsonFormatter) -> FormatResult<()> {
        format_leading_comments(node.syntax()).fmt(f)
    }

    /// Formats the [dangling comments](biome_formatter::comments#dangling-comments) of the node.
    ///
    /// You should override this method if the node handled by this rule can have dangling comments because the
    /// default implementation formats the dangling comments at the end of the node, which isn't ideal but ensures that
    /// no comments are dropped.
    ///
    /// A node can have dangling comments if all its children are tokens or if all node childrens are optional.
    fn fmt_dangling_comments(&self, node: &N, f: &mut JsonFormatter) -> FormatResult<()> {
        format_dangling_comments(node.syntax())
            .with_soft_block_indent()
            .fmt(f)
    }

    /// Formats the [trailing comments](biome_formatter::comments#trailing-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the trailing comments.
    fn fmt_trailing_comments(&self, node: &N, f: &mut JsonFormatter) -> FormatResult<()> {
        format_trailing_comments(node.syntax()).fmt(f)
    }
}

/// Rule for formatting an bogus nodes.
pub(crate) trait FormatBogusNodeRule<N>
where
    N: AstNode<Language = JsonLanguage>,
{
    fn fmt(&self, node: &N, f: &mut JsonFormatter) -> FormatResult<()> {
        format_bogus_node(node.syntax()).fmt(f)
    }
}

#[derive(Debug, Default, Clone)]
pub struct JsonFormatLanguage {
    options: JsonFormatOptions,
}

impl JsonFormatLanguage {
    pub fn new(options: JsonFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for JsonFormatLanguage {
    type SyntaxLanguage = JsonLanguage;
    type Context = JsonFormatContext;
    type FormatRule = FormatJsonSyntaxNode;

    fn is_range_formatting_node(&self, node: &SyntaxNode<Self::SyntaxLanguage>) -> bool {
        AnyJsonValue::can_cast(node.kind())
    }

    fn options(&self) -> &<Self::Context as FormatContext>::Options {
        &self.options
    }

    fn create_context(
        self,
        root: &JsonSyntaxNode,
        source_map: Option<TransformSourceMap>,
    ) -> Self::Context {
        let comments = Comments::from_node(root, &JsonCommentStyle, source_map.as_ref());
        JsonFormatContext::new(self.options, comments).with_source_map(source_map)
    }
}

/// Format implementation specific to JSON tokens.
pub(crate) type FormatJsonSyntaxToken = FormatToken<JsonFormatContext>;

impl AsFormat<JsonFormatContext> for JsonSyntaxToken {
    type Format<'a> = FormatRefWithRule<'a, JsonSyntaxToken, FormatJsonSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatJsonSyntaxToken::default())
    }
}

impl IntoFormat<JsonFormatContext> for JsonSyntaxToken {
    type Format = FormatOwnedWithRule<JsonSyntaxToken, FormatJsonSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatJsonSyntaxToken::default())
    }
}

/// Formats a range within a file, supported by Biome
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [JsonFormatOptions], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Printed] result with a range corresponding to the
/// range of the input that was effectively overwritten by the formatter
pub fn format_range(
    options: JsonFormatOptions,
    root: &JsonSyntaxNode,
    range: TextRange,
) -> FormatResult<Printed> {
    biome_formatter::format_range(root, range, JsonFormatLanguage::new(options))
}

/// Formats a JSON syntax tree.
///
/// It returns the [Formatted] document that can be printed to a string.
pub fn format_node(
    options: JsonFormatOptions,
    root: &JsonSyntaxNode,
) -> FormatResult<Formatted<JsonFormatContext>> {
    biome_formatter::format_node(root, JsonFormatLanguage::new(options))
}

/// Formats a single node within a file, supported by Biome.
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [JsonFormatOptions], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// Returns the [Printed] code.
pub fn format_sub_tree(options: JsonFormatOptions, root: &JsonSyntaxNode) -> FormatResult<Printed> {
    biome_formatter::format_sub_tree(root, JsonFormatLanguage::new(options))
}

#[cfg(test)]
mod tests {

    use crate::context::JsonFormatOptions;
    use crate::format_node;
    use biome_json_parser::{parse_json, JsonParserOptions};

    #[test]
    fn smoke_test() {
        let src = r#"
{
    "a": 5,
    "b": [1, 2, 3, 4],
    "c": null,
    "d": true,
    "e": false
}
"#;
        let parse = parse_json(src, JsonParserOptions::default());
        let options = JsonFormatOptions::default();
        let formatted = format_node(options, &parse.syntax()).unwrap();
        assert_eq!(
            formatted.print().unwrap().as_code(),
            "{\n\t\"a\": 5,\n\t\"b\": [1, 2, 3, 4],\n\t\"c\": null,\n\t\"d\": true,\n\t\"e\": false\n}\n"
        );
    }
}
