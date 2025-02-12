//! Biome's official JavaScript formatter.
//!
//! ## Implement the formatter
//!
//! Our formatter is node based. Meaning that each AST node knows how to format itself. In order to implement
//! the formatting, a node has to implement the trait `FormatNode`.
//!
//! `biome` has an automatic code generation that creates automatically the files out of the grammar.
//! By default, all implementations will format verbatim,
//! meaning that the formatter will print tokens and trivia as they are (`format_verbatim`).
//!
//! Our formatter has its own [internal IR](https://en.wikipedia.org/wiki/Intermediate_representation), it creates its own abstraction from an AST.
//!
//! The developer won't be creating directly this IR, but they will use a series of utilities that will help
//! to create this IR. The whole IR is represented by the `enum` `FormatElement`.
//!
//! ### Best Practices
//!
//! 1. Use the `*Fields` struct to extract all the tokens/nodes
//!     ```rust,ignore
//!     #[derive(Debug, Clone, Default)]
//!     pub struct FormatJsExportDefaultExpressionClause;
//!
//!     impl FormatNodeRule<JsExportDefaultExpressionClause> for FormatJsExportDefaultExpressionClauses {
//!         fn fmt_fields(&self, node: &JsExportDefaultExpressionClause, f: &mut JsFormatter) -> FormatResult<()> {
//!             let JsExportDefaultExpressionClauseFields {
//!                 default_token,
//!                 expression,
//!                 semicolon_token,
//!             }  = node.as_fields();
//!        }
//!     }
//!     ```
//! 2. When using `.as_fields()` with the destructuring, don't use the `..` feature. Prefer extracting all fields and ignore them
//!    using the `_`
//!    ```rust,ignore
//!    #[derive(Debug, Clone, Default)]
//!    pub struct FormatJsExportDefaultExpressionClause;
//!
//!    impl FormatNodeRule<JsExportDefaultExpressionClause> for FormatJsExportDefaultExpressionClauses {
//!        fn fmt_fields(&self, node: &JsExportDefaultExpressionClause, f: &mut JsFormatter) -> FormatResult<()> {
//!             let JsExportDefaultExpressionClauseFields {
//!                 default_token,
//!                 expression: _,
//!                 semicolon_token
//!             } = node.as_fields();
//!         }
//!    }
//!    ```
//!    The reason why we want to promote this pattern is because we want to make explicit when a token/node is excluded;
//! 3. Use the APIs provided by `builders.rs`, `formatter` and `format_extensions.rs`.
//!    1. `builders.rs` exposes a series of utilities to craft the formatter IR; please refer to their internal
//!         documentation to understand what the utilities are for;
//!    2. `formatter` exposes a set of functions to help to format some recurring patterns; please refer to their internal
//!         documentation to understand how to use them and when;
//!    3. `format_extensions.rs`: with these traits, we give the ability to nodes and tokens to implements certain methods
//!         that are exposed based on its type. If you have a good IDE support, this feature will help you. For example:
//!
//!    ```rust,ignore
//!    #[derive(Debug, Clone, Default)]
//!    pub struct FormatJsExportDefaultExpressionClause;
//!
//!    impl FormatNodeRule<JsExportDefaultExpressionClause> for FormatJsExportDefaultExpressionClauses{
//!         fn fmt_fields(&self, node: &JsExportDefaultExpressionClause, f: &mut JsFormatter) -> FormatResult<()> {
//!             let JsExportDefaultExpressionClauseFields {
//!                 default_token,
//!                 expression, // it's a mandatory node
//!                 semicolon_token, // this is not a mandatory node
//!             } = node.as_fields();
//!             let element = expression.format();
//!
//!             if let Some(expression) = &expression? {
//!                 write!(f, [expression.format(), space()])?;
//!             }
//!
//!             if let Some(semicolon) = &semicolon_token {
//!                 write!(f, [semicolon.format()])?;
//!             } else {
//!                 write!(f, [space()])?;
//!             }
//!         }
//!    }
//!    ```
//!
//! 4. Use the [playground](https://biomejs.dev/playground/) to inspect the code that you want to format.
//!     It helps you to understand which nodes need to be implemented/modified
//!     in order to implement formatting. Alternatively, you can locally run the playground by following
//!     the [playground instructions](https://github.com/biomejs/biome/blob/main/website/README.md).
//! 5. Use the `quick_test.rs` file in `tests/` directory.
//!     Function to test you snippet straight from your IDE, without running the whole test suite. The test is ignored on purpose, so you won't need to worry about the CI breaking.
//!
//! ## Testing
//!
//! We use [insta.rs](https://insta.rs/docs) for our snapshot tests, please make sure you read its documentation to learn the basics of snapshot testing.
//! You should install the companion [`cargo-insta`](https://insta.rs/docs/cli/) command to assist with snapshot reviewing.
//!
//! Directories are divided by language, so when creating a new test file, make sure to have the correct file
//! under the correct folder:
//! - `JavaScript` => `js/` directory
//! - `TypeScript` => `ts/` directory
//! - `JSX` => `jsx/` directory
//! - `TSX` => `ts/` directory
//!
//! To create a new snapshot test for JavaScript, create a new file to `crates/biome_js_formatter/tests/specs/js/`, e.g. `arrow_with_spaces.js`
//!
//! ```javascript
//! const foo     = ()    => {
//!     return bar
//! }
//! ```
//!
//! Files processed as modules must go inside the `module/` directory, files processed as script must go inside the
//! `script/` directory.
//!
//! Run the following command to generate the new snapshot (the snapshot tests are generated by a procedure macro so we need to recompile the tests):
//!
//! ```bash
//! touch crates/biome_js_formatter/tests/spec_tests.rs && cargo test -p biome_js_formatter formatter
//! ```
//!
//! For better test driven development flow, start the formatter tests with [`cargo-watch`](https://crates.io/crates/cargo-watch):
//!
//! ```bash
//! cargo watch -i '*.new' -x 'test -p biome_js_formatter formatter'
//! ```
//!
//! After test execution, you will get a new `arrow.js.snap.new` file.
//!
//! To actually update the snapshot, run `cargo insta review` to interactively review and accept the pending snapshot. `arrow.js.snap.new` will be replaced with `arrow.js.snap`
//!
//! Sometimes, you need to verify the formatting for different cases/options. In order to do that, create a folder with
//! the cases you need to verify. If we needed to follow the previous example:
//!
//! 1. create a folder called `arrow_with_spaces/` and move the JS file there;
//! 2. then create a file called `options.json`
//! 3. The content would be something like:
//!     ```json
//!     {
//!         "cases": [
//!             {
//!                 "line_width": 120,
//!                 "indent_style": {"Space": 4}
//!             }
//!         ]
//!     }
//!     ````
//! 4. the `cases` keyword is mandatory;
//! 5. then each object of the array will contain the matrix of options you'd want to test.
//!    In this case the test suite will run a **second test case** with `line_width` to 120 and `ident_style` with  4 spaces
//! 6. when the test suite is run, you will have two outputs in your snapshot: the default one and the custom one
//!
//! ### Debugging Test Failures
//!
//! There are four cases when a test is not correct:
//! - you try to print/format the same token multiple times; the formatter will check at runtime when a test is run;
//! - some tokens haven't been printed; usually you will have this information inside the snapshot, under a section
//!     called `"Unimplemented tokens/nodes"`; a test, in order to be valid, can't have that section;
//!
//! If removing a token is the actual behaviour (removing some parenthesis or a semicolon), then the correct way
//! to do it by using the formatter API [biome_formatter::trivia::format_removed];
//! - the emitted code is not a valid program anymore, the test suite will parse again the emitted code and it will
//!     fail if there are syntax errors;
//! - the emitted code, when formatted again, differs from the original; this usually happens when removing/adding new
//!     elements, and the grouping is not correctly set;

