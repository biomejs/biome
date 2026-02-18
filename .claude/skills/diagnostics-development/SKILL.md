---
name: diagnostics-development
description: Guide for creating high-quality, user-friendly diagnostics in Biome. Use when implementing error messages, warnings, and code frame displays. Examples:<example>User needs to create a diagnostic for a lint rule</example><example>User wants to add helpful advice to error messages</example><example>User is improving diagnostic quality</example>
---

## Purpose

Use this skill when creating diagnostics - the error messages, warnings, and hints shown to users. Covers the `Diagnostic` trait, advice types, and best practices for clear, actionable messages.

## Prerequisites

1. Read `crates/biome_diagnostics/CONTRIBUTING.md` for concepts
2. Understand Biome's [Technical Principles](https://biomejs.dev/internals/philosophy/#technical)
3. Follow the "show don't tell" philosophy

## Diagnostic Principles

1. **Explain what** - State what the error is (diagnostic message)
2. **Explain why** - Explain why it's an error (advice notes)
3. **Tell how to fix** - Provide actionable fixes (code actions, diff advice, command advice)

**Follow Technical Principles:**
- Informative: Explain, don't just state
- Concise: Short messages, rich context via advices
- Actionable: Always suggest how to fix
- Show don't tell: Prefer code frames over textual explanations

## Common Workflows

### Create a Diagnostic Type

Use the `#[derive(Diagnostic)]` macro:

```rust
use biome_diagnostics::{Diagnostic, category};

#[derive(Debug, Diagnostic)]
#[diagnostic(
    severity = Error,
    category = "lint/correctness/noVar"
)]
struct NoVarDiagnostic {
    #[location(span)]
    span: TextRange,
    
    #[message]
    #[description]
    message: MessageAndDescription,
    
    #[advice]
    advice: NoVarAdvice,
}

#[derive(Debug)]
struct MessageAndDescription;

impl fmt::Display for MessageAndDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Use 'let' or 'const' instead of 'var'")
    }
}
```

### Implement Advices

Create advice types that implement `Advices` trait:

```rust
use biome_diagnostics::{Advices, Visit};
use biome_console::markup;

struct NoVarAdvice {
    is_const_candidate: bool,
}

impl Advices for NoVarAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        if self.is_const_candidate {
            visitor.record_log(
                LogCategory::Info,
                &markup! {
                    "This variable is never reassigned, use 'const' instead."
                }
            )?;
        } else {
            visitor.record_log(
                LogCategory::Info,
                &markup! {
                    "Variables declared with 'var' are function-scoped, use 'let' for block-scoping."
                }
            )?;
        }
        Ok(())
    }
}
```

### Use Built-in Advice Types

```rust
use biome_diagnostics::v2::{
    LogAdvice, CodeFrameAdvice, DiffAdvice, CommandAdvice
};

// Log advice - simple text
LogAdvice {
    category: LogCategory::Info,
    message: markup! { "Consider using arrow functions." }
}

// Code frame advice - highlight code location
CodeFrameAdvice {
    location: node.text_range(),
    source_code: ctx.source_code(),
    annotation: markup! { "This code is problematic" },
}

// Diff advice - show before/after
DiffAdvice {
    old: "var x = 1;",
    new: "const x = 1;",
}

// Command advice - suggest CLI command
CommandAdvice {
    command: "biome check --apply",
    message: markup! { "Run this command to fix automatically" },
}
```

### Add Diagnostic to Rule

```rust
use biome_analyze::{Rule, RuleDiagnostic};

impl Rule for NoVar {
    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use "<Emphasis>"let"</Emphasis>" or "<Emphasis>"const"</Emphasis>" instead of "<Emphasis>"var"</Emphasis>"."
                },
            )
            .note(markup! {
                "Variables declared with "<Emphasis>"var"</Emphasis>" are function-scoped, not block-scoped."
            })
            .note(markup! {
                "See the "<Hyperlink href="https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/var">"MDN documentation"</Hyperlink>" for more details."
            })
        )
    }
}
```

### Use Markup for Rich Text

Biome supports rich markup in diagnostic messages:

```rust
use biome_console::markup;

markup! {
    // Emphasis (bold/colored)
    "Use "<Emphasis>"const"</Emphasis>" instead."
    
    // Code/identifiers
    "The variable "<Emphasis>{variable_name}</Emphasis>" is never used."
    
    // Hyperlinks
    "See the "<Hyperlink href="https://example.com">"documentation"</Hyperlink>"."
    
    // Interpolation
    "Found "{count}" issues."
}
```

### Register Diagnostic Category

Add new categories to `crates/biome_diagnostics_categories/src/categories.rs`:

