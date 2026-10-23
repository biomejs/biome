# Formatter

The `biome_formatter` crate provides the building blocks that every Biome formatter uses. This guide explains how to add a formatter for a new language to Biome.

To use `biome_formatter` with a parser that isn't part of Biome, read the [README](./README.md) instead.

## How the formatter works

Formatting happens in two phases:

1. The formatter walks the syntax tree and builds a **document**. A document is a list of instructions, such as "write this text", "start a group", or "break the line here if the group doesn't fit". Biome calls these instructions the **IR** (intermediate representation).
2. The **printer** reads the document and writes the formatted code. When a group doesn't fit within the line width, the printer breaks its lines.

Each kind of syntax node knows how to format itself. A node implements the `Format` trait, or a separate **rule** type implements `FormatRule` for it, and writes its part of the document.

## Source code types

Every formatter context declares the kind of source code it formats, with the `Source` trait:

```rust
impl FormatContext for HtmlFormatContext {
    type Options = HtmlFormatOptions;
    type Source = RowanSource;
    // ...
}
```

All Biome formatters use `RowanSource`, the source of syntax trees built with `biome_rowan`. With `RowanSource`:

- a position is a `TextSize`, a byte offset in the source text;
- a range is a `TextRange`, a pair of byte offsets;
- the formatted result, `Printed`, reports its positions and ranges with these same types.

Because every Biome formatter uses the same source, the document of one language can be placed inside the document of another. For example, the formatted CSS of an HTML `<style>` tag becomes part of the formatted HTML document.

### The rowan prelude

Import the formatter API from `biome_formatter::rowan::prelude`, not from `biome_formatter::prelude`. The rowan prelude contains everything from the generic prelude. It also replaces the generic IR types, such as `FormatElement`, `Document`, and `Interned`, with versions that use `RowanSource`. This means that you can write `FormatElement` without naming its source type.

### Crate features

`biome_formatter` has the following Cargo features:

| Feature       | Enabled by default | What it adds                                                                                                  |
| ------------- | ------------------ | ------------------------------------------------------------------------------------------------------------- |
| `rowan`       | Yes                | `RowanSource`, the rowan prelude, comment handling, and the entry points that format a `biome_rowan` syntax tree. |
| `diagnostics` | Yes                | Biome diagnostics for formatter errors, and console output for formatter options.                            |
| `deserialize` | Yes                | Reading formatter options from Biome's configuration file.                                                    |
| `serde`       | No                 | Serialization of formatter options and of the formatted result.                                               |
| `schema`      | No                 | JSON Schema for formatter options. Also enables `serde`.                                                      |

Biome crates use the default features, so a new formatter crate doesn't need to enable any feature.

## Prerequisites

This guide assumes that the parser and the grammar of your language already exist. To create them, follow [the parser guide](../biome_parser/CONTRIBUTING.md).

The formatter code generator reads the syntax types generated from the grammar. Finish the parser and the grammar before you start the formatter.

## Getting started

Biome generates most of the boilerplate of a new formatter from the grammar of the language. The following steps use HTML as the example, but they apply to any language.

### Step 1: Create the formatter crate

Create the crate:

```shell
just new-crate biome_html_formatter
```

This command creates `crates/biome_html_formatter/` with a `Cargo.toml` and a `lib.rs`. You fill them in after the next step.

### Step 2: Generate the boilerplate

Run the code generator:

```shell
just gen-formatter
```

The code generator:

1. creates a formatter file for each node of the grammar in `crates/biome_<language>_formatter/src/`;
2. creates the `mod.rs` files that connect these files;
3. creates a `FormatNodeRule<N>` implementation for each node. At first, each implementation prints its node exactly as it's written, with `format_verbatim_node`.

### Step 3: Set up the crate structure

The code generator creates the `<language>/` folder and `generated.rs`. You create the other files yourself. When you're done, the crate looks like this:

```
crates/biome_<language>_formatter/src/
├── lib.rs                 # Public API and core types (you create this file)
├── context.rs             # Formatter context and options (you create this file)
├── comments.rs            # Comment handling (you create this file)
├── cst.rs                 # Rule that formats any syntax node (you create this file)
├── prelude.rs             # Imports shared by all formatter files (you create this file)
├── verbatim.rs            # Helpers that print nodes as written (you create this file)
├── <language>/            # One formatter file per node (generated)
│   ├── statements/
│   ├── expressions/
│   ├── auxiliary/
│   └── mod.rs
└── generated.rs           # Trait implementations for every node (generated, don't edit)
```

### Step 4: Create the required types

At this point, the workspace doesn't compile. The following types connect the generated files with `biome_formatter`. Once they all exist, the crate compiles again.

#### Prelude

