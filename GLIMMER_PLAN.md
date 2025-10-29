# Glimmer.js (GJS/GTS) Support Implementation Plan for Biome

## Overview

This plan outlines the complete implementation for adding Glimmer.js support to Biome, including full support for the Glimmer VM AST (https://github.com/glimmerjs/glimmer-vm). This will enable parsing, formatting, and linting of `.gjs` (JavaScript + Glimmer templates) and `.gts` (TypeScript + Glimmer templates) files.

## Architecture Overview

Glimmer.js uses a unique syntax where templates are embedded in JavaScript/TypeScript as tagged template literals:

```gjs
// Example GJS file
import Component from '@glimmer/component';

export default class Counter extends Component {
  <template>
    <button {{on "click" this.increment}}>
      {{this.count}}
    </button>
  </template>
  
  count = 0;
  
  increment = () => {
    this.count++;
  }
}
```

The key insight is that `<template>` is a **tagged template literal** in the JavaScript AST, not an HTML element. The template content contains Glimmer syntax (Handlebars-like).

---

## Phase 1: Foundation & File Recognition

### 1.1 Add Glimmer EmbeddingKind

**File:** `crates/biome_js_syntax/src/file_source.rs`

**Changes:**
- Add `Glimmer` variant to `EmbeddingKind` enum
- Add helper method `is_glimmer()`
- Add `gjs()` and `gts()` constructor methods to `JsFileSource`
- Update `try_from_extension()` to handle `.gjs` and `.gts`
- Update `try_from_language_id()` to handle LSP language IDs

```rust
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub enum EmbeddingKind {
    Astro,
    Vue,
    Svelte,
    Glimmer, // NEW
    #[default]
    None,
}

impl JsFileSource {
    /// GJS file definition (JavaScript + Glimmer templates)
    pub fn gjs() -> Self {
        Self::jsx().with_embedding_kind(EmbeddingKind::Glimmer)
    }

    /// GTS file definition (TypeScript + Glimmer templates)
    pub fn gts() -> Self {
        Self::tsx().with_embedding_kind(EmbeddingKind::Glimmer)
    }
}
```

### 1.2 Update Configuration

**Files:**
- `crates/biome_configuration/src/lib.rs` (or appropriate config file)
- `biome.json` schema

Add GJS/GTS to JavaScript language settings so they're recognized as JavaScript files.

---

## Phase 2: Create Glimmer Syntax Crates

We need to create a full Glimmer language implementation following Biome's architecture.

### 2.1 Create `biome_glimmer_syntax` Crate

**Purpose:** Define the AST nodes for Glimmer templates based on Glimmer VM AST.

**Structure:**
```
crates/biome_glimmer_syntax/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── kind.rs           // Token kinds (enum)
│   ├── syntax_node.rs    // Syntax node wrappers
│   ├── generated/        // Auto-generated from grammar
│   │   ├── nodes.rs
│   │   └── kind.rs
│   └── file_source.rs
```

**Key Types to Define (mirroring Glimmer VM):**
- `GlimmerRoot` - Root template node
- `GlimmerElementNode` - HTML elements (including components)
- `GlimmerMustacheStatement` - `{{expression}}`
- `GlimmerBlockStatement` - `{{#if}}...{{/if}}`
- `GlimmerTextNode` - Plain text
- `GlimmerCommentStatement` - `{{! comment }}`
- `GlimmerPathExpression` - Variable paths like `this.count` or `@arg`
- `GlimmerSubExpression` - `(helper arg)`
- `GlimmerAttrNode` - HTML attributes
- `GlimmerElementModifierStatement` - `{{on "click" this.handler}}`
- Literal nodes: String, Number, Boolean, Null, Undefined
- `GlimmerHash` and `GlimmerHashPair` - Named arguments

**Grammar File:**
Create `crates/biome_glimmer_syntax/glimmer.ungram` following Glimmer VM's AST structure.

Example:
```ungram
// Root
GlimmerRoot = Statement*

Statement =
    GlimmerElementNode
  | GlimmerTextNode
  | GlimmerMustacheStatement
  | GlimmerBlockStatement
  | GlimmerCommentStatement

GlimmerElementNode =
  'element_start' 'path:' PathExpression
  'attributes:' AttrNode*
  'modifiers:' ElementModifierStatement*
  'children:' Statement*
  'element_end'

GlimmerMustacheStatement =
  'mustache_open' 
  'path:' Expression
  'params:' Expression*
  'hash:' Hash
  'mustache_close'

// ... more nodes
```

### 2.2 Create `biome_glimmer_parser` Crate

**Purpose:** Parse Glimmer template syntax into the AST.

**Structure:**
```
crates/biome_glimmer_parser/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── lexer/
│   │   ├── mod.rs        // Tokenizer
│   │   └── tests.rs
│   ├── parser/
│   │   ├── mod.rs        // Main parser logic
│   │   ├── template.rs   // Template parsing
│   │   ├── expression.rs // Expression parsing
│   │   ├── statement.rs  // Statement parsing
│   │   └── attribute.rs  // Attribute parsing
│   ├── token_source.rs
│   └── prelude.rs
└── tests/
    └── spec_tests.rs
```

**Key Parsing Functions:**
```rust
/// Parse a Glimmer template string
pub fn parse_glimmer_template(
    source: &str,
    options: GlimmerParserOptions,
) -> GlimmerParse {
    // Entry point
}

/// Parse with offset for embedded templates
pub fn parse_glimmer_template_with_offset(
    source: &str,
    offset: TextSize,
    options: GlimmerParserOptions,
    cache: &mut NodeCache,
) -> GlimmerParse {
    // For use in embedded context
}
```

**Lexer Tokens:**
- `MUSTACHE_OPEN` - `{{`
- `MUSTACHE_CLOSE` - `}}`
- `BLOCK_OPEN` - `{{#`
- `BLOCK_CLOSE` - `{{/`
- `ELEMENT_START` - `<`
- `ELEMENT_END` - `>`
- `SELF_CLOSING` - `/>`
- `IDENT` - Identifiers
- `STRING_LITERAL`
- `NUMBER_LITERAL`
- `DOT` - `.` for paths
- `AT` - `@` for arguments
- `THIS` - `this` keyword
- etc.

**Parser Strategy:**
1. Use a handwritten recursive descent parser (like `biome_js_parser`)
2. Handle Handlebars/Glimmer syntax: `{{`, `{{#if}}`, `{{#each}}`, etc.
3. Parse HTML-like elements with component syntax: `<MyComponent @arg={{value}} />`
4. Support element modifiers: `{{on "click" handler}}`
5. Proper error recovery for incomplete templates

### 2.3 Create `biome_glimmer_factory` Crate

**Purpose:** Builders for constructing Glimmer AST nodes programmatically.

**Structure:**
```
crates/biome_glimmer_factory/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── generated/
│   │   └── node_factory.rs  // Auto-generated builders
│   └── syntax_factory.rs
```

Generated builders for creating AST nodes in formatters and transformations.

### 2.4 Create `biome_glimmer_formatter` Crate

**Purpose:** Format Glimmer template code.

**Structure:**
```
crates/biome_glimmer_formatter/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── context.rs         // Formatting context/options
│   ├── glimmer/
│   │   ├── mod.rs
│   │   ├── template.rs    // Format template root
│   │   ├── element.rs     // Format elements
│   │   ├── mustache.rs    // Format {{expressions}}
│   │   ├── block.rs       // Format block statements
│   │   └── auxiliary.rs   // Format helpers, modifiers
│   └── utils.rs
└── tests/
    └── spec_tests.rs
```

**Formatting Rules:**
- Indent nested elements
- Format attributes (one per line if many, inline if few)
- Format Handlebars expressions
- Respect whitespace sensitivity around text nodes
- Handle component arguments formatting

### 2.5 Create `biome_glimmer_analyze` Crate

**Purpose:** Lint rules for Glimmer templates.

**Structure:**
```
crates/biome_glimmer_analyze/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── lint/
│   │   ├── mod.rs
│   │   ├── a11y/          // Accessibility rules
│   │   ├── correctness/   // Correctness rules
│   │   ├── security/      // Security rules
│   │   └── style/         // Style rules
│   ├── options.rs
│   ├── registry.rs
│   └── suppression_action.rs
└── tests/
    └── spec_tests.rs
```

**Example Lint Rules:**
1. **Template-only rules:**
   - `noUndefinedComponents` - Component must be imported
   - `noUnusedArguments` - `@arg` declared but not used
   - `requireAltText` - Images must have alt text
   - `noInvalidModifiers` - Modifier must be valid
   
2. **Hybrid rules (require JS context):**
   - `noUnboundThis` - `this.property` must exist in class
   - `requireValidHelpers` - Helper must be imported
   - `noMissingArguments` - `@arg` used but not passed

---

## Phase 3: Glimmer File Handler

### 3.1 Create `glimmer.rs` File Handler

**File:** `crates/biome_service/src/file_handlers/glimmer.rs`

This is the key integration point that ties everything together.

```rust
use crate::WorkspaceError;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, CodeActionsParams, DebugCapabilities,
    EnabledForPath, ExtensionHandler, FixAllParams, FormatterCapabilities,
    LintParams, LintResults, ParseResult, ParserCapabilities, javascript,
};
use crate::settings::Settings;
use crate::workspace::{DocumentFileSource, FixFileResult, PullActionsResult, EmbeddedSnippet};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_js_parser::{JsParserOptions, parse_js_with_cache};
use biome_js_syntax::{JsFileSource, JsLanguage, TextRange, TextSize};
use biome_glimmer_parser::parse_glimmer_template_with_offset;
use biome_glimmer_syntax::GlimmerLanguage;
use biome_parser::AnyParse;
use biome_rowan::NodeCache;

use super::SearchCapabilities;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct GlimmerFileHandler;

impl GlimmerFileHandler {
    /// Extract template literals from JS/TS
    pub fn extract_templates(js_tree: &JsSyntaxNode) -> Vec<TemplateInfo> {
        // Walk the JS AST looking for:
        // 1. <template> tagged template literals
        // 2. class members with <template> syntax
        // Return info about each template's location and content
    }
}

impl ExtensionHandler for GlimmerFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            enabled_for_path: EnabledForPath {
                formatter: Some(formatter_enabled),
                search: Some(javascript::search_enabled),
                assist: Some(javascript::assist_enabled),
                linter: Some(linter_enabled),
            },
            parser: ParserCapabilities {
                parse: Some(parse),
                parse_embedded_nodes: Some(parse_embedded_nodes), // KEY!
            },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: None,
                debug_formatter_ir: None,
                debug_type_info: None,
                debug_registered_types: None,
                debug_semantic_model: None,
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: Some(code_actions),
                rename: None,
                fix_all: Some(fix_all),
                update_snippets: None,
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
                format_embedded: Some(format_embedded),
            },
            search: SearchCapabilities {
                search: Some(search),
            },
        }
    }
}

/// Parse the GJS/GTS file as JavaScript/TypeScript
fn parse(
    biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    _settings: &Settings,
    cache: &mut NodeCache,
) -> ParseResult {
    // Parse as JS/TS with JSX enabled
    let js_file_source = if biome_path.extension() == Some("gts") {
        JsFileSource::gts()
    } else {
        JsFileSource::gjs()
    };
    
    let parse = parse_js_with_cache(
        text,
        js_file_source,
        JsParserOptions::default(),
        cache,
    );
    
    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source),
    }
}

/// Extract and parse embedded <template> blocks
fn parse_embedded_nodes(
    root: &AnyParse,
    biome_path: &BiomePath,
    file_source: &DocumentFileSource,
    settings: &Settings,
    cache: &mut NodeCache,
) -> ParseEmbedResult {
    let mut nodes = Vec::new();
    let js_root = root.tree();
    
    // Walk the JavaScript AST to find template literals
    for node in js_root.syntax().descendants() {
        // Look for patterns like:
        // 1. <template>...</template> (tagged template literal)
        // 2. Static class field: <template>
        
        if let Some(template_info) = find_glimmer_template(node) {
            // Parse the template content as Glimmer
            let glimmer_parse = parse_glimmer_template_with_offset(
                template_info.content,
                template_info.content_offset,
                GlimmerParserOptions::default(),
                cache,
            );
            
            let snippet = EmbeddedSnippet::new(
                glimmer_parse.into(),
                template_info.element_range,
                template_info.content_range,
                template_info.content_offset,
            );
            
            nodes.push((
                snippet.into(),
                DocumentFileSource::Glimmer(GlimmerFileSource::default()),
            ));
        }
    }
    
    ParseEmbedResult { nodes }
}

// Implement other functions: lint, format, code_actions, etc.
```

### 3.2 Template Extraction Logic

The tricky part is identifying `<template>` in the JavaScript AST. In GJS/GTS:

```javascript
// Pattern 1: Tagged template literal
<template>Hello {{@name}}</template>

// Pattern 2: Class field with <template>
class MyComponent {
  <template>Hello</template>
}
```

These appear in the JavaScript AST as:
- **Pattern 1**: `JsxTaggedTemplateExpression` or similar
- **Pattern 2**: `JsClassMemberName` followed by template content

**Implementation:**
```rust
fn find_glimmer_template(node: &JsSyntaxNode) -> Option<TemplateInfo> {
    // Check if this is a <template> tagged template
    if node.kind() == JsSyntaxKind::JSX_ELEMENT {
        let element = JsxElement::cast_ref(node)?;
        let opening = element.opening_element()?;
        let name = opening.name()?;
        
        if name.as_jsx_name()?.value_token()?.text() == "template" {
            // Extract the content between tags
            let content = extract_template_content(&element);
            return Some(TemplateInfo {
                content,
                element_range: element.range(),
                content_range: content.text_range(),
                content_offset: content.text_range().start(),
            });
        }
    }
    
    None
}
```

### 3.3 Register Handler

**File:** `crates/biome_service/src/file_handlers/mod.rs`

```rust
mod glimmer;
pub use glimmer::GlimmerFileHandler;

// In the handler registration function:
pub fn extension_handler(path: &BiomePath) -> Box<dyn ExtensionHandler> {
    match path.extension().and_then(|s| s.to_str()) {
        Some("gjs") | Some("gts") => Box::new(GlimmerFileHandler),
        // ... other handlers
        _ => Box::new(UnknownFileHandler),
    }
}
```

---

## Phase 4: Enhanced Linting with Hybrid Analysis

### 4.1 Cross-Language Context

**Create:** `crates/biome_glimmer_analyze/src/context.rs`

```rust
/// Context for Glimmer analysis that includes JS scope information
pub struct GlimmerAnalysisContext {
    /// Parsed JavaScript/TypeScript for the file
    pub js_parse: AnyParse,
    
    /// Imported components and helpers
    pub imports: HashMap<String, ImportInfo>,
    
    /// Class properties (for this.xxx validation)
    pub class_properties: HashSet<String>,
    
    /// Available arguments (for @arg validation)
    pub available_args: HashSet<String>,
}

impl GlimmerAnalysisContext {
    pub fn from_js_parse(parse: &AnyParse) -> Self {
        // Walk the JS AST to extract:
        // - Import statements
        // - Class property declarations
        // - Method definitions
        Self {
            js_parse: parse.clone(),
            imports: extract_imports(parse),
            class_properties: extract_class_properties(parse),
            available_args: HashSet::new(),
        }
    }
}
```

### 4.2 Hybrid Lint Rules

**Example Rule:** `noUndefinedComponents`

**File:** `crates/biome_glimmer_analyze/src/lint/correctness/no_undefined_components.rs`

```rust
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_glimmer_syntax::{GlimmerElementNode};
use crate::context::GlimmerAnalysisContext;

declare_lint_rule! {
    /// Disallow using components that are not imported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```gjs
    /// // No import for MyComponent
    /// <template>
    ///   <MyComponent />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```gjs
    /// import MyComponent from './my-component';
    /// 
    /// <template>
    ///   <MyComponent />
    /// </template>
    /// ```
    pub NoUndefinedComponents {
        version: "next",
        name: "noUndefinedComponents",
        recommended: true,
    }
}