mod cst;
mod js;
mod jsx;
mod prelude;
mod ts;
pub mod utils;

#[rustfmt::skip]
mod generated;
pub mod comments;
pub mod context;
mod parentheses;
pub(crate) mod separated;
mod syntax_rewriter;

use biome_formatter::format_element::tag::Label;
use biome_formatter::prelude::*;
use biome_formatter::{
    comments::Comments, write, CstFormatContext, Format, FormatLanguage, FormatToken,
    TransformSourceMap,
};
use biome_formatter::{Buffer, FormatOwnedWithRule, FormatRefWithRule, Formatted, Printed};
use biome_js_syntax::{
    AnyJsDeclaration, AnyJsStatement, JsLanguage, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
};
use biome_rowan::TextRange;
use biome_rowan::{AstNode, SyntaxNode};

use crate::comments::JsCommentStyle;
use crate::context::{JsFormatContext, JsFormatOptions};
use crate::cst::FormatJsSyntaxNode;
use crate::syntax_rewriter::transform;

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

pub(crate) type JsFormatter<'buf> = Formatter<'buf, JsFormatContext>;

/// Rule for formatting a JavaScript [AstNode].
pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = JsLanguage>,
{
    fn fmt(&self, node: &N, f: &mut JsFormatter) -> FormatResult<()> {
        if self.is_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_node(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    /// Formats the node without comments. Ignores any suppression comments.
    fn fmt_node(&self, node: &N, f: &mut JsFormatter) -> FormatResult<()> {
        let needs_parentheses = self.needs_parentheses(node);

        if needs_parentheses {
            write!(f, [text("(")])?;
        }

        self.fmt_fields(node, f)?;

        if needs_parentheses {
            write!(f, [text(")")])?;
        }

        Ok(())
    }

    /// Formats the node's fields.
    fn fmt_fields(&self, item: &N, f: &mut JsFormatter) -> FormatResult<()>;

    /// Returns whether the node requires parens.
    fn needs_parentheses(&self, item: &N) -> bool {
        let _ = item;
        false
    }

    /// Returns `true` if the node has a suppression comment and should use the same formatting as in the source document.
    fn is_suppressed(&self, node: &N, f: &JsFormatter) -> bool {
        f.context().comments().is_suppressed(node.syntax())
    }

    /// Formats the [leading comments](biome_formatter::comments#leading-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the leading comments.
    fn fmt_leading_comments(&self, node: &N, f: &mut JsFormatter) -> FormatResult<()> {
        format_leading_comments(node.syntax()).fmt(f)
    }

    /// Formats the [dangling comments](biome_formatter::comments#dangling-comments) of the node.
    ///
    /// You should override this method if the node handled by this rule can have dangling comments because the
    /// default implementation formats the dangling comments at the end of the node, which isn't ideal but ensures that
    /// no comments are dropped.
    ///
    /// A node can have dangling comments if all its children are tokens or if all node childrens are optional.
    fn fmt_dangling_comments(&self, node: &N, f: &mut JsFormatter) -> FormatResult<()> {
        format_dangling_comments(node.syntax())
            .with_soft_block_indent()
            .fmt(f)
    }

    /// Formats the [trailing comments](biome_formatter::comments#trailing-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the trailing comments.
    fn fmt_trailing_comments(&self, node: &N, f: &mut JsFormatter) -> FormatResult<()> {
        format_trailing_comments(node.syntax()).fmt(f)
    }
}

/// Rule for formatting an bogus node.
pub(crate) trait FormatBogusNodeRule<N>
where
    N: AstNode<Language = JsLanguage>,
{
    fn fmt(&self, node: &N, f: &mut JsFormatter) -> FormatResult<()> {
        format_bogus_node(node.syntax()).fmt(f)
    }
}

/// Format implementation specific to JavaScript tokens.
pub(crate) type FormatJsSyntaxToken = FormatToken<JsFormatContext>;

impl AsFormat<JsFormatContext> for JsSyntaxToken {
    type Format<'a> = FormatRefWithRule<'a, JsSyntaxToken, FormatJsSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatJsSyntaxToken::default())
    }
}

