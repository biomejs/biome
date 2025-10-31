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

## Phase 2: Parser Implementation ✅ (Infrastructure Complete)

### COMPLETED:
- ✅ Created `biome_glimmer_parser` crate with all necessary infrastructure
  - ✅ `Cargo.toml` with all necessary dependencies
  - ✅ `src/lib.rs` with public `parse_glimmer()` API
  - ✅ `src/lexer/mod.rs` implementing `Lexer<'src>` and `LexerWithCheckpoint<'src>` traits
  - ✅ `src/parser.rs` implementing proper `Parser` trait
  - ✅ `src/token_source.rs` implementing `BumpWithContext` trait
  - ✅ `src/syntax/mod.rs` with parsing rule skeleton
  - ✅ Basic test infrastructure
  - ✅ **ALL COMPILATION ERRORS RESOLVED** - crate builds successfully!

### Lexer Features Implemented:
- ✅ Context-aware lexing with `GlimmerLexContext` enum
- ✅ Text content tokenization
- ✅ Mustache delimiters (`{{`, `}}`, `{{{`, `}}}`)
- ✅ HTML tags (`<`, `>`, `/`)
- ✅ Special characters (`@`, `#`, `.`, `|`, `=`)
- ✅ Keywords (`this`, `as`, `if`, `else`, `each`, `let`, etc.)
- ✅ String and number literals
- ✅ Identifiers and path expressions
- ✅ Comments (HTML and Mustache style)

### TODO (Implementation Details):
- [ ] Complete parser logic for all node types:
  - Basic statement parsing exists but needs full implementation
  - Parse mustache expressions with proper nesting
  - Parse block statements (#if, #each, #let)
  - Parse element/component with attributes and modifiers
  - Parse path expressions (this.foo, @arg, helper)
  - Parse sub-expressions
  - Parse block params (as |item index|)
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

### Phase 1 Completed ✅
1. Created `glimmer_kinds_src.rs` defining all Glimmer tokens, keywords, and nodes
2. Copied `Glimmer.ungram` to `xtask/codegen/` for build system integration
3. Added Glimmer to the `LanguageKind` enum across all codegen files
4. Fixed ungram syntax issues (replaced `+` with `*`, added proper list nodes, fixed optional lists)
5. Added bogus node definitions for error recovery
6. Successfully generated complete AST with 1000+ lines of type-safe Rust code
7. Auto-created `biome_glimmer_factory` crate with factory functions

### Phase 2 Started ⚠️ (Needs Rework)
1. Created `biome_glimmer_parser` crate structure with all modules
2. Implemented context-aware lexer (800+ lines) supporting:
   - Regular template content (text nodes)
   - Inside mustache expressions ({{...}})
   - Inside HTML/component tags (<...>)
   - Attribute values
3. Created token source with lex context management
4. Implemented parser foundation and basic syntax parsing rules
5. Added test infrastructure with spec_test.rs
6. Fixed module organization for biome_glimmer_syntax and biome_glimmer_factory

**Parser Status**: Parser skeleton needs to properly implement Biome's parser traits:
- Lexer should implement `Lexer<'src>` and `LexerWithCheckpoint<'src>` (not `LexerTrait`)
- Token source needs proper `BumpWithContext` implementation
- Parser needs correct `Parser` trait implementation
- Currently has 23 compilation errors related to trait implementations

## Recent Session: Parser Infrastructure Complete ✅

Successfully completed all infrastructure setup and fixed compilation errors:

### Session 1: Grammar & Token Fixes
- ✅ Separated STRING_LITERAL/NUMBER_LITERAL tokens from node types
- ✅ Renamed duplicate token fields (opening/closing, opening_pipe/closing_pipe)
- ✅ Fixed GlimmerBlockStatement to have distinct open/close token names
- ✅ Manually fixed factory T! macro calls to use `T!["{{"]` instead of `T![l_curly2]`

### Session 2: Trait Implementation Rewrite
- ✅ Rewrote lexer to implement `Lexer<'src>` and `LexerWithCheckpoint<'src>` traits
- ✅ Added missing lexer fields: current_kind, current_start, current_flags, unicode_bom_length
- ✅ Fixed token source `BumpWithContext` and `TokenSourceWithBufferedLexer` implementations
- ✅ Fixed checkpoint/rewind functionality with proper generic parameters
- ✅ Replaced non-existent `to_trivia()` with `is_trivia()` check
- ✅ Fixed parser method calls (`p.current()` → `p.cur()`)
- ✅ Added missing `T` macro import

### Final Build Status:
- ✅ `biome_glimmer_syntax`: Builds successfully
- ✅ `biome_glimmer_factory`: Builds successfully
- ✅ `biome_glimmer_parser`: **Builds successfully with zero errors!**

### Commits Made:
1. `b77cbe4534`: Grammar and token handling fixes
2. `02580de7cb`: Status documentation update
3. `432192a57a`: Lexer and token source trait implementations
4. `5242f92ceb`: T macro import fix

## Next Steps

1. ✅ ~~Run codegen to generate full syntax tree~~ **DONE!**
2. ✅ ~~Fix parser trait implementations~~ **DONE!**
3. **Implement complete parser logic** (NEXT):
   - Expand statement parsing beyond skeleton
   - Full mustache/block expression parsing
   - Element/component parsing with attributes
   - Path expression parsing
   - Error recovery with bogus nodes
4. Write comprehensive parser tests with snapshots
5. Integrate with JS parser for GJS/GTS file handling
