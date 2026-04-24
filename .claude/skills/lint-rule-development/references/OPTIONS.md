# Rule Options — Detailed Reference

This file contains detailed patterns, design guidelines, and examples for implementing
configurable options on lint rules. For the quick-start workflow, see the main
[SKILL.md](../SKILL.md) "Adding Configurable Options" section.

## Define Rule Options Type

Options live in `biome_rule_options` crate. After running `just gen-rules`, a file is
created for your rule.

Example for `useThisConvention` rule in `biome_rule_options/src/use_this_convention.rs`:

```rust
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Deserializable)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseThisConventionOptions {
    /// What behavior to enforce
    #[serde(skip_serializing_if = "Option::is_none")]
    behavior: Option<Behavior>,

    /// Threshold value between 0-255
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<u8>,

    /// Exceptions to the behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    behavior_exceptions: Option<Box<[Box<str>]>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Deserializable, Merge)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum Behavior {
    #[default]
    A,
    B,
    C,
}
```

**Key points:**
- All fields wrapped in `Option<_>` for proper merging
- Use `Box<[Box<str>]>` instead of `Vec<String>` (saves memory)
- `#[serde(rename_all = "camelCase")]` for JavaScript naming
- `#[serde(deny_unknown_fields)]` to catch typos
- `#[serde(default)]` makes all fields optional

## Implement Merge Trait

Options from shared config + user config need merging:

```rust
impl biome_deserialize::Merge for UseThisConventionOptions {
    fn merge_with(&mut self, other: Self) {
        // `self` = shared config
        // `other` = user config

        // For simple values, use helper
        self.behavior.merge_with(other.behavior);
        self.threshold.merge_with(other.threshold);

        // For collections, typically reset instead of combine
        if let Some(exceptions) = other.behavior_exceptions {
            self.behavior_exceptions = Some(exceptions);
        }
    }
}
```

**Merge strategies:**
- **Simple values** (enums, numbers): Use `merge_with()` (takes user value if present)
- **Collections**: Usually reset to user value, not combine
- **Derive macro**: Can use `#[derive(Merge)]` for simple cases

## Option Design Guidelines

### When to Add Options

**Good reasons:**
- Conflicting style preferences in community
- Rule has multiple valid interpretations
- Different behavior needed for different environments

**Bad reasons:**
- Making rule "more flexible" without clear use case
- Avoiding making opinionated decision
- Working around incomplete implementation

### Option Naming

```rust
// Good - clear, semantic names
allow_single_line: bool
max_depth: u8
ignore_patterns: Box<[Box<str>]>

// Bad - unclear, technical names
flag: bool
n: u8
list: Vec<String>
```

### Option Types

```rust
// Simple values
enabled: bool
max_count: u8  // or u16, u32
min_length: usize

// Enums for fixed choices
#[derive(Deserializable, Merge)]
enum QuoteStyle {
    Single,
    Double,
    Preserve,
}

// Collections (use boxed slices)
patterns: Box<[Box<str>]>
ignore_names: Box<[Box<str>]>

// Complex nested options
#[derive(Deserializable)]
struct AdvancedOptions {
    mode: Mode,
    exclusions: Box<[Box<str>]>,
}
```

## Common Patterns

```rust
// Pattern 1: Boolean option with default false
#[derive(Default)]
struct MyOptions {
    allow_something: Option<bool>,
}

impl Rule for MyRule {
    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let allow = ctx.options().allow_something.unwrap_or(false);
        if allow { return None; }
        // ...
    }
}

// Pattern 2: Enum option with default
#[derive(Default)]
enum Mode {
    #[default]
    Strict,
    Loose,
}

// Pattern 3: Collection option (exclusions)
fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    let options = ctx.options();

    if let Some(exclusions) = &options.exclusions {
        if exclusions.iter().any(|ex| matches_name(ex, name)) {
            return None;  // Excluded
        }
    }

    // Check rule normally
}

// Pattern 4: Numeric threshold
fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    let threshold = ctx.options().max_depth.unwrap_or(3);

    if depth > threshold {
        return Some(());
    }

    None
}
```

## Document Options in Rule

Add options documentation to rule's rustdoc:

```rust
declare_lint_rule! {
    /// Enforces a specific convention for code organization.
    ///
    /// ## Options
    ///
    /// ### `behavior`
    ///
    /// Specifies which behavior to enforce. Accepted values are:
    /// - `"A"` (default): Enforces behavior A
    /// - `"B"`: Enforces behavior B
    /// - `"C"`: Enforces behavior C
    ///
    /// ### `threshold`
    ///
    /// A number between 0-255 (default: 50). Controls sensitivity of detection.
    ///
    /// ### `behaviorExceptions`
    ///
    /// An array of strings. Names listed here are excluded from the rule.
    ///
    /// ## Examples
    ///
    /// ### With default options
    ///
    /// [examples with default behavior]
    ///
    /// ### With `behavior` set to "B"
    ///
    /// ```json
    /// {
    ///   "useThisConvention": {
    ///     "level": "error",
    ///     "options": {
    ///       "behavior": "B"
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// [examples with behavior B]
    pub UseThisConvention {
        version: "next",
        name: "useThisConvention",
        language: "js",
        recommended: false,
    }
}
```

## Configuration Merging Example

```json5
// shared.jsonc (extended configuration)
{
  "linter": {
    "rules": {
      "nursery": {
        "myRule": {
          "options": {
            "behavior": "A",
            "exclusions": ["foo"]
          }
        }
      }
    }
  }
}

// biome.jsonc (user configuration)
{
  "extends": ["./shared.jsonc"],
  "linter": {
    "rules": {
      "nursery": {
        "myRule": {
          "options": {
            "threshold": 30,
            "exclusions": ["bar"]  // Replaces ["foo"], doesn't append
          }
        }
      }
    }
  }
}

// Result after merging:
// behavior: "A" (from shared)
// threshold: 30 (from user)
// exclusions: ["bar"] (user replaces shared)
```

## Test with Options

Create `options.json` in test directory:

```
tests/specs/nursery/useThisConvention/
├── invalid.js
├── valid.js
├── with_behavior_a/
│   ├── options.json
│   ├── invalid.js
│   └── valid.js
└── with_exceptions/
    ├── options.json
    └── valid.js
```

Example `with_behavior_a/options.json`:
```json
{
  "linter": {
    "rules": {
      "nursery": {
        "useThisConvention": {
          "level": "error",
          "options": {
            "behavior": "A",
            "threshold": 10
          }
        }
      }
    }
  }
}
```

Options apply to all test files in that directory.

## Tips

- **Minimize options**: Only add when truly needed
- **Memory efficiency**: Use `Box<[Box<str>]>` not `Vec<String>` for arrays
- **Optional wrapping**: All option fields should be `Option<T>` for proper merging
- **Serde attributes**: Always use `rename_all = "camelCase"` and `deny_unknown_fields`
- **Schema generation**: Use `#[cfg_attr(feature = "schema", derive(JsonSchema))]`
- **Default trait**: Implement or derive `Default` for option types
- **Testing**: Test with multiple option combinations
- **Documentation**: Document each option with examples in the rule's rustdoc
- **Codegen**: Run `just gen-rules && just gen-configuration` after adding options

## References

- Analyzer guide: `crates/biome_analyze/CONTRIBUTING.md` § Rule Options
- Options crate: `crates/biome_rule_options/`
- Deserialize macros: `crates/biome_deserialize_macros/`
- Example rules with options: Search for `type Options =` in `biome_*_analyze` crates
