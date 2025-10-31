# Biome JavaScript Parser Architecture Analysis

## Overview

The biome_js_parser is an extremely fast, lossless, error-tolerant JavaScript/TypeScript parser that forms the foundation for Biome's language support. This document explores its structure, mechanisms for handling embedded content (JSX, template literals), and potential integration points for Glimmer support.

## 1. Entry Points and Parsing Flow

### Main Entry Points (crates/biome_js_parser/src/parse.rs)

The parser exposes several high-level APIs:

1. **`parse_script(text, options)`** - Parse JS as a script (global context)
   - Returns `Parse<JsScript>`
   - Calls `parse()` with `ModuleKind::Script`

2. **`parse_module(text, options)`** - Parse JS as an ES module
   - Returns `Parse<JsModule>`
   - Calls `parse()` with `ModuleKind::Module`

3. **`parse(text, source_type, options)`** - Generic parsing with full source type control
   - Returns `Parse<AnyJsRoot>` (can be cast to either Script or Module)
   - Takes `JsFileSource` parameter (specifies language: JS/TS/JSX/TSX, variant, version, **embedding_kind**)

4. **`parse_js_with_cache(..., cache)`** - Reusable cache version for performance

5. **`parse_js_with_offset(text, base_offset, source_type, options)`** - **For embedded content!**
   - Returns `JsOffsetParse` with offset-aware ranges
   - Perfect for parsing JS inside other languages (HTML script tags, Vue <script>, etc.)
   - All text ranges in AST are offset by base_offset

6. **`parse_js_with_offset_and_cache(..., cache)`** - Offset parsing with cache

### Parsing Flow

```
parse_js_with_cache()
  ↓
parse_common(text, source_type, options)  [lines 142-153]
  ├─ Creates JsParser with text, source_type, and options
  ├─ Calls syntax::program::parse(&mut parser)  [entry into syntax parsing]
  ├─ Calls parser.finish() → (Vec<Event>, Vec<Trivia>, Vec<ParseDiagnostic>)
  ↓
Creates JsLosslessTreeSink
  ↓
biome_parser::event::process(&mut tree_sink, events, errors)
  ↓
Returns Parse<AnyJsRoot> with syntax tree and diagnostics
```

## 2. Core Parser Structure

### JsParser (crates/biome_js_parser/src/parser.rs)

```rust
pub struct JsParser<'source> {
    pub(super) state: JsParserState,          // Contextual state machine
    pub source_type: JsFileSource,             // File configuration
    context: ParserContext<JsSyntaxKind>,     // Biome parser infrastructure
    source: JsTokenSource<'source>,            // Token stream
    options: JsParserOptions,                  // Parser options
}
```

**Key methods:**
- `new(source, source_type, options)` - Create parser instance
- `state()` / `state_mut()` - Access parser state
- `source_type()` - Query source type configuration
- `re_lex(context)` - Re-lex token in different context
- `lookahead(op)` - Parse speculatively without consuming tokens
- `with_state(change, func)` - Temporarily modify parser state
- `checkpoint()` / `rewind()` - Save and restore parser position

### JsFileSource Configuration (crates/biome_js_syntax/src/file_source.rs)

```rust
pub struct JsFileSource {
    language: Language,               // JS or TypeScript
    variant: LanguageVariant,         // Standard, StandardRestricted, or Jsx
    module_kind: ModuleKind,          // Script or Module
    version: LanguageVersion,         // ES2022, ESNext
    embedding_kind: EmbeddingKind,    // ⭐ CRITICAL FOR EMBEDDED CONTENT
}

pub enum EmbeddingKind {
    Astro,
    Vue,
    Svelte,
    Glimmer,    // ← Already defined!
    #[default]
    None,
}
```