impl Rule for NoUndefinedComponents {
    type Query = GlimmerElementNode;
    type State = String; // Component name
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let component_name = node.tag()?;
        
        // Skip HTML elements (lowercase)
        if component_name.chars().next()?.is_lowercase() {
            return None;
        }
        
        // Get the JS context
        let js_context = ctx.metadata::<GlimmerAnalysisContext>()?;
        
        // Check if component is imported
        if !js_context.imports.contains_key(&component_name) {
            return Some(component_name);
        }
        
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, component_name: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            format!("Component '{}' is used but not imported.", component_name),
        ))
    }
}
```

**Example Rule:** `noUnboundThis`

**File:** `crates/biome_glimmer_analyze/src/lint/correctness/no_unbound_this.rs`

```rust
declare_lint_rule! {
    /// Disallow using `this.property` when property doesn't exist in the component class.
    pub NoUnboundThis {
        version: "next",
        name: "noUnboundThis",
        recommended: true,
    }
}

impl Rule for NoUnboundThis {
    type Query = GlimmerPathExpression;
    type State = (String, String); // (path, property)
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let path = ctx.query();
        
        // Check if path starts with "this."
        if !path.is_this_path() {
            return None;
        }
        
        let property = path.tail().first()?;
        let js_context = ctx.metadata::<GlimmerAnalysisContext>()?;
        
