# Formatter

The crate `biome_formatter` offers a generic infrastructure to implement a formatting logic for different languages.

The formatting infrastructure of Biome is implemented using traits on syntax nodes. This means that *each node* knows how to format itself.

## Getting started

Let's start creating some plumbing, for our new language called `Html`.

Create a new crate using the command `just new-crate biome_html_formatter`, where `html` is the language you want to format.

The infrastructure of the formatter requires some preliminary code that can't be vendored from `biome_formatter`, due to some constraints of our infrastructure.

Add the following code inside your `lib.rs` file:

<details>

<summary>Code to copy</summary>

```rust

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
    type Format<'a> = T::Format<'a> where Self: 'a;

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
    type Format<'a> = biome_rowan::SyntaxResult<T::Format<'a>> where Self: 'a;

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
    type Format<'a> = Option<T::Format<'a>> where Self: 'a;

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
```
</details>


Then, you'll have to create four types:
1. `HtmlCommentStyle`
1. `HtmlFormatContext`
1. `FormatHtmlSyntaxNode`
1. `HtmlLanguage`

### `HtmlCommentStyle`

The formatter will use this type to get information about the comments of the language.

It's more idiomatic to have `HtmlCommentStyle` inside a file called `comments.rs`.

This type must implement the trait `CommentStyle`.

For brevity, create a public type called `HtmlComments`:

```rust
use biome_formatter::comments::Comments;
use biome_html_syntax::HtmlLanguage;

pub type HtmlComments = Comments<HtmlLanguage>;
```

### `HtmlFormatContext`

The formatter infrastructure allows you to define a `context` that can be mutated during the IR creation phase.

It's more idiomatic to have `HtmlFormatContext` inside a file called `context.rs`.

Usually, the type context must contain `comments` and `source_map` fields:

```rust
pub struct HtmlFormatContext {
    /// The comments of the nodes and tokens in the program.
    comments: Rc<HtmlComments>,
    source_map: Option<TransformSourceMap>,
}

impl HtmlFormatContext {
    pub fn new(comments: HtmlComments) -> Self {
        Self {
            comments: Rc::new(comments),
            source_map: None,
        }
    }

    pub fn with_source_map(mut self, source_map: Option<TransformSourceMap>) -> Self {
        self.source_map = source_map;
        self
    }
}
```

This type needs to implement the traits `FormatContext` and `CstFormatContext`.

### `FormatHtmlSyntaxNode`

This type will instruct the formatter how to format a generic node.

It's more idiomatic to have `FormatHtmlSyntaxNode` inside a file called `cst.rs`.

This is a low level API, it requires just some plumbing. Copy the following code:

<details>

<summary>Low level formatting of CST code.</summary>

```rust
use crate::prelude::*;
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use biome_html_syntax::{map_syntax_node, HtmlSyntaxNode};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatHtmlSyntaxNode;

impl FormatRule<HtmlSyntaxNode> for FormatHtmlSyntaxNode {
    type Context = HtmlFormatContext;

    fn fmt(&self, node: &HtmlSyntaxNode, f: &mut HtmlFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<HtmlFormatContext> for HtmlSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, HtmlSyntaxNode, FormatHtmlSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatHtmlSyntaxNode)
    }
}

impl IntoFormat<HtmlFormatContext> for HtmlSyntaxNode {
    type Format = FormatOwnedWithRule<HtmlSyntaxNode, FormatHtmlSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatHtmlSyntaxNode)
    }
}
```
</details>


### `HtmlLanguage`

This is small type that you need to instruct the formatter infra about a certain language. This type needs to implement the trait `biome_formatter::FormatLanguage`

```rust
impl FormatLanguage for HtmlFormatLanguage {
    type SyntaxLanguage = HtmlLanguage;
    type Context = HtmlFormatContext;
    type FormatRule = FormatHtmlSyntaxNode;
}
```

Then, create a type called `HtmlFormatter`:

```rust
pub(crate) type HtmlFormatter<'buf> = Formatter<'buf, HtmlFormatContext>;
```