**Helper constructors:**
- `JsFileSource::js_module()` - Plain JS as module
- `JsFileSource::jsx()` - JS with JSX support
- `JsFileSource::ts()` - TypeScript
- `JsFileSource::tsx()` - TypeScript with JSX
- `JsFileSource::gjs()` - **GJS (JS + Glimmer)**
- `JsFileSource::gts()` - **GTS (TS + Glimmer)**
- `.with_embedding_kind(EmbeddingKind::Glimmer)`

## 3. Special Content Handling Mechanisms

### A. Lexical Contexts (JsLexContext Enum)

The lexer operates in different contexts to tokenize differently based on content type:

```rust
pub enum JsLexContext {
    #[default]
    Regular,                              // Standard JS tokenization

    TemplateElement { tagged: bool },    // Inside `${...}` in template literals
                                         // Doesn't skip whitespace trivia

    JsxChild,                            // Inside JSX element children
                                         // Returns: whitespace, JSX text, <, {, or EOF

    JsxAttributeValue,                   // Inside JSX attribute values
                                         // Allows quoted strings like "value"
}
```

**How it works:**
1. Lexer starts in `Regular` context
2. When parser encounters JSX tag start `<`, it switches lexer to `JsxChild` context
3. Lexer returns JSX-aware tokens (JsxText, JsxExpressionStart `{`, tag delimiters)
4. When parser enters JSX expression `{...}`, it switches back to `Regular` context
5. Re-lexing via `JsReLexContext::JsxChild` for additional processing

**For Glimmer:** We could similarly add:
```rust
JsLexContext::GlimmerChild,          // Inside Glimmer <template>
JsLexContext::GlimmerMustache,       // Inside {{...}}
```

### B. Lexer Re-lexing (JsReLexContext Enum)

The parser can request re-lexing of the current token in a different context:

```rust
pub enum JsReLexContext {
    Regex,                          // Re-interpret / as regex
    BinaryOperator,                 // Re-lex > > as >>
    TypeArgumentLessThan,           // Re-lex << as TypeArgument
    JsxIdentifier,                  // Re-lex identifier allowing `-`
    JsxChild,                       // Special JSX child handling
}
```

**Key method:** `parser.re_lex(context)` - re-lexes current token in new context

### C. State Management (JsParserState)

Parser maintains state across the parse:

```rust
pub(crate) struct JsParserState {
    parsing_context: ParsingContextFlags,  // Bitflags: IN_FUNCTION, IN_ASYNC, etc.
    label_set: IndexMap<String, LabelledItem>,
    strict: Option<StrictMode>,           // Track "use strict"
    default_item: Option<ExportDefaultItem>,
    duplicate_binding_parent: Option<&'static str>,
    name_map: IndexMap<String, TextRange>,
    speculative_parsing: bool,
    not_parenthesized_arrow: FxHashSet<TextSize>,
}
```

**Context flags (scoped state that can be saved/restored):**
- `IN_FUNCTION` - Inside function body
- `IN_ASYNC` - In async context
- `IN_GENERATOR` - In generator function
- `IN_CONSTRUCTOR` - In class constructor
- `TOP_LEVEL` - At top-level statements
- `BREAK_ALLOWED` / `CONTINUE_ALLOWED` - In loops/switches
- `AMBIENT_CONTEXT` - In TypeScript ambient context (d.ts files)

These enable context-sensitive parsing rules without passing state through every function.

## 4. JSX Parsing Implementation (Reference Model)

Since JSX is already implemented, it's the perfect reference for Glimmer integration.

### JSX Entry Point

**File:** `crates/biome_js_parser/src/syntax/jsx/mod.rs`

```rust
pub(crate) fn parse_jsx_tag_expression(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![<]) { return Absent; }
    // Check for JSX tag indicators
    if !p.nth_at(1, T![>])
        && !is_nth_at_identifier_or_keyword(p, 1)
        && !is_nth_at_metavariable(p, 1)
    { return Absent; }
    
    let m = p.start();
    parse_any_jsx_tag(p, true).unwrap();
    Present(m.complete(p, JSX_TAG_EXPRESSION))
}
```

