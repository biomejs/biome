#![deny(clippy::use_self)]

mod comments;
pub mod context;
mod css;
mod cst;
mod generated;
mod prelude;
mod scss;
mod separated;
mod tailwind;
mod trivia;
mod utils;
mod verbatim;

use std::borrow::Cow;

use crate::comments::CssCommentStyle;
pub(crate) use crate::context::CssFormatContext;
use crate::context::CssFormatOptions;
use crate::cst::FormatCssSyntaxNode;
use crate::prelude::{format_bogus_node, format_suppressed_node};
pub(crate) use crate::trivia::*;
use crate::utils::case::CssCase;
use biome_css_syntax::{
    AnyCssDeclarationBlock, AnyCssRule, AnyCssRuleBlock, AnyCssValue, CssLanguage, CssSyntaxKind,
    CssSyntaxNode, CssSyntaxNodeWithOffset, CssSyntaxToken,
};
use biome_formatter::comments::Comments;
use biome_formatter::prelude::*;
use biome_formatter::trivia::{FormatToken, format_skipped_token_trivia};
use biome_formatter::{
    CstFormatContext, FormatContext, FormatLanguage, FormatOwnedWithRule, FormatRefWithRule,
    FormatRuleWithTextCase, TransformSourceMap, write,
};
use biome_formatter::{Formatted, Printed};
use biome_rowan::{AstNode, SyntaxNode, TextRange};
use biome_string_case::StrLikeExtension;

/// Used to get an object that knows how to format this object.
pub(crate) trait AsFormat<Context> {
    type Format<'a>: Format<Context>
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
        if self.is_suppressed(node, f) || self.is_global_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_node(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    /// Formats the node body after leading comments and before dangling or
    /// trailing comments.
    ///
    /// Override this when a node needs more than raw `fmt_fields`, for example
    /// to keep `/* comment */ $arg: 1px` grouped as one `@include` argument.
    fn fmt_node(&self, node: &N, f: &mut CssFormatter) -> FormatResult<()> {
        self.fmt_fields(node, f)
    }

    fn fmt_fields(&self, node: &N, f: &mut CssFormatter) -> FormatResult<()>;

    /// Returns `true` if the node has a suppression comment and should use the same formatting as in the source document.
    fn is_suppressed(&self, node: &N, f: &CssFormatter) -> bool {
        f.context().comments().is_suppressed(node.syntax())
    }

    /// Returns `true` if the node has a global suppression comment and should use the same formatting as in the source document.
    fn is_global_suppressed(&self, node: &N, f: &CssFormatter) -> bool {
        f.context().comments().is_global_suppressed(node.syntax())
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
        _delegate_fmt_embedded_nodes: bool,
    ) -> Self::Context {
        let comments = Comments::from_node(root, &CssCommentStyle, source_map.as_ref());
        CssFormatContext::new(self.options, comments).with_source_map(source_map)
    }
}

/// Format implementation specific to CSS tokens.
///
/// This re-implementation of FormatToken owns token trivia and casing policy.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct FormatCssSyntaxToken {
    case: CssCase,
}

impl FormatRuleWithTextCase<CssLanguage> for FormatCssSyntaxToken {
    #[inline]
    fn with_text_case(mut self, case: CssCase) -> Self {
        self.case = case;
        self
    }
}

impl FormatRule<CssSyntaxToken> for FormatCssSyntaxToken {
    type Context = CssFormatContext;

    fn fmt(&self, token: &CssSyntaxToken, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
        f.state_mut().track_token(token);

        self.format_skipped_token_trivia(token, f)?;

        #[cfg(debug_assertions)]
        if self.case == CssCase::Auto {
            crate::utils::case::record_auto_contextual_token(token, f);
        }

        if self.case == CssCase::Lowercase {
            let original = token.text_trimmed();
            match original.to_ascii_lowercase_cow() {
                Cow::Borrowed(_) => self.format_trimmed_token_trivia(token, f),
                Cow::Owned(lowercase) => {
                    write!(
                        f,
                        [text(&lowercase, Some(token.text_trimmed_range().start()))]
                    )
                }
            }
        } else {
            self.format_trimmed_token_trivia(token, f)
        }
    }
}