In `prelude.rs`, re-export the rowan prelude together with the types of your crate:

```rust
pub(crate) use crate::{
    AsFormat, FormatNodeRule, FormattedIterExt, HtmlFormatContext, HtmlFormatter, verbatim::*,
};
pub(crate) use biome_formatter::rowan::prelude::*;
pub(crate) use biome_rowan::{AstNode, AstNodeList};
```

Every generated formatter file starts with `use crate::prelude::*;`.

#### Comments

In `comments.rs`, describe the comments of your language. The `CommentStyle` trait tells the formatter which comments are line comments and where to place each comment:

```rust
use biome_formatter::comments::{CommentKind, CommentStyle, Comments};
use biome_html_syntax::HtmlLanguage;
use biome_rowan::SyntaxTriviaPieceComments;

pub type HtmlComments = Comments<HtmlLanguage>;

#[derive(Default)]
pub struct HtmlCommentStyle;

impl CommentStyle for HtmlCommentStyle {
    type Language = HtmlLanguage;

    fn get_comment_kind(comment: &SyntaxTriviaPieceComments<HtmlLanguage>) -> CommentKind {
        // Return whether `comment` is a line comment or a block comment.
    }
}
```

#### Context

In `context.rs`, define the context. The context holds the options and the comments while the formatter runs:

```rust
pub struct HtmlFormatContext {
    options: HtmlFormatOptions,
    comments: Rc<HtmlComments>,
    source_map: Option<TransformSourceMap>,
}

impl HtmlFormatContext {
    pub fn new(options: HtmlFormatOptions, comments: HtmlComments) -> Self {
        Self {
            options,
            comments: Rc::new(comments),
            source_map: None,
        }
    }

    pub fn with_source_map(mut self, source_map: Option<TransformSourceMap>) -> Self {
        self.source_map = source_map;
        self
    }
}

impl FormatOptions for HtmlFormatOptions {
    // The indentation, line width, and line ending used by the printer.
}

impl FormatContext for HtmlFormatContext {
    type Options = HtmlFormatOptions;
    type Source = RowanSource;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        self.source_map.as_ref()
    }
}

impl CstFormatContext for HtmlFormatContext {
    type Language = HtmlLanguage;
    type Style = HtmlCommentStyle;
    type CommentRule = FormatHtmlComment;

    fn comments(&self) -> &HtmlComments {
        &self.comments
    }
}
```

The `source_map` field is `None` unless your formatter changes the syntax tree before formatting it. Biome's JavaScript formatter is an example: it removes parentheses from the tree. Most formatters don't change the tree.

#### Syntax node rule

In `cst.rs`, define the rule that formats any syntax node. It finds the type of the node and calls the formatter of that type:

```rust
use crate::prelude::*;
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use biome_html_syntax::{HtmlSyntaxNode, map_syntax_node};

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

#### Language

In `lib.rs`, implement `FormatLanguage`. This trait connects the syntax tree, the context, and the syntax node rule:

```rust
pub struct HtmlFormatLanguage {
    options: HtmlFormatOptions,
}

impl FormatLanguage for HtmlFormatLanguage {
    type SyntaxLanguage = HtmlLanguage;
    type Context = HtmlFormatContext;
    type FormatRule = FormatHtmlSyntaxNode;

    fn options(&self) -> &HtmlFormatOptions {
        &self.options
    }

    fn create_context(
        self,
        root: &HtmlSyntaxNode,
        source_map: Option<TransformSourceMap>,
        _delegate_fmt_embedded_nodes: bool,
    ) -> HtmlFormatContext {
        let comments = Comments::from_node(root, &HtmlCommentStyle, source_map.as_ref());
        HtmlFormatContext::new(self.options, comments).with_source_map(source_map)
    }
}

pub(crate) type HtmlFormatter<'buf> = Formatter<'buf, HtmlFormatContext>;
```

Then add the public function that formats a file:

```rust
pub fn format_node(
    options: HtmlFormatOptions,
    root: &HtmlSyntaxNode,
) -> FormatResult<Formatted<HtmlFormatContext>> {
    biome_formatter::format_node(root, HtmlFormatLanguage::new(options), false)
}
```

The last argument of `biome_formatter::format_node` tells the formatter whether the caller formats code written in other languages separately. Pass `false` until your language supports such code. The section [Code written in another language](#code-written-in-another-language) explains how.

### Step 5: Format each node

The generated formatters print each node exactly as it's written. Replace this with real formatting, one node at a time:

```rust
use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FormatHtmlElement;

