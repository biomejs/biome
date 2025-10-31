# Quick Reference: Key Integration Points for Glimmer

## System Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     Biome Workspace Service                      │
├─────────────────────────────────────────────────────────────────┤
│  File Handlers (biome_service/file_handlers/)                    │
│  ┌──────────────┬──────────────┬──────────────┬──────────────┐   │
│  │ JavaScript   │ Vue Handler  │ Svelte       │ Glimmer      │   │
│  │              │              │ Handler      │ Handler      │   │
│  │ (plain JS)   │ (extract     │ (extract     │ (extract     │   │
│  │              │  <script>)   │  <script>)   │  <template>) │   │
│  └──────────────┴──────────────┴──────────────┴──────────────┘   │
│                           ↓                                       │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │    Parse with JsFileSource config                       │    │
│  │  - File type (JS, TS, JSX, TSX)                         │    │
│  │  - Module kind (Script, Module)                         │    │
│  │  - Embedding kind (None, Vue, Svelte, Glimmer) ← KEY! │    │
│  └─────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────────┐
│                   biome_js_parser                                │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Lexer (context-aware tokenization)                      │   │
│  │  - JsLexContext::Regular                                 │   │
│  │  - JsLexContext::TemplateElement                         │   │
│  │  - JsLexContext::JsxChild                                │   │
│  │  - JsLexContext::JsxAttributeValue                       │   │
│  └──────────────────────────────────────────────────────────┘   │
│                           ↓                                      │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Parser (JsParser struct)                                │   │
│  │  - Maintains JsParserState                               │   │
│  │  - Checks embedding_kind in JsFileSource                 │   │
│  │  - Calls appropriate parsing functions                   │   │
│  │  - Integration point for Glimmer!                        │   │
│  └──────────────────────────────────────────────────────────┘   │
│                           ↓                                      │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Syntax modules (src/syntax/*.rs)                        │   │
│  │  - program.rs      (entry point: Script vs Module)       │   │
│  │  - module.rs       (ES module parsing)                   │   │
│  │  - stmt.rs         (statements)                          │   │
│  │  - expr.rs         (expressions, calls JSX)              │   │
│  │  - jsx/mod.rs      (JSX parsing - REFERENCE MODEL!)      │   │
│  │  - (glimmer.rs)    (TO BE ADDED)                         │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────────┐
│                   AST & Syntax Nodes                             │
│  - JsScript / JsModule                                           │
│  - Statements, Expressions                                      │
│  - For templates: embedded Glimmer AST                           │
└─────────────────────────────────────────────────────────────────┘
```

## File Type Detection Flow

```
Extension Detection
  ├─ .js         → JsFileSource::js_module()
  ├─ .jsx        → JsFileSource::jsx()
  ├─ .ts         → JsFileSource::ts()
  ├─ .tsx        → JsFileSource::tsx()
  ├─ .gjs ✨     → JsFileSource::gjs()        [embedding_kind: Glimmer]
  └─ .gts ✨     → JsFileSource::gts()        [embedding_kind: Glimmer]
```

## Glimmer-Specific Configuration

```rust
// Already exists in JsFileSource:
pub enum EmbeddingKind {
    Astro,
    Vue,
    Svelte,
    Glimmer,    // ✓ Already here!
    #[default]
    None,
}

// Already exists:
pub fn gjs() -> Self {
    JsFileSource::js_module()
        .with_embedding_kind(EmbeddingKind::Glimmer)
}

pub fn gts() -> Self {
    JsFileSource::ts()
        .with_embedding_kind(EmbeddingKind::Glimmer)
}

// Check in code:
if p.source_type().embedding_kind().is_glimmer() {
    // Handle Glimmer-specific parsing
}
```

## JSX Reference Implementation (for Glimmer pattern)

### Entry Point in expr.rs
```rust
use crate::syntax::jsx::parse_jsx_tag_expression;

if matches!(p.source_type().variant(), LanguageVariant::Jsx) {
    if let Present(jsx_expr) = parse_jsx_tag_expression(p) {
        return Present(jsx_expr);
    }
}
```

### Pattern for Glimmer (suggested):
```rust
use crate::syntax::glimmer::parse_glimmer_template_expression;

if matches!(p.source_type().embedding_kind(), EmbeddingKind::Glimmer) {
    if let Present(template_expr) = parse_glimmer_template_expression(p) {
        return Present(template_expr);
    }
}
```

## Lexer Context Management

### How JSX Uses Contexts:
```
Regular context
  ↓ See JSX tag `<`
JsxChild context (lex template content)
  ↓ See expression start `{`
Back to Regular context
  ↓ After expression `}`
Back to JsxChild context
```

### How Glimmer Could Use Contexts:
```
Regular context
  ↓ See <template> tag
GlimmerChild context (lex template content)
  ↓ See {{
GlimmerMustache context (lex expression)
  ↓ See }}
Back to GlimmerChild context
```

## File Handler Pattern (CURRENT BEST PRACTICE)

```rust
// 1. Extract: Replace Glimmer templates with markers
pub fn extract_js_content(text: &str) -> String {
    // Input:  "export default class { <template>...</template> }"
    // Output: "export default class { __BIOME_GLIMMER_TEMPLATE_0__ }"
}

// 2. Parse: Feed marker-replaced JS to parser
let js = Self::extract_js_content(input);
let parse = parse_js_with_cache(
    &js,
    JsFileSource::gjs().with_embedding_kind(Glimmer),
    options,
    cache
);

// 3. Format: Format the JS AST
let formatted_js = format_node(&parse.syntax(), ...)?;

// 4. Reconstruct: Replace markers with original templates
pub fn output(input: &str, formatted_js: &str) -> String {
    // Replace __BIOME_GLIMMER_TEMPLATE_0__ with original <template>...</template>
}
```

## Key Files to Study

### Understanding the Architecture:
1. **biome_js_parser/src/parse.rs** (200 lines)
   - Entry points: parse(), parse_script(), parse_module()
   - **parse_js_with_offset()** - for embedded content!

2. **biome_js_syntax/src/file_source.rs** (250 lines)
   - JsFileSource structure
   - EmbeddingKind enum
   - LanguageVariant, Language, ModuleKind

3. **biome_js_parser/src/lexer/mod.rs** (50-150 lines)
   - JsLexContext enum definition
   - Context-aware tokenization

### Reference Implementation:
4. **biome_js_parser/src/syntax/jsx/mod.rs** (500+ lines)
   - Complete JSX parsing
   - parse_jsx_tag_expression() function
   - JSX children/text handling

5. **biome_service/src/file_handlers/vue.rs** (150 lines)
   - Pattern for extracting embedded code
   - file_source() with embedding_kind
   - input() / output() methods

### Current Glimmer Handler:
6. **biome_service/src/file_handlers/glimmer.rs** (150+ lines)
   - Already implements extraction
   - Uses marker replacement strategy

## Direct Lines in Code

### Entry point routing (where Glimmer would be checked):
- **biome_js_parser/src/syntax/expr.rs** - Line ~176 (where JSX is parsed)
- **biome_service/src/file_handlers/mod.rs** - EmbeddingKind dispatch

### File source configuration:
- **biome_js_syntax/src/file_source.rs** - Lines 122-144, 221-229

### Lexer contexts available:
- **biome_js_parser/src/lexer/mod.rs** - Lines 53-76

### Parser state available:
- **biome_js_parser/src/state.rs** - Lines 82-150 (JsParserState)

## What Already Works

✅ **Infrastructure that's ready:**
- EmbeddingKind::Glimmer enum
- JsFileSource::gjs() and ::gts() constructors
- GlimmerFileHandler with extraction logic
- .gjs/.gts file type detection
- Parser state management
- Offset-aware parsing for embedded content
- File handler pattern established (Vue, Svelte, Astro)

✅ **Glimmer Parser already implemented:**
- biome_glimmer_parser crate (fully functional)
- biome_glimmer_syntax with AST definitions
- biome_glimmer_factory with node construction

## What's Missing for Full Integration

- [ ] Parse tree integration (combine JS + Glimmer ASTs)
- [ ] File handler wiring in service layer
- [ ] Formatter integration
- [ ] Linter/Analyzer integration
- [ ] CLI support for .gjs/.gts files
- [ ] Comprehensive tests

## Implementation Priority

1. **High Priority** - File handler completion (service layer)
   - Complete glimmer.rs file_source() method
   - Register handler in mod.rs
   - Wire up CLI

2. **Medium Priority** - Parser integration
   - Add Glimmer expression parsing to expr.rs
   - Add Glimmer lexer contexts if needed
   - Test with actual .gjs files

3. **Medium Priority** - Formatter
   - Integrate biome_glimmer_formatter
   - Combine formatting results

4. **Lower Priority** - Linting & Analysis
   - Integrate biome_glimmer_analyzer
   - Cross-module lint rules

