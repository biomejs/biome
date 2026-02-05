# Formatter

The crate `biome_formatter` offers a generic infrastructure to implement formatting logic for different languages.

The formatting infrastructure of Biome is implemented using traits on syntax nodes. This means that _each node_ knows how to format itself.

## Prerequisites

This guide assumes the parser and grammar for your language are already implemented by following [the guidance](../biome_parser/CONTRIBUTING.md)

The formatter codegen depends on the generated AST types from the grammar, so you must complete the parser and grammar implementation before starting on the formatter.

## Getting started

The recommended way to add a formatter for a new language is to use the provided codegen. This automatically generates the boilerplate code based on the language's grammar. The following steps refer to html as an example but it should be mostly relevant for any language.

### Step 1: Create the formatter crate

First, create the formatter crate structure:

```shell
just new-crate biome_html_formatter
```

This creates `crates/biome_html_formatter/` with the basic Cargo.toml and lib.rs. You'll fill in the implementation details after codegen runs.

### Step 2: Generate boilerplate

Run the codegen script to generate formatter boilerplate:

```shell
just gen-formatter
```

which will

1. Generates boilerplate formatter code in `crates/biome_<language>_formatter/src/`
2. Creates the module structure (`mod.rs` files) automatically
3. Generates default `FormatNodeRule<N>` implementations (initially using `format_verbatim_node`)

### Step 3: Set up the formatter crate structure

After codegen runs, you'll have some auto-generated files, but you'll need to create the folder structure and implement the core types. The complete structure should look like this:

```
crates/biome_<language>_formatter/src/
├── lib.rs                 # Export public API and types (CREATE THIS)
├── context.rs             # FormatContext implementation (CREATE THIS)
├── comments.rs            # Comment handling (CREATE THIS)
├── cst.rs                 # Syntax node formatting (CREATE THIS)
├── prelude.rs             # Re-export common types (CREATE THIS)
├── verbatim.rs            # Verbatim formatting helpers (CREATE THIS)
├── <language>/            # Language-specific formatters (GENERATED)
│   ├── statements/
│   ├── expressions/
│   ├── auxiliary/
│   └── mod.rs
└── generated.rs           # Auto-generated trait implementations (DO NOT EDIT)
```

The codegen creates the `<language>/` folder structure and `generated.rs`. You must manually create the other files (`lib.rs`, `context.rs`, `comments.rs`, `cst.rs`, `prelude.rs`, `verbatim.rs`) and populate them with the required types and traits.

### Step 4: Create the required types

At this point, it's expected that your workspace will have many compile errors. We'll need to do all the plumbing until there are no more compile errors.

#### `CommentStyle`

In `comments.rs`, define how comments are handled:

```rust
use biome_formatter::comments::Comments;
use biome_html_syntax::HtmlLanguage;

pub type HtmlComments = Comments<HtmlLanguage>;
```

#### `FormatContext`

In `context.rs`, define the formatting context that carries state during formatting:

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
    // ... implement required methods
}

impl FormatContext for HtmlFormatContext {
    // ... implement required methods
}

impl CstFormatContext for HtmlFormatContext {
    // ... implement required methods
}
```

#### `FormatSyntaxNode`

In `cst.rs`, define the generic rule for formatting any syntax node:

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

#### `FormatLanguage`

In `lib.rs`, implement the `FormatLanguage` trait that ties everything together:

```rust
pub struct HtmlFormatLanguage {
    options: HtmlFormatOptions,
}

impl FormatLanguage for HtmlFormatLanguage {
    type SyntaxLanguage = HtmlLanguage;
    type Context = HtmlFormatContext;
    type FormatRule = FormatHtmlSyntaxNode;
}

pub(crate) type HtmlFormatter<'buf> = Formatter<'buf, HtmlFormatContext>;
```

And expose the public formatting entry point:

```rust
pub fn format_node(
    options: HtmlFormatOptions,
    root: &HtmlSyntaxNode,
) -> FormatResult<Formatted<HtmlFormatContext>> {
    biome_formatter::format_node(root, HtmlFormatLanguage::new(options))
}
```

### Step 5: Implement node-specific formatters

The generated code initially uses `format_verbatim_node`, which outputs nodes as-is without applying formatting rules. You'll replace it with proper implementation.

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

Each node type gets its own formatter that implements `FormatNodeRule<NodeType>`. The `fmt_fields` method is where you define how to format the node's children.

## How the codegen works

The `.ungram` file is the **source of truth** for your language's syntax structure. When you run `cargo run -p xtask_codegen -- formatter`:

1. **Determine paths**: For each CST node (already generated from `.ungram`), it determines the output path using the node's name:
   - Extracts the **dialect** (language prefix): `HtmlElement` → `Html`
   - Determines the **concept** (node category) by matching suffixes:
     - `HtmlElement` → `Element` concept → `elements/` directory
     - `HtmlAttribute` → `Attribute` concept → `attributes/` directory
     - Generic names → `auxiliary/` directory
   - Converts to snake_case: `HtmlElement` → `element.rs`
2. **Generate files**: Creates formatter files at paths like:
   ```
   crates/biome_html_formatter/src/html/elements/element.rs
   crates/biome_html_formatter/src/html/attributes/attribute.rs
   ```
3. **Generate `mod.rs`**: Creates module files that export all child modules
4. **Generate `generated.rs`**: Creates trait implementations (`AsFormat`, `IntoFormat`, `FormatRule`) for all nodes

### Key traits created by codegen

The generated code creates implementations for:

- **`AsFormat<Context>`**: Allows formatting by reference (`node.format()`)
- **`IntoFormat<Context>`**: Allows formatting by ownership (`node.into_format()`)
- **`FormatNodeRule<N>`**: The trait that defines formatting logic for a specific node type

You only need to implement `fmt_fields()` to define how to format a node's children. Override the other methods only if you need custom behavior.

## Testing

### Set up the test infrastructure

Create a test module in `crates/biome_html_formatter/tests/`:

```rust
// tests/spec_tests.rs
mod spec_test;

mod formatter {
    mod html_module {
        tests_macros::gen_tests! {"tests/specs/html/**/*.html", crate::spec_test::run, ""}
    }
}
```

This will auto-generate a test function for each `.html` file in `tests/specs/html/`.

### Define test language

Create `tests/language.rs`:

```rust
use biome_formatter_test::TestFormatLanguage;

#[derive(Default)]
pub struct HtmlTestFormatLanguage;

impl TestFormatLanguage for HtmlTestFormatLanguage {
    // Implement test-specific behavior if needed
}
```

And `tests/spec_test.rs`:

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

### Create and run tests

Create test files in `tests/specs/html/`:

```html
<!-- tests/specs/html/simple_element.html -->
<div>
  <p>Hello</p>
</div>
```

Run tests:

```shell
cargo t html::simple_element
```

Accept snapshots after verifying they're correct:

```shell
cargo insta accept
```

To use non-default options, create `tests/specs/html/options.json`:

```json
{
  "formatter": {
    "indentWidth": 2
  }
}
```

## Troubleshooting

### "Undefined node" errors during codegen

If you get an error like `Undefined node: AnyHtmlBlock`, it's likely that there's a bug in the grammar. Please refer to [the parser contributing guide](../crates/biome_parser/CONTRIBUTING.md) to troubleshoot.

### Generated code references wrong paths

If generated code has incorrect module paths (e.g., `crate::js::any::` when it should be `crate::html::`), the node name in your `.ungram` file is probably missing the language prefix. Use `HtmlElement` instead of `Element`.