impl IntoFormat<JsFormatContext> for JsSyntaxToken {
    type Format = FormatOwnedWithRule<JsSyntaxToken, FormatJsSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatJsSyntaxToken::default())
    }
}

#[derive(Debug, Clone)]
pub struct JsFormatLanguage {
    options: JsFormatOptions,
}
impl JsFormatLanguage {
    pub fn new(options: JsFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for JsFormatLanguage {
    type SyntaxLanguage = JsLanguage;
    type Context = JsFormatContext;
    type FormatRule = FormatJsSyntaxNode;

    fn transform(
        &self,
        root: &SyntaxNode<Self::SyntaxLanguage>,
    ) -> Option<(SyntaxNode<Self::SyntaxLanguage>, TransformSourceMap)> {
        Some(transform(root.clone()))
    }

    fn is_range_formatting_node(&self, node: &JsSyntaxNode) -> bool {
        let kind = node.kind();

        // Do not format variable declaration nodes, format the whole statement instead
        if matches!(kind, JsSyntaxKind::JS_VARIABLE_DECLARATION) {
            return false;
        }

        AnyJsStatement::can_cast(kind)
            || AnyJsDeclaration::can_cast(kind)
            || matches!(
                kind,
                JsSyntaxKind::JS_DIRECTIVE | JsSyntaxKind::JS_EXPORT | JsSyntaxKind::JS_IMPORT
            )
    }

    fn options(&self) -> &JsFormatOptions {
        &self.options
    }

    fn create_context(
        self,
        root: &JsSyntaxNode,
        source_map: Option<TransformSourceMap>,
    ) -> Self::Context {
        let comments = Comments::from_node(root, &JsCommentStyle, source_map.as_ref());
        JsFormatContext::new(self.options, comments).with_source_map(source_map)
    }
}

/// Formats a range within a file, supported by Biome
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [JsFormatContext], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Formatted] result with a range corresponding to the
/// range of the input that was effectively overwritten by the formatter
pub fn format_range(
    options: JsFormatOptions,
    root: &JsSyntaxNode,
    range: TextRange,
) -> FormatResult<Printed> {
    biome_formatter::format_range(root, range, JsFormatLanguage::new(options))
}

/// Formats a JavaScript (and its super languages) file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format_node(
    options: JsFormatOptions,
    root: &JsSyntaxNode,
) -> FormatResult<Formatted<JsFormatContext>> {
    biome_formatter::format_node(root, JsFormatLanguage::new(options))
}

