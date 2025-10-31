# Glimmer Support Implementation Status

## Phase 1: Core Infrastructure ✅

### Created Crates:
- ✅ `biome_glimmer_syntax` - Syntax definitions and AST node types
  - Contains `Glimmer.ungram` grammar definition (copied to `xtask/codegen/glimmer.ungram`)
  - **FULLY GENERATED** `generated/` directory with:
    - `kind.rs` - GlimmerSyntaxKind enum with all tokens and nodes
    - `nodes.rs` - Complete AST node type definitions
    - `nodes_mut.rs` - Mutable AST node wrappers
    - `macros.rs` - Helper macros for working with nodes
- ✅ `biome_glimmer_factory` - Factory functions for creating AST nodes
  - **AUTO-GENERATED** by codegen
  - `generated/syntax_factory.rs` - Low-level syntax node creation
  - `generated/node_factory.rs` - High-level typed node creation

### File Type Support:
- ✅ Added `.gjs` and `.gts` extensions to `DocumentFileSource`
- ✅ Added `JsFileSource::gjs()` and `JsFileSource::gts()` methods
- ✅ Integrated with VS Code extension

### Codegen Integration:
- ✅ Created `xtask/codegen/src/glimmer_kinds_src.rs` with token/node definitions
- ✅ Added Glimmer to `LanguageKind` enum in `xtask/codegen/src/language_kind.rs`
- ✅ Added Glimmer support to formatter and syntax_kinds codegen
- ✅ Successfully ran `cargo run -p xtask_codegen -- grammar glimmer`

## Phase 2: Parser Implementation (NEXT)

### TODO:
- [ ] Create `biome_glimmer_parser` crate
  - Add `Cargo.toml` with dependencies
  - Create `src/lib.rs` entry point
  - Create `src/lexer.rs` for tokenization
  - Create `src/parser.rs` for parsing logic
- [ ] Implement lexer for Glimmer tokens:
  - Text content
  - Mustache delimiters (`{{`, `}}`)
  - HTML tags (`<`, `>`, `/`)
  - Special characters (`@`, `#`, `.`, `|`)
  - Keywords (`this`, `as`, `if`, `else`, etc.)
- [ ] Implement parser based on generated AST:
  - Parse GlimmerRoot
  - Parse statements (Text, Mustache, Block, Element, Comments)
  - Parse expressions (Paths, SubExpressions, Literals)
  - Error recovery and bogus node handling
- [ ] Add comprehensive parser tests:
  - Test fixtures in `tests/` directory
  - Snapshot tests using `cargo insta`
  - Edge cases and error scenarios

## Phase 3: JS/TS Integration (NOT STARTED)

### TODO:
- [ ] Modify `biome_js_parser` to detect `<template>` tags
- [ ] Extract template content and parse with Glimmer parser
- [ ] Handle three template contexts:
  - Template in class body
  - Template as variable/export value
  - Template-only file (implicit default export)
- [ ] Create file handlers for `.gjs`/`.gts` files
- [ ] Combine JS and Glimmer parse results

## Phase 4: Formatter Integration (NOT STARTED)

### TODO:
- [ ] Create `biome_glimmer_formatter` crate
- [ ] Format Glimmer template syntax
- [ ] Integrate with JS/TS formatter
- [ ] Preserve boundaries between script and template code

## Phase 5: Linter Integration (NOT STARTED)

### TODO:
- [ ] Create `biome_glimmer_analyzer` crate
- [ ] Implement template-only lint rules
- [ ] Implement hybrid lint rules (JS + template)
- [ ] Component usage validation

## Grammar Reference

The Glimmer grammar is based on the official Glimmer VM AST:
https://github.com/glimmerjs/glimmer-vm

Key AST nodes:
- Template/Block (top-level containers)
- Statements: MustacheStatement, BlockStatement, ElementNode, TextNode, CommentStatement
- Expressions: PathExpression, SubExpression, Literals
- Path heads: ThisHead (`this`), AtHead (`@arg`), VarHead (`variable`)
- Element features: Attributes, ElementModifiers, BlockParams

## Recent Progress Summary (Latest Session)

✅ **Completed:**
1. Created `glimmer_kinds_src.rs` defining all Glimmer tokens, keywords, and nodes
2. Copied `Glimmer.ungram` to `xtask/codegen/` for build system integration
3. Added Glimmer to the `LanguageKind` enum across all codegen files
4. Fixed ungram syntax issues (replaced `+` with `*`, added proper list nodes, fixed optional lists)
5. Added bogus node definitions for error recovery
6. Successfully generated complete AST with 1000+ lines of type-safe Rust code
7. Auto-created `biome_glimmer_factory` crate with factory functions

## Next Steps

1. ✅ ~~Run codegen to generate full syntax tree~~ **DONE!**
2. Create `biome_glimmer_parser` crate skeleton (Cargo.toml, lib.rs, lexer.rs, parser.rs)
3. Implement lexer to tokenize Glimmer template syntax
4. Implement parser to build the AST using generated node types
5. Write comprehensive parser tests with snapshots
6. Integrate with JS parser for GJS/GTS file handling