        // Check if property exists in class
        if !js_context.class_properties.contains(property) {
            return Some((path.to_string(), property.to_string()));
        }
        
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            format!(
                "Property '{}' is referenced in template but not defined in component class.",
                state.1
            ),
        ))
    }
}
```

---

## Phase 5: Formatting Integration

### 5.1 Format Entire File

When formatting a GJS/GTS file:

1. **Format the JavaScript/TypeScript** (excluding template content)
2. **Format each template** separately
3. **Reconstruct the file** with formatted templates inserted back

**File:** `crates/biome_service/src/file_handlers/glimmer.rs`

```rust
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
) -> Result<Printed, WorkspaceError> {
    // Format the JavaScript portion
    let js_formatted = javascript::format(biome_path, document_file_source, parse, settings)?;
    
    // Get embedded templates
    let templates = extract_templates(parse.tree());
    
    // Format each template
    let formatted_templates: Vec<_> = templates
        .iter()
        .map(|template| format_template(template, settings))
        .collect();
    
    // Reconstruct: replace template regions with formatted versions
    let final_output = reconstruct_with_templates(
        js_formatted.as_code(),
        &templates,
        &formatted_templates,
    );
    
    Ok(Printed::new(final_output))
}

fn format_template(
    template: &TemplateInfo,
    settings: &Settings,
) -> Result<String, FormatError> {
    let options = settings.format_options::<GlimmerLanguage>(...);
    let formatted = biome_glimmer_formatter::format_node(options, &template.ast)?;
    Ok(formatted.print()?.as_code().to_string())
}
```

---

## Phase 6: LSP Integration & Testing

### 6.1 LSP Support

Ensure the LSP recognizes GJS/GTS files:

**File:** `crates/biome_lsp/src/...` (wherever language IDs are registered)

```rust
// Register language IDs for LSP clients
match language_id {
    "glimmer-js" | "gjs" => JsFileSource::gjs(),
    "glimmer-ts" | "gts" => JsFileSource::gts(),
    // ... existing languages
}
```

### 6.2 Testing Strategy

**Unit Tests:**
1. **Parser tests:** `crates/biome_glimmer_parser/tests/`
   - Test all Glimmer syntax forms
   - Error recovery tests
   - Offset tracking tests

2. **Formatter tests:** `crates/biome_glimmer_formatter/tests/`
   - Snapshot tests for formatting
   - Idempotency tests

3. **Analyzer tests:** `crates/biome_glimmer_analyze/tests/`
   - Each lint rule with valid/invalid cases
   - Hybrid rule tests with JS context

**Integration Tests:**
4. **End-to-end tests:** `crates/biome_service/tests/`
   - Parse → Lint → Format full GJS/GTS files
   - Test embedded template extraction
   - Test multi-template files

**Test Files:**
```
// test-cases/glimmer/simple.gjs
import Component from '@glimmer/component';