impl FormatToken<CssLanguage, CssFormatContext> for FormatCssSyntaxToken {
    fn format_skipped_token_trivia(
        &self,
        token: &CssSyntaxToken,
        f: &mut Formatter<CssFormatContext>,
    ) -> FormatResult<()> {
        format_skipped_token_trivia(token).fmt(f)
    }
}

impl AsFormat<CssFormatContext> for CssSyntaxToken {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatCssSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatCssSyntaxToken::default())
    }
}

impl IntoFormat<CssFormatContext> for CssSyntaxToken {
    type Format = FormatOwnedWithRule<Self, FormatCssSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatCssSyntaxToken::default())
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
    biome_formatter::format_node(root, CssFormatLanguage::new(options), false)
}

/// Formats a CSS syntax tree.
///
/// It returns the [Formatted] document that can be printed to a string.
pub fn format_node_with_offset(
    options: CssFormatOptions,
    root: &CssSyntaxNodeWithOffset,
) -> FormatResult<Formatted<CssFormatContext>> {
    biome_formatter::format_node_with_offset(root, CssFormatLanguage::new(options), false)
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
    use crate::comments::CssCommentStyle;
    use crate::context::CssFormatOptions;
    use crate::format_node;
    use crate::utils::case::CssCase;
    use crate::{AsFormat, CssFormatContext, CssFormatLanguage, CssFormatter, FormatNodeRule};
    use biome_css_parser::{CssParserOptions, parse_css};
    use biome_css_syntax::CssSyntaxKind;
    use biome_css_syntax::{
        AnyCssFunctionName, CssGenericComponentValueList, CssIdentifier, CssRoot,
    };
    use biome_formatter::comments::Comments;
    use biome_formatter::prelude::token;
    use biome_formatter::{
        Buffer, FormatLanguage, FormatRefWithRule, FormatResult, FormatRule, FormatState,
        FormatTextCaseExt as _, VecBuffer, write,
    };
    use biome_languages::CssFileSource;
    use biome_rowan::{AstNode, Direction};

    #[test]
    fn smoke_test() {
        let src = r#"html {}"#;
        let parse = parse_css(src, CssFileSource::css(), CssParserOptions::default());
        let options = CssFormatOptions::default();
        let formatted = format_node(options, &parse.syntax()).unwrap();
        assert_eq!(formatted.print().unwrap().as_code(), "html {\n}\n");
    }

    #[test]
    fn css_syntax_token_format_supports_case_options() {
        let parse = parse_css(
            "@IMPORT \"Keep\";",
            CssFileSource::css(),
            CssParserOptions::default(),
        );
        let syntax = parse.syntax();
        let import_token = syntax
            .descendants_tokens(Direction::Next)
            .find(|token| token.kind() == CssSyntaxKind::IMPORT_KW)
            .unwrap();
        let comments = Comments::from_node(&syntax, &CssCommentStyle, None);
        let context = CssFormatContext::new(CssFormatOptions::default(), comments);

        let lowercase = biome_formatter::format!(
            context,
            [import_token.format().with_text_case(CssCase::Lowercase)]
        )
        .unwrap();
        assert_eq!(lowercase.print().unwrap().as_code(), "import");

        let comments = Comments::from_node(&syntax, &CssCommentStyle, None);
        let context = CssFormatContext::new(CssFormatOptions::default(), comments);
        let preserved = biome_formatter::format!(
            context,
            [import_token.format().with_text_case(CssCase::Preserve)]
        )
        .unwrap();
        assert_eq!(preserved.print().unwrap().as_code(), "IMPORT");
    }

    #[test]
    fn scoped_identifier_case_uses_the_normal_union_formatter() {
        let parse = parse_css(
            ".a { color: RGB(0 0 0); }",
            CssFileSource::css(),
            CssParserOptions::default(),
        );
        let syntax = parse.syntax();
        let name = syntax
            .descendants()
            .filter_map(AnyCssFunctionName::cast)
            .find(|name| name.syntax().text_trimmed() == "RGB")
            .unwrap();
        let comments = Comments::from_node(&syntax, &CssCommentStyle, None);
        let context = CssFormatContext::new(CssFormatOptions::default(), comments);

        let formatted =
            biome_formatter::format!(context, [name.format().with_text_case(CssCase::Lowercase)])
                .unwrap();

        assert_eq!(formatted.print().unwrap().as_code(), "rgb");
        assert_eq!(formatted.context().identifier_case(), CssCase::Auto);
    }

    #[test]
    fn scoped_identifier_preserve_overrides_an_enclosing_case() {
        let parse = parse_css(
            ".a { color: RGB(0 0 0); }",
            CssFileSource::css(),
            CssParserOptions::default(),
        );
        let syntax = parse.syntax();
        let name = syntax
            .descendants()
            .filter_map(AnyCssFunctionName::cast)
            .find(|name| name.syntax().text_trimmed() == "RGB")
            .unwrap();
        let comments = Comments::from_node(&syntax, &CssCommentStyle, None);
        let context = CssFormatContext::new(CssFormatOptions::default(), comments);

        let formatted = biome_formatter::format!(
            context,
            [name
                .format()
                .with_text_case(CssCase::Preserve)
                .with_text_case(CssCase::Lowercase)]
        )
        .unwrap();

        assert_eq!(formatted.print().unwrap().as_code(), "RGB");
        assert_eq!(formatted.context().identifier_case(), CssCase::Auto);
    }

    #[test]
    fn scoped_identifier_case_does_not_leak_into_scss_function_names() {
        let parse = parse_css(
            ".a { x: #{$KeepFunc}($Keep); }",
            CssFileSource::scss(),
            CssParserOptions::default(),
        );
        assert!(parse.diagnostics().is_empty(), "{:?}", parse.diagnostics());

        let syntax = parse.syntax();
        let name = syntax
            .descendants()
            .filter_map(AnyCssFunctionName::cast)
            .find(|name| name.syntax().text_trimmed() == "#{$KeepFunc}")
            .unwrap();
        let comments = Comments::from_node(&syntax, &CssCommentStyle, None);
        let context = CssFormatContext::new(CssFormatOptions::default(), comments);

        let formatted =
            biome_formatter::format!(context, [name.format().with_text_case(CssCase::Lowercase)])
                .unwrap();

        assert_eq!(formatted.print().unwrap().as_code(), "#{$KeepFunc}");
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "CSS identifier case requires an identifier-capable node")]
    fn scoped_identifier_case_rejects_non_identifier_nodes() {
        let parse = parse_css("", CssFileSource::css(), CssParserOptions::default());
        let syntax = parse.syntax();
        let root = CssRoot::cast(syntax.clone()).unwrap();
        let comments = Comments::from_node(&syntax, &CssCommentStyle, None);
        let context = CssFormatContext::new(CssFormatOptions::default(), comments);

        biome_formatter::format!(context, [root.format().with_text_case(CssCase::Preserve)])
            .unwrap();
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "CSS formatter used an unclassified case policy")]
    fn default_css_token_case_records_formatter_audit_event() {
        let parse = parse_css(
            "@IMPORT \"Keep\";",
            CssFileSource::css(),
            CssParserOptions::default(),
        );
        let syntax = parse.syntax();
        let import_token = syntax
            .descendants_tokens(Direction::Next)
            .find(|token| token.kind() == CssSyntaxKind::IMPORT_KW)
            .unwrap();
        let comments = Comments::from_node(&syntax, &CssCommentStyle, None);
        let context = CssFormatContext::new(CssFormatOptions::default(), comments);
        let mut state = FormatState::new(context);

        {
            let mut buffer = VecBuffer::new(&mut state);
            write!(buffer, [import_token.format()]).unwrap();
        }

        state.assert_no_audit_events();
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "CSS formatter used an unclassified case policy")]
    fn default_css_identifier_case_records_formatter_audit_event() {
        let parse = parse_css(
            "COLOR: red;",
            CssFileSource::css(),
            CssParserOptions::default(),
        );
        let syntax = parse.syntax();
        let identifier = syntax.descendants().find_map(CssIdentifier::cast).unwrap();
        let comments = Comments::from_node(&syntax, &CssCommentStyle, None);
        let context = CssFormatContext::new(CssFormatOptions::default(), comments);
        let mut state = FormatState::new(context);

        {
            let mut buffer = VecBuffer::new(&mut state);
            write!(buffer, [identifier.format()]).unwrap();
        }

        state.assert_no_audit_events();
    }

    #[test]
    fn css_syntax_node_generic_component_identifier_preserves_case() {
        let parse = parse_css(
            ".a { unknown: KeepOne; }",
            CssFileSource::css(),
            CssParserOptions::default(),
        );
        let syntax = parse.syntax();
        let identifier = syntax
            .descendants()
            .filter_map(CssIdentifier::cast)
            .find(|identifier| {
                identifier.syntax().text_trimmed() == "KeepOne"
                    && identifier
                        .parent::<CssGenericComponentValueList>()
                        .is_some()
            })
            .unwrap();

        let formatted = format_node(CssFormatOptions::default(), identifier.syntax()).unwrap();

        assert_eq!(formatted.print().unwrap().as_code(), "KeepOne");
    }

    #[test]
    fn grit_metavariables_preserve_casing() {
        let src = "\u{00b5}Selector { \u{00b5}Declaration; COLOR: \u{00b5}Value; }\n@media \u{00b5}Query { A:HOVER { COLOR: RED; } }";
        let parse = parse_css(
            src,
            CssFileSource::css(),
            CssParserOptions::default().allow_metavariables(),
        );
        assert!(parse.diagnostics().is_empty(), "{:?}", parse.diagnostics());

        let formatted = format_node(CssFormatOptions::default(), &parse.syntax()).unwrap();

        assert_eq!(
            formatted.print().unwrap().as_code(),
            "\u{00b5}Selector {\n\t\u{00b5}Declaration\n\tcolor: \u{00b5}Value;\n}\n@media \u{00b5}Query {\n\tA:hover {\n\t\tcolor: RED;\n\t}\n}\n"
        );
    }

    #[test]
    fn format_node_rule_uses_fmt_node_hook() {
        #[derive(Debug, Clone, Default)]
        struct ProbeRule;

        impl FormatNodeRule<CssRoot> for ProbeRule {
            fn fmt_node(&self, _: &CssRoot, f: &mut CssFormatter) -> FormatResult<()> {
                write!(f, [token("fmt_node")])
            }

            fn fmt_fields(&self, _: &CssRoot, f: &mut CssFormatter) -> FormatResult<()> {
                write!(f, [token("fmt_fields")])
            }

            fn fmt_leading_comments(&self, _: &CssRoot, _: &mut CssFormatter) -> FormatResult<()> {
                Ok(())
            }

            fn fmt_dangling_comments(&self, _: &CssRoot, _: &mut CssFormatter) -> FormatResult<()> {
                Ok(())
            }

            fn fmt_trailing_comments(&self, _: &CssRoot, _: &mut CssFormatter) -> FormatResult<()> {
                Ok(())
            }
        }

        impl FormatRule<CssRoot> for ProbeRule {
            type Context = CssFormatContext;

            fn fmt(&self, node: &CssRoot, f: &mut CssFormatter) -> FormatResult<()> {
                FormatNodeRule::fmt(self, node, f)
            }
        }

        let parse = parse_css("html {}", CssFileSource::css(), CssParserOptions::default());
        let root = CssRoot::cast(parse.syntax().clone()).unwrap();
        let context = CssFormatLanguage::new(CssFormatOptions::default()).create_context(
            root.syntax(),
            None,
            false,
        );

        let formatted =
            biome_formatter::format!(context, [FormatRefWithRule::new(&root, ProbeRule)]).unwrap();

        assert_eq!(formatted.print().unwrap().as_code(), "fmt_node");
    }
}