### JSX Integration Point

**File:** `crates/biome_js_parser/src/syntax/expr.rs` (lines 31, 176+)

Inside expression parsing, it tries to parse JSX:
```rust
use crate::syntax::jsx::parse_jsx_tag_expression;

// In parse_primary_expression or similar...
if matches!(p.source_type().variant(), LanguageVariant::Jsx) {
    if let Present(jsx) = parse_jsx_tag_expression(p) {
        return Present(jsx);
    }
}
```

**Key feature:** Uses `p.source_type().variant()` to check if JSX is enabled, allowing JSX parsing only for:
- `.jsx` files
- `.tsx` files
- Files with `LanguageVariant::Jsx` or when explicitly enabled

### JSX Context Switching

When parsing JSX children:
1. Parser switches lexer context: `p.bump_with_context(T![>], JsLexContext::JsxChild)`
2. Lexer returns tokens appropriate for JSX child context
3. When encountering `{`, parser switches back to Regular context
4. After expression ends at `}`, switches back to JsxChild

## 5. Embedded Content Handler Pattern (File Handlers)

The service layer (`crates/biome_service/src/file_handlers/`) handles file-type specific extraction and reconstruction.

### Pattern: Vue File Handler (Reference)

**File:** `crates/biome_service/src/file_handlers/vue.rs`

```rust
pub struct VueFileHandler;

impl VueFileHandler {
    /// Extract JS code from <script> block
    pub fn input(text: &str) -> &str {
        match Self::matches_script(text) {
            Some(script) => &text[script.start()..script.end()],
            _ => "",
        }
    }

    /// Reconstruct Vue file with formatted JS
    pub fn output(input: &str, output: &str) -> String {
        if let Some(script) = Self::matches_script(input) {
            format!(
                "{}{}{}",
                &input[..script.start()],
                output,
                &input[script.end()..]
            )
        } else {
            input.to_string()
        }
    }

    /// Get JsFileSource with embedding_kind marked
    pub fn file_source(text: &str) -> JsFileSource {
        VUE_FENCE
            .captures(text)
            .and_then(|captures| {
                let (language, variant) = parse_lang_from_script_opening_tag(...);
                Some(
                    JsFileSource::from(language)
                        .with_variant(variant)
                        .with_embedding_kind(EmbeddingKind::Vue)  // ← Mark as embedded
                )
            })
            .map_or(JsFileSource::js_module(), |fs| fs)
    }
}
```

### Pattern: Glimmer File Handler (Already Exists!)

**File:** `crates/biome_service/src/file_handlers/glimmer.rs` (partially implemented)

Already has:
```rust
pub fn extract_js_content(text: &str) -> String {
    // Extract JS, replacing <template> with markers
}

pub fn output(input: &str, formatted_js: &str) -> String {
    // Reconstruct with original templates
}
```

## 6. File Type Detection

The system detects file types from extensions and passes appropriate `JsFileSource`:

**Routes:**
- `.js` → `JsFileSource::js_module()`
- `.jsx` → `JsFileSource::jsx()`
- `.ts` → `JsFileSource::ts()`
- `.tsx` → `JsFileSource::tsx()`
- `.gjs` → `JsFileSource::gjs()` (JS + Glimmer, embedding_kind = Glimmer)
- `.gts` → `JsFileSource::gts()` (TS + Glimmer, embedding_kind = Glimmer)

The embedding_kind field is used by:
- File handlers to determine extraction/reconstruction strategy
- Parser to enable/disable features (e.g., top-level return in Astro)

## 7. Extension Points for Glimmer Integration

### A. In the Parser (Recommended Approach)

**Similar to JSX, but simpler:**

1. **Add Glimmer support to expression parsing** (src/syntax/expr.rs)
   - When `embedding_kind == Glimmer`, allow `<template>` at top-level
   - Parse as template expression (different from JSX)