export default class Counter extends Component {
  <template>
    <button {{on "click" this.increment}}>
      Count: {{this.count}}
    </button>
  </template>
  
  count = 0;
  increment = () => this.count++;
}
```

### 6.3 Documentation

**Update:**
1. **Website:** Add Glimmer.js to supported languages
2. **Guides:** Create GJS/GTS configuration guide
3. **Rules:** Document all Glimmer-specific lint rules
4. **Migration:** Guide for Prettier/ESLint users

---

## Phase 7: Advanced Features

### 7.1 Semantic Analysis

**Create:** `crates/biome_glimmer_semantic/`

Build a semantic model for templates:
- Track variable bindings from `{{#each}}`, `{{#let}}`
- Resolve component argument flow
- Build scope chains for block helpers

### 7.2 Code Actions

**Quick fixes:**
- Import missing component
- Add missing argument to component invocation
- Convert `{{this.property}}` to use getter
- Simplify boolean attributes

### 7.3 Rename Support

Enable renaming that works across template and script:
- Rename class property → updates all `{{this.property}}` references
- Rename argument → updates all `@arg` references
- Rename component → updates import and usages

### 7.4 Module Resolution

Integrate with Biome's module graph:
- Resolve component imports
- Track helper imports
- Validate modifier imports

---

## Implementation Timeline

### Milestone 1: Basic Support (4-6 weeks)
- ✅ File recognition (Phase 1)
- ✅ Glimmer syntax crate (Phase 2.1)
- ✅ Glimmer parser crate (Phase 2.2)
- ✅ Basic file handler (Phase 3.1-3.2)
- ✅ Template extraction
- ⚠️ Basic parsing only (no formatting/linting yet)

**Deliverable:** Biome can parse GJS/GTS files without errors

### Milestone 2: Formatting (3-4 weeks)
- ✅ Glimmer formatter crate (Phase 2.4)
- ✅ Format embedded templates (Phase 5)
- ✅ Formatter tests

**Deliverable:** `biome format` works on GJS/GTS files

### Milestone 3: Basic Linting (4-6 weeks)
- ✅ Glimmer analyze crate (Phase 2.5)
- ✅ Template-only lint rules
- ✅ Basic analyzer tests

**Deliverable:** Template-specific lint rules work

### Milestone 4: Hybrid Analysis (6-8 weeks)
- ✅ Cross-language context (Phase 4.1)
- ✅ Hybrid lint rules (Phase 4.2)
- ✅ Full integration tests (Phase 6.2)

**Deliverable:** Lint rules that validate templates against JS/TS context

### Milestone 5: Polish & Advanced (4-6 weeks)
- ✅ LSP integration (Phase 6.1)
- ✅ Documentation (Phase 6.3)
- ✅ Code actions (Phase 7.2)
- ✅ Performance optimization

**Deliverable:** Production-ready Glimmer.js support

---

## Technical Challenges & Solutions

### Challenge 1: Template Extraction from JS AST

**Problem:** `<template>` looks like JSX but isn't standard JSX.

**Solution:** 
- Treat as a special JSX-like tagged template
- Parse the JS with JSX enabled but recognize `<template>` specifically
- Extract content and parse separately with Glimmer parser

### Challenge 2: Offset Mapping

**Problem:** Diagnostics in templates need correct source positions in the original file.

**Solution:**
- Use `TextSize` offset tracking throughout
- `parse_glimmer_template_with_offset()` adjusts all positions
- `EmbeddedSnippet` maintains all ranges

### Challenge 3: Formatting Templates in JS Context

**Problem:** Template indentation depends on JS context.

**Solution:**
- Calculate base indentation from JS context
- Pass indent level to Glimmer formatter
- Use `print_with_indent()` like Vue handler does

### Challenge 4: Glimmer Syntax Complexity

**Problem:** Glimmer has rich syntax: modifiers, helpers, named blocks, etc.

**Solution:**
- Study Glimmer VM parser thoroughly
- Implement incrementally: elements → mustaches → blocks → modifiers
- Extensive test suite

### Challenge 5: Hybrid Rule Performance

**Problem:** Analyzing both JS and templates is expensive.

**Solution:**
- Cache JS analysis results
- Only extract needed information (imports, properties)
- Lazy evaluation where possible

---

## Dependencies

### New Crate Dependencies
- `biome_glimmer_syntax` → `biome_rowan`, `biome_string_case`
- `biome_glimmer_parser` → `biome_glimmer_syntax`, `biome_parser`, `biome_unicode_table`
- `biome_glimmer_formatter` → `biome_glimmer_syntax`, `biome_formatter`
- `biome_glimmer_analyze` → `biome_glimmer_syntax`, `biome_analyze`, `biome_js_syntax`
- `biome_service` → Add `biome_glimmer_*` as dependencies

### External Resources
- Reference: https://github.com/glimmerjs/glimmer-vm
- Glimmer syntax spec: https://github.com/glimmerjs/glimmer-vm/tree/main/packages/@glimmer/syntax
- Content-tag spec: https://github.com/embroider-build/content-tag

---

## Success Criteria

✅ **Parsing:**
- Can parse all valid GJS/GTS syntax
- Proper error recovery for invalid syntax
- No false positives on valid code

✅ **Formatting:**
- Formats both JS and template portions
- Respects Glimmer conventions
- Idempotent (formatting twice = same result)

✅ **Linting:**
- Template-only rules work
- Hybrid rules validate template against JS context
- Clear, actionable error messages

✅ **Performance:**
- Parse + Lint + Format < 100ms for typical file
- Scales to files with multiple templates

✅ **DX:**
- LSP integration works
- Good error messages
- Documentation is clear

---

## Notes

- This implementation follows Biome's existing patterns (Vue, Astro, Svelte)
- The embedded language approach is preferred over extending the JS parser
- Full Glimmer VM AST compatibility ensures future-proofing
- Incremental implementation allows for early feedback

---

## References

1. Glimmer VM: https://github.com/glimmerjs/glimmer-vm
2. Content Tag: https://github.com/embroider-build/content-tag
3. Biome Contributing Guide: https://github.com/biomejs/biome/blob/main/CONTRIBUTING.md
4. Glimmer Component Spec: https://guides.emberjs.com/release/components/