```rust
define_categories! {
    // Existing categories...
    
    "lint/correctness/noVar": "https://biomejs.dev/linter/rules/no-var",
    "lint/style/useConst": "https://biomejs.dev/linter/rules/use-const",
}
```

### Create Multi-Advice Diagnostics

```rust
#[derive(Debug, Diagnostic)]
#[diagnostic(severity = Warning)]
struct ComplexDiagnostic {
    #[location(span)]
    span: TextRange,
    
    #[message]
    message: &'static str,
    
    // Multiple advices
    #[advice]
    first_advice: LogAdvice,
    
    #[advice]
    code_frame: CodeFrameAdvice,
    
    #[verbose_advice]
    verbose_help: LogAdvice,
}
```

### Add Tags to Diagnostics

```rust
#[derive(Debug, Diagnostic)]
#[diagnostic(
    severity = Warning,
    tags(FIXABLE, DEPRECATED_CODE)  // Add diagnostic tags
)]
struct MyDiagnostic {
    // ...
}
```

Available tags:
- `FIXABLE` - Diagnostic has fix information
- `INTERNAL` - Internal error in Biome
- `UNNECESSARY_CODE` - Code is unused
- `DEPRECATED_CODE` - Code uses deprecated features

## Best Practices

### Message Guidelines

**Good messages:**
```rust
// ✅ Specific and actionable
"Use 'let' or 'const' instead of 'var'"

// ✅ Explains why
"This variable is never reassigned, consider using 'const'"

// ✅ Shows what to do
"Remove the unused import statement"
```

**Bad messages:**
```rust
// ❌ Too vague
"Invalid syntax"

// ❌ Just states the obvious
"Variable declared with 'var'"

// ❌ No guidance
"This code has a problem"
```

### Advice Guidelines

**Show, don't tell:**
```rust
// ✅ Good - shows code frame
CodeFrameAdvice {
    location: node.range(),
    source_code: source,
    annotation: markup! { "This expression is always truthy" }
}

// ❌ Less helpful - just text
LogAdvice {
    message: markup! { "The expression at line 5 is always truthy" }
}
```

**Provide actionable fixes:**
```rust
// ✅ Good - shows exact change
DiffAdvice {
    old: "var x = 1;",
    new: "const x = 1;",
}

// ❌ Less helpful - describes change
LogAdvice {
    message: markup! { "Change 'var' to 'const'" }
}
```

### Severity Levels

Choose appropriate severity:

```rust
// Fatal - Biome can't continue
severity = Fatal

// Error - Must be fixed (correctness, security, a11y)
severity = Error

// Warning - Should be fixed (suspicious code)
severity = Warning

// Information - Style suggestions
severity = Information

// Hint - Minor improvements
severity = Hint
```

## Common Patterns

```rust
// Pattern 1: Simple diagnostic with note
RuleDiagnostic::new(
    rule_category!(),
    node.range(),
    markup! { "Main message" },
)
.note(markup! { "Additional context" })

// Pattern 2: Diagnostic with code frame
RuleDiagnostic::new(
    rule_category!(),
    node.range(),
    markup! { "Main message" },
)
.detail(
    node.syntax().text_range(),
    markup! { "This part is problematic" }
)

// Pattern 3: Diagnostic with link
RuleDiagnostic::new(
    rule_category!(),
    node.range(),
    markup! { "Main message" },
)
.note(markup! {
    "See "<Hyperlink href="https://biomejs.dev/linter">"documentation"</Hyperlink>"."
})

// Pattern 4: Conditional advice
impl Advices for MyAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        if self.show_hint {
            visitor.record_log(
                LogCategory::Info,
                &markup! { "Hint: ..." }
            )?;
        }
        Ok(())
    }
}
```

## Tips

- **Category format**: Use `area/group/ruleName` format (e.g., `lint/correctness/noVar`)
- **Markup formatting**: Use `markup!` macro for all user-facing text
- **Hyperlinks**: Always link to documentation for more details
- **Code frames**: Include for spatial context when helpful
- **Multiple advices**: Chain multiple pieces of information
- **Verbose advices**: Use for extra details users can opt into
- **Description vs Message**: Description for plain text contexts (IDE popover), message for rich display
- **Register categories**: Don't forget to add to `categories.rs`

## References

- Full guide: `crates/biome_diagnostics/CONTRIBUTING.md`
- Technical principles: https://biomejs.dev/internals/philosophy/#technical
- Diagnostic trait: `crates/biome_diagnostics/src/diagnostic.rs`
- Advice types: `crates/biome_diagnostics/src/v2/`
- Examples: Search for `#[derive(Diagnostic)]` in codebase