/// Formats a single node within a file, supported by Biome.
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [JsFormatContext], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Formatted] result
pub fn format_sub_tree(options: JsFormatOptions, root: &JsSyntaxNode) -> FormatResult<Printed> {
    biome_formatter::format_sub_tree(root, JsFormatLanguage::new(options))
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum JsLabels {
    MemberChain,
}

impl Label for JsLabels {
    fn id(&self) -> u64 {
        *self as u64
    }

    fn debug_name(&self) -> &'static str {
        match self {
            JsLabels::MemberChain => "MemberChain",
        }
    }
}

#[cfg(test)]
mod tests {

    use super::format_range;

    use crate::context::JsFormatOptions;
    use biome_formatter::IndentStyle;
    use biome_js_parser::{parse, parse_script, JsParserOptions};
    use biome_js_syntax::JsFileSource;
    use biome_rowan::{TextRange, TextSize};

    #[test]
    fn test_range_formatting() {
        let input = "
while(
    true
) {
    function func() {
    func(     /* comment */
    );

    let array =
        [ 1
    , 2];

    }

    function func2()
    {

    const no_format    =    () => {};

    }
}
";

        // Start the formatting range two characters before the "let" keywords,
        // in the middle of the indentation whitespace for the line
        let range_start = TextSize::try_from(input.find("let").unwrap() - 2).unwrap();
        let range_end = TextSize::try_from(input.find("const").unwrap()).unwrap();

        let tree = parse_script(input, JsParserOptions::default());
        let result = format_range(
            JsFormatOptions::new(JsFileSource::js_script())
                .with_indent_style(IndentStyle::Space)
                .with_indent_width(4.try_into().unwrap()),
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        assert_eq!(
            result.as_code(),
            "function func() {\n        func(/* comment */);\n\n        let array = [1, 2];\n    }\n\n    function func2() {\n        const no_format = () => {};\n    }"
        );
        assert_eq!(
            result.range(),
            Some(TextRange::new(
                range_start - TextSize::from(56),
                range_end + TextSize::from(40)
            ))
        );
    }

    #[test]
    fn test_range_formatting_indentation() {
        let input = "
function() {
         const veryLongIdentifierToCauseALineBreak = { veryLongKeyToCauseALineBreak: 'veryLongValueToCauseALineBreak' }
}
";

        let range_start = TextSize::try_from(input.find("const").unwrap()).unwrap();
        let range_end = TextSize::try_from(input.find('}').unwrap()).unwrap();

        let tree = parse_script(input, JsParserOptions::default());
        let result = format_range(
            JsFormatOptions::new(JsFileSource::js_script())
                .with_indent_style(IndentStyle::Space)
                .with_indent_width(4.try_into().unwrap()),
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        // As a result of the indentation normalization, the number of spaces within
        // the object expression is currently rounded down from an odd indentation level
        assert_eq!(
            result.as_code(),
            "const veryLongIdentifierToCauseALineBreak = {\n            veryLongKeyToCauseALineBreak: \"veryLongValueToCauseALineBreak\",\n        };"
        );
        assert_eq!(
            result.range(),
            Some(TextRange::new(range_start, range_end + TextSize::from(1)))
        );
    }

    #[test]
    fn test_range_formatting_whitespace() {
        let input = "               ";

        let range_start = TextSize::from(5);
        let range_end = TextSize::from(5);

        let tree = parse_script(input, JsParserOptions::default());
        let result = format_range(
            JsFormatOptions::new(JsFileSource::js_script())
                .with_indent_style(IndentStyle::Space)
                .with_indent_width(4.try_into().unwrap()),
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        assert_eq!(result.as_code(), "");
        assert_eq!(result.range(), Some(TextRange::new(range_start, range_end)));
    }

    #[test]
    fn test_range_formatting_middle_of_token() {
        let input = r#"/* */ function Foo(){
/**/
}
"#;

        let range = TextRange::new(TextSize::from(16), TextSize::from(28));

        debug_assert_eq!(
            &input[range],
            r#"oo(){
/**/
}"#
        );

        let tree = parse_script(input, JsParserOptions::default());
        let result = format_range(
            JsFormatOptions::new(JsFileSource::js_script())
                .with_indent_style(IndentStyle::Space)
                .with_indent_width(4.try_into().unwrap()),
            &tree.syntax(),
            range,
        )
        .expect("Range formatting failed");

        assert_eq!(
            result.as_code(),
            r#"/* */ function Foo() {
    /**/
}"#
        );
        assert_eq!(
            result.range(),
            Some(TextRange::new(TextSize::from(0), TextSize::from(28)))
        )
    }

    #[test]
    fn range_formatting_trailing_comments() {
        let input = r#"let fn =a((x ) => {
          quux (); //
        });
"#;

        let range = TextRange::new(TextSize::from(28), TextSize::from(41));

        debug_assert_eq!(&input[range], r#"  quux (); //"#);

        let tree = parse_script(input, JsParserOptions::default());
        let result = format_range(
            JsFormatOptions::new(JsFileSource::js_script())
                .with_indent_style(IndentStyle::Space)
                .with_indent_width(4.try_into().unwrap()),
            &tree.syntax(),
            range,
        )
        .expect("Range formatting failed");

        assert_eq!(result.as_code(), r#"quux(); //"#);
        assert_eq!(
            result.range(),
            Some(TextRange::new(TextSize::from(30), TextSize::from(41)))
        )
    }

    #[test]
    fn format_range_out_of_bounds() {
        let src = "statement();";

        let syntax = JsFileSource::js_module();
        let tree = parse(src, syntax, JsParserOptions::default());

        let result = format_range(
            JsFormatOptions::new(syntax),
            &tree.syntax(),
            TextRange::new(TextSize::from(0), TextSize::of(src) + TextSize::from(5)),
        );

        assert!(result.is_err());
    }
}
