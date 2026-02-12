---
name: parser-development
description: Guide for implementing parsers with error recovery for new languages in Biome. Use when creating parsers for JavaScript, CSS, JSON, HTML, GraphQL, or adding new language support. Examples:<example>User needs to add parsing support for a new language</example><example>User wants to implement error recovery in parser</example><example>User is writing grammar definitions in .ungram format</example>
---

## Purpose

Use this skill when creating or modifying Biome's parsers. Covers grammar authoring with ungrammar, lexer implementation, error recovery strategies, and list parsing patterns.

## Prerequisites

1. Install required tools: `just install-tools`
2. Understand the language syntax you're implementing
3. Read `crates/biome_parser/CONTRIBUTING.md` for detailed concepts

## Common Workflows

### Create Grammar for New Language

Create a `.ungram` file in `xtask/codegen/` (e.g., `html.ungram`):

```
// html.ungram
// Legend:
//   Name =                -- non-terminal definition
//   'ident'               -- token (terminal)
//   A B                   -- sequence
//   A | B                 -- alternation
//   A*                    -- zero or more repetition
//   (A (',' A)* ','?)     -- repetition with separator and optional trailing comma
//   A?                    -- zero or one repetition
//   label:A               -- suggested name for field

HtmlRoot = element*

HtmlElement =
  '<'
  tag_name: HtmlName
  attributes: HtmlAttributeList
  '>'
  children: HtmlElementList
  '<' '/' close_tag_name: HtmlName '>'

HtmlAttributeList = HtmlAttribute*

HtmlAttribute =
  | HtmlSimpleAttribute
  | HtmlBogusAttribute

HtmlSimpleAttribute =
  name: HtmlName
  '='
  value: HtmlString

HtmlBogusAttribute = /* error recovery node */
```

**Naming conventions:**
- Prefix all nodes with language name: `HtmlElement`, `CssRule`
- Unions start with `Any`: `AnyHtmlAttribute`
- Error recovery nodes use `Bogus`: `HtmlBogusAttribute`
- Lists end with `List`: `HtmlAttributeList`
- Lists are mandatory (never optional), empty by default

### Generate Parser from Grammar

```shell
# Generate for specific language
just gen-grammar html

# Generate for multiple languages
just gen-grammar html css

# Generate all grammars
just gen-grammar
```