impl FormatNodeRule<HtmlElement> for FormatHtmlElement {
    fn fmt_fields(&self, node: &HtmlElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
```

Each node type has its own formatter, which implements `FormatNodeRule<NodeType>`. The `fmt_fields` method writes the document for the children of the node. The formatter writes the comments around the node for you.

## Code written in another language

Some languages contain code written in another language. For example, an HTML `<script>` tag contains JavaScript. Biome formats this code with the formatter of its own language, in two passes:

1. Your formatter formats the code itself and wraps the result in a `FormatEmbedded` element. The element identifies the code by its range in the source text.
2. The caller calls `Formatted::format_embedded`. For each range that it knows, it formats the code with the formatter of its language and replaces the content of the element.

When the caller doesn't replace an element, the element prints the output of your formatter. This means that the file is still formatted when the other formatter fails.

```rust
fn fmt_fields(&self, node: &HtmlEmbeddedContent, f: &mut HtmlFormatter) -> FormatResult<()> {
    let range = node.range();
    // Your context stores the ranges that the caller formats separately.
    if f.context().is_embedded_node_range(range) {
        let content = format_with(|f| write!(f, [format_verbatim_node(node.syntax())]));
        let embedded = FormatEmbedded::new(range, &content, f)?;
        // The replacement doesn't end with a line break, so write it here.
        write!(f, [hard_line_break(), embedded, hard_line_break()])
    } else {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
```

## How the code generator works

The `.ungram` file is the source of truth for the syntax of your language. When you run `just gen-formatter`, the code generator:

1. **Chooses a path for each node.** It uses the name of the node, which comes from the `.ungram` file:
   - It reads the **dialect**, the language prefix: `HtmlElement` → `Html`.
   - It reads the **concept**, the category of the node, from the end of the name:
     - `HtmlElement` → `Element` → `elements/` folder
     - `HtmlAttribute` → `Attribute` → `attributes/` folder
     - Other names → `auxiliary/` folder
   - It converts the rest of the name to snake case: `HtmlElement` → `element.rs`.
2. **Creates the formatter files** at these paths:
   ```
   crates/biome_html_formatter/src/html/elements/element.rs
   crates/biome_html_formatter/src/html/attributes/attribute.rs
   ```
3. **Creates the `mod.rs` files** that export the formatter files.
4. **Creates `generated.rs`**, which implements `AsFormat`, `IntoFormat`, and `FormatRule` for every node.

### Traits implemented by the generated code

- **`AsFormat<Context>`** formats a node by reference: `node.format()`.
- **`IntoFormat<Context>`** formats a node that you own: `node.into_format()`.
- **`FormatNodeRule<N>`** contains the formatting logic of one node type.

You only implement `fmt_fields()`. Override the other methods of `FormatNodeRule` only when a node needs different behavior, such as custom comment placement.

## Testing

Formatter tests are snapshot tests. Each test formats an input file and saves the result next to it in a `.snap` file. When the output changes, the test fails until you accept the new snapshot.

### Set up the tests

Create `tests/spec_tests.rs`. It generates one test for each file that matches the pattern:

```rust
mod spec_test;

mod formatter {
    mod html {
        tests_macros::gen_tests! {"tests/specs/html/**/*.html", crate::spec_test::run, ""}
    }
}
```

Then create `tests/spec_test.rs`. The test formats the file through Biome's workspace, with the configuration that you pass:

```rust
use biome_configuration::Configuration;
use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use camino::Utf8Path;

pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let root_path = Utf8Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/html"));

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path) else {
        panic!("Failed to set up snapshot test");
    };

    let snapshot = SpecSnapshot::new(test_file, test_directory, Configuration::default());
    snapshot.test()
}
```

Because the tests run through the workspace, your language must be registered in `biome_service` first. The tests of an existing formatter, such as `crates/biome_html_formatter/tests/`, show a complete setup.

### Write and run tests

Add an input file to `tests/specs/html/`:

```html
<!-- tests/specs/html/simple_element.html -->
<div>
  <p>Hello</p>
</div>
```

Run the tests of the crate:

```shell
cargo test -p biome_html_formatter
```

The first run creates `simple_element.html.snap`, which contains the formatted output. Read it, and accept it when the output is correct:

```shell
cargo insta accept
```

To test different options, add an `options.json` file to the folder of the input files. It uses the format of the Biome configuration file and applies to every input file in that folder:

```json
{
  "formatter": {
    "indentWidth": 2
  }
}
```

## Troubleshooting

### The code generator reports an undefined node

An error such as `Undefined node: AnyHtmlBlock` usually means that the grammar has a mistake. To find it, read [the parser guide](../biome_parser/CONTRIBUTING.md).

### The generated code uses the wrong module paths

When the generated code refers to the wrong language, such as `crate::js::any::` instead of `crate::html::`, a node name in the `.ungram` file is probably missing its language prefix. Name the node `HtmlElement`, not `Element`.