The last step is to create a trait that will start the actual formatting:

<details>

<summary>Wire the specific formatting with the `biome_formatter` formatting infra.</summary>

```rust
/// Format a [HtmlSyntaxNode]
pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = HtmlLanguage>,
{
    // this is the method that actually start the formatting
    fn fmt(&self, node: &N, f: &mut HtmlFormatter) -> FormatResult<()> {
        if self.is_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_fields(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    fn fmt_fields(&self, node: &N, f: &mut HtmlFormatter) -> FormatResult<()>;

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
```
</details>


Now that everything is wired, you just needs to expose a public method that does the actual formattings:

```rust
pub fn format_node(
    options: HtmlFormatOptions,
    root: &HtmlSyntaxNode,
) -> FormatResult<Formatted<HtmlFormatContext>> {
    biome_formatter::format_node(root, HtmlFormatLanguage::new(options))
}
```

Since this is a public method, make sure it's appropriately documented.


## Code generation

Considering that we work with traits on syntax nodes, there could be a lot of initial code to start with. No worries, we have command script that generates the initial code for now, starting from the grammar.

```shell
just gen-formatter
```

The initial implementation for the formatting will use the `format_verbatim_node` formatting, which means that the code will be formatted **as is**. From here, you'll have to remove `format_verbatim_node` and use the `biome_formatter` utilities to generate the correct IR.

## Testing

### Plumbing

Inside the `biome_html_formatter` crate, create a folder called `tests`. Inside this folder you have to have a `specs` folder and two files called `spec_test.rs` and `spec_tests.rs` (the names aren't very important though). Create a `language.rs` file too.

Updated the `Cargo.toml` file to import some testing utility:

```toml
[dev-dependencies]
biome_formatter_test = { path = "../biome_formatter_test" }
biome_html_factory     = { path = "../biome_html_factory" }
biome_html_parser      = { path = "../biome_html_parser" }
biome_parser         = { path = "../biome_parser" }
biome_service        = { path = "../biome_service" }
countme              = { workspace = true, features = ["enable"] }
iai                  = "0.1.1"
quickcheck           = { workspace = true }
quickcheck_macros    = { workspace = true }
tests_macros         = { path = "../tests_macros" }
```

Update the `spec_tests.rs` file to look like this:

```rust
mod spec_test;

mod formatter {

    mod html_module {
        tests_macros::gen_tests! {"tests/specs/html/**/*.html", crate::spec_test::run, ""}
    }
}

```

This code will generate a test function for each `html` file found inside `tests/specs/html`. For each test function, it will run the function `spec_test::run`.

Create the function `run` inside the `spec_test.rs` file:

```rust
use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use std::path::Path;

mod language {
    include!("language.rs");
}

pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let root_path = Utf8Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/"));

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path) else {
        return;
    };

    let options = HtmlFormatOptions::default();
    let language = language::HtmlTestFormatLanguage::default();

    let snapshot = SpecSnapshot::new(
        test_file,
        test_directory,
        language,
        HtmlFormatLanguage::new(options),
    );

    snapshot.test()
}
```

Now, let's modify the `language.rs` file:

```rust
use biome_formatter_test::TestFormatLanguage;

#[derive(Default)]
pub struct HtmlTestFormatLanguage {
}

impl TestFormatLanguage for HtmlTestFormatLanguage {

}
```

The `TestFormatLanguage` contains a series of methods that must be implemented.

### Create and running the tests

Now that the plumbing is ready, you just need to create your first `.html` file inside `tests/specs/html`. It's **highly** suggested to create a folder for each kind of test.

Use `cargo t` to run the testing infrastructure. The infrastructure will create a potential snapshot that will show:
- the input;
- the current options applied;
- the formatted input as output;

If the snapshot is correct, use `cargo insta accept`, or use `cargo insta review` to check them one by one and accept or reject them.

If you require testing something using options that aren't the default ones, create a file called `options.json` in the same folder where the `.html` files are. Those options will be applied to **all** files that are the in current folder.

The `options.json` file is a `biome.json` file, so you can use the same options as you were and end-user.