2. **Create Glimmer lexer context** (src/lexer/mod.rs)
   - Add `GlimmerTemplate` context
   - Add `GlimmerMustache` context
   - Handle tokenization of Glimmer syntax

3. **Add Glimmer statement parsing** (new src/syntax/glimmer.rs)
   - Parse template as expression or statement
   - Use biome_glimmer_parser for template content

### B. In the File Handler (Current Best Practice)

**Follows Vue/Astro/Svelte pattern:**

1. Extract JS code from `.gjs`/`.gts` (replace `<template>` with markers)
2. Parse JS with `embedding_kind = Glimmer` marker
3. Format JS independently
4. Reconstruct file with original templates

**Benefits:**
- Minimal changes to core JS parser
- Glimmer templates handled separately by biome_glimmer_parser
- Consistent with existing embedded language support
- Easy to maintain separation of concerns

## 8. Special Parsing Features Already Available

The parser already has several features useful for embedded content:

### A. Offset-Aware Parsing
- `parse_js_with_offset()` - Parse JS at specific position with offset ranges
- Perfect for parsing JS within larger files
- Returns `JsOffsetParse` with adjusted text ranges

### B. Parser Options
```rust
pub struct JsParserOptions {
    pub grit_metavariables: bool,
    pub parse_class_parameter_decorators: bool,
}
```

Can be extended to add Glimmer-specific options:
- `parse_glimmer_templates: bool`
- `glimmer_version: GlimmerVersion`

### C. Caching
- `NodeCache` can be reused across multiple parse calls
- Enables efficient re-parsing during edits

### D. State Management
- `ParserState` tracks parsing context
- Can be extended for Glimmer-specific state

## 9. Integration Architecture Recommendation

### Optimal Integration Path:

```
.gjs/.gts file
    ↓
biome_service::file_handlers::glimmer::GlimmerFileHandler
    ├─ Extract: Replace <template> with markers
    ├─ Parse JS with biome_js_parser
    │   └─ JsFileSource::gjs().with_embedding_kind(Glimmer)
    └─ If needed: Parse templates with biome_glimmer_parser
        (for separate template analysis, formatting, linting)

Formatting:
    ├─ Format JS AST with biome_js_formatter
    ├─ Format Glimmer templates with biome_glimmer_formatter
    └─ Reconstruct file with output()

Linting:
    ├─ Lint JS AST with biome_js_analyzer
    ├─ Lint Glimmer templates with biome_glimmer_analyzer
    └─ Combine diagnostics
```

### Files to Modify:

1. **biome_js_syntax/src/file_source.rs**
   - Already has `EmbeddingKind::Glimmer` ✓
   - Already has `JsFileSource::gjs()` and `::gts()` ✓

2. **biome_service/src/file_handlers/glimmer.rs**
   - Enhance extraction logic (handle semicolons, complex cases)
   - Add file_source() method
   - Add parse() implementation

3. **biome_service/src/file_handlers/mod.rs**
   - Register GlimmerFileHandler
   - Route `.gjs`/`.gts` to handler

4. **biome_js_parser/src/parse.rs** (Optional)
   - Could add `parse_gjs()` convenience function
   - Or keep generic, let file handler handle extraction

5. **CLI/Service** 
   - Enable Glimmer file processing in workflows

## 10. Key Takeaways

1. **The parser is highly modular** - JSX is parsed as an expression within the normal JS grammar, not as a special mode

2. **EmbeddingKind already exists** - Glimmer is already recognized as an embedding kind in JsFileSource

3. **File handlers are the bridge** - Service layer extracts language-specific content and reconstructs files

4. **Context switching is powerful** - Lexer contexts (`JsLexContext`) allow different tokenization rules

5. **Offset parsing is built-in** - Perfect for parsing embedded JS content with correct position tracking

6. **State is manageable** - Parser state can be saved/restored and scoped to blocks

7. **The path is clear** - Follow Vue/Svelte/Astro pattern for embedding + use biome_glimmer_parser for templates