This creates:
- `biome_html_syntax/src/generated/` - Node definitions
- `biome_html_factory/src/generated/` - Node construction helpers
- Parser skeleton files (you'll implement the actual parsing logic)

### Implement a Lexer

Create `lexer/mod.rs` in your parser crate:

```rust
use biome_html_syntax::HtmlSyntaxKind;
use biome_parser::{lexer::Lexer, ParseDiagnostic};

pub(crate) struct HtmlLexer<'source> {
    source: &'source str,
    position: usize,
    current_kind: HtmlSyntaxKind,
    diagnostics: Vec<ParseDiagnostic>,
}

impl<'source> Lexer<'source> for HtmlLexer<'source> {
    const NEWLINE: Self::Kind = HtmlSyntaxKind::NEWLINE;
    const WHITESPACE: Self::Kind = HtmlSyntaxKind::WHITESPACE;
    
    type Kind = HtmlSyntaxKind;
    type LexContext = ();
    type ReLexContext = ();

    fn source(&self) -> &'source str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn position(&self) -> usize {
        self.position
    }

    fn advance(&mut self, context: Self::LexContext) -> Self::Kind {
        // Implement token scanning logic
        let start = self.position;
        let kind = self.read_next_token();
        self.current_kind = kind;
        kind
    }
    
    // Implement other required methods...
}
```

### Implement Token Source

```rust
use biome_parser::lexer::BufferedLexer;
use biome_html_syntax::HtmlSyntaxKind;
use crate::lexer::HtmlLexer;

pub(crate) struct HtmlTokenSource<'src> {
    lexer: BufferedLexer<HtmlSyntaxKind, HtmlLexer<'src>>,
}

impl<'source> TokenSourceWithBufferedLexer<HtmlLexer<'source>> for HtmlTokenSource<'source> {
    fn lexer(&mut self) -> &mut BufferedLexer<HtmlSyntaxKind, HtmlLexer<'source>> {
        &mut self.lexer
    }
}
```

### Write Parse Rules

Example: Parsing an if statement:

```rust
use biome_parser::prelude::*;
use biome_js_syntax::JsSyntaxKind::*;

fn parse_if_statement(p: &mut JsParser) -> ParsedSyntax {
    // Presence test - return Absent if not at 'if'
    if !p.at(T![if]) {
        return Absent;
    }

    let m = p.start();

    // Parse required tokens
    p.expect(T![if]);
    p.expect(T!['(']);
    
    // Parse required nodes with error recovery
    parse_any_expression(p).or_add_diagnostic(p, expected_expression);
    
    p.expect(T![')']);
    parse_block_statement(p).or_add_diagnostic(p, expected_block);
    
    // Parse optional else clause
    if p.at(T![else]) {
        parse_else_clause(p).ok();
    }

    Present(m.complete(p, JS_IF_STATEMENT))
}
```

### Parse Lists with Error Recovery

Use `ParseSeparatedList` for comma-separated lists:

```rust
struct ArrayElementsList;

impl ParseSeparatedList for ArrayElementsList {
    type ParsedElement = CompletedMarker;

    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<Self::ParsedElement> {
        parse_array_element(p)
    }

    fn is_at_list_end(&self, p: &mut Parser) -> bool {
        // Stop at array closing bracket or file end
        p.at(T![']']) || p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Parser,
        parsed_element: ParsedSyntax<Self::ParsedElement>,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecoveryTokenSet::new(
                JS_BOGUS_EXPRESSION,
                token_set![T![']'], T![,]]
            ),
            expected_array_element,
        )
    }
    
    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }
}

// Use the list parser
fn parse_array_elements(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    ArrayElementsList.parse_list(p);
    m.complete(p, JS_ARRAY_ELEMENT_LIST)
}
```

### Implement Error Recovery

Error recovery wraps invalid tokens in `BOGUS` nodes:

```rust
// Recovery set includes:
// - List terminator tokens (e.g., ']', '}')
// - Statement terminators (e.g., ';')
// - List separators (e.g., ',')
let recovery_set = token_set![T![']'], T![,], T![;]];

parsed_element.or_recover(
    p,
    &ParseRecoveryTokenSet::new(JS_BOGUS_EXPRESSION, recovery_set),
    expected_expression_error,
)
```

### Handle Conditional Syntax

For syntax only valid in certain contexts (e.g., strict mode):

```rust
fn parse_with_statement(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![with]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![with]);
    parenthesized_expression(p).or_add_diagnostic(p, expected_expression);
    parse_statement(p).or_add_diagnostic(p, expected_statement);
    
    let with_stmt = m.complete(p, JS_WITH_STATEMENT);

    // Mark as invalid in strict mode
    let conditional = StrictMode.excluding_syntax(p, with_stmt, |p, marker| {
        p.err_builder(
            "`with` statements are not allowed in strict mode",
            marker.range(p)
        )
    });

    Present(conditional.or_invalid_to_bogus(p))
}
```

### Test Parser

Create test files in `tests/`:

```
crates/biome_html_parser/tests/
├── html_specs/
│   ├── ok/
│   │   ├── simple_element.html
│   │   └── nested_elements.html
│   └── error/
│       ├── unclosed_tag.html
│       └── invalid_syntax.html
└── html_test.rs
```

Run tests:
```shell
cd crates/biome_html_parser
cargo test
```

## Tips

- **Presence test**: Always return `Absent` if the first token doesn't match - never progress parsing before returning `Absent`
- **Required vs optional**: Use `p.expect()` for required tokens, `p.eat()` for optional ones
- **Missing markers**: Use `.or_add_diagnostic()` for required nodes to add missing markers and errors
- **Error recovery**: Include list terminators, separators, and statement boundaries in recovery sets
- **Bogus nodes**: Check grammar for which `BOGUS_*` node types are valid in your context
- **Checkpoints**: Use `p.checkpoint()` to save state and `p.rewind()` if parsing fails
- **Lookahead**: Use `p.at()` to check tokens, `p.nth_at()` for lookahead beyond current token
- **Lists are mandatory**: Always create list nodes even if empty - use `parse_list()` not `parse_list().ok()`

## Common Patterns

```rust
// Optional token
if p.eat(T![async]) {
    // handle async
}

// Required token with error
p.expect(T!['{']);

// Optional node
parse_type_annotation(p).ok();

// Required node with error
parse_expression(p).or_add_diagnostic(p, expected_expression);

// Lookahead
if p.at(T![if]) || p.at(T![for]) {
    // handle control flow
}

// Checkpoint for backtracking
let checkpoint = p.checkpoint();
if parse_something(p).is_absent() {
    p.rewind(checkpoint);
    parse_something_else(p);
}
```

## References

- Full guide: `crates/biome_parser/CONTRIBUTING.md`
- Grammar examples: `xtask/codegen/*.ungram`
- Parser examples: `crates/biome_js_parser/src/syntax/`
- Error recovery: Search for `ParseRecoveryTokenSet` in existing parsers
