---
name: eslint-migrate-options
description: Guide for implementing ESLint-to-Biome rule option migrators inside `biome migrate eslint`. Use whenever you add or update a Biome lint rule that has an ESLint source rule with configurable options, need to deserialize plugin-specific ESLint options, or need custom migration logic beyond the auto-generated severity mapping.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

## Purpose

Use this skill when a Biome lint rule already exists and `biome migrate eslint` should preserve more than just the rule severity.

This skill is specifically for cases where an ESLint rule has options that need to be:

- deserialized from ESLint config
- translated into Biome rule options
- wired into the migrate pipeline
- tested through migrator spec fixtures without depending on CLI tests

Do not use this skill for severity-only migrations. Those are usually covered by the generated rule mapping in `eslint_any_rule_to_biome.rs`.

## Before You Edit

Confirm these points first:

1. The target Biome rule already exists and already has its own options type in `crates/biome_rule_options/src/`.
2. The Biome rule metadata already declares the ESLint source rule, so severity-only migration exists or can be generated.
3. The ESLint rule really has user-facing options worth preserving.
4. You have checked the ESLint rule docs or source so you know the exact option shape, defaults, and any plugin-specific quirks.

If any of those are missing, fix that first before adding a migrator.

## Mental Model

The migrate pipeline has two layers:

1. Generated severity mapping: `eslint_any_rule_to_biome.rs`
2. Hand-written option migration: plugin-specific structs plus a custom arm in `migrate_eslint_rule()`

The generated file already handles the common case:

```json
{
  "some-rule": "error"
}
```

Add a custom migrator only when a config like this should keep its options:

```json
{
  "some-rule": ["error", { "someOption": true }]
}
```

## Key Files

| File | Role |
| - | - |
| `crates/biome_cli/src/execute/migrate/eslint_eslint.rs` | Shared ESLint config model, `Rule` enum, `RuleConf<T>`, deserialization entry points |
| `crates/biome_cli/src/execute/migrate/eslint_unicorn.rs` | `eslint-plugin-unicorn` option structs and conversions |
| `crates/biome_cli/src/execute/migrate/eslint_typescript.rs` | `@typescript-eslint` option structs and conversions |
| `crates/biome_cli/src/execute/migrate/eslint_jsxa11y.rs` | `jsx-a11y` option structs and conversions |
| `crates/biome_cli/src/execute/migrate/eslint_to_biome.rs` | Main conversion logic, including `migrate_eslint_rule()` |
| `crates/biome_cli/tests/specs/migrate_eslint/` | Fixture-driven snapshot tests for custom ESLint migrators |
| `crates/biome_cli/src/execute/migrate/eslint_any_rule_to_biome.rs` | Generated severity mapping for all known ESLint-backed rules |
| `xtask/codegen/src/generate_migrate_eslint.rs` | Codegen for the generated rule mapping |

Use the plugin-specific file that matches the source ESLint rule. Keep option structs close to similar migrators so future edits stay discoverable.

## Recommended Workflow

### Step 1: Inspect an Existing Migrator First

Before writing anything new, find a nearby rule that already migrates options. Reuse its shape if the target rule is in the same plugin or has the same Biome configuration type (`RuleConfiguration` vs `RuleFixConfiguration`).

This saves time and helps match the patterns already used in `migrate_eslint_rule()`.

### Step 2: Model the ESLint Options Exactly

Add structs in the correct plugin file. Match ESLint's option payload shape, not Biome's.

```rust
use biome_deserialize_macros::Deserializable;

#[derive(Clone, Debug, Default, Deserializable)]
pub(crate) struct EslintMyRuleOptions {
    some_option: Option<u8>,
    another_option: bool,
    nested: EslintMyRuleNestedOptions,
}

#[derive(Clone, Debug, Default, Deserializable)]
pub(crate) struct EslintMyRuleNestedOptions {
    threshold: Option<u8>,
}
```

Guidelines:

- Use snake_case Rust field names; `Deserializable` handles camelCase JSON keys.
- Use `Option<T>` for fields that can be omitted.
- Keep unsupported ESLint fields in the struct if they appear in the config shape; ignore them later during conversion.
- Prefer mirroring the real JSON nesting instead of flattening early.

### Step 3: Convert ESLint Options Into Biome Options

Implement `From<Eslint...Options> for biome_rule_options::...` in the same plugin file.

```rust
impl From<EslintMyRuleOptions> for my_rule::MyRuleOptions {
    fn from(value: EslintMyRuleOptions) -> Self {
        Self {
            some_option: value.some_option,
            different_name: Some(value.another_option),
            threshold: value.nested.threshold,
        }
    }
}
```

Focus on semantic mapping, not field-for-field copying:

- rename concepts when ESLint and Biome use different names
- drop unsupported knobs deliberately
- preserve defaults only when they match Biome's behavior
- add small helper functions when the conversion needs filtering or normalization

If an ESLint option should only be emitted when at least one nested field is set, use a helper that returns `Option<_>` rather than constructing empty Biome option objects.

### Step 4: Add a Typed `Rule` Variant

In `eslint_eslint.rs`, add a `Rule` enum variant using `RuleConf<T>`:

```rust
pub(crate) enum Rule {
    // ...
    MyPluginMyRule(RuleConf<eslint_my_plugin::EslintMyRuleOptions>),
}
```

Then update both of these places:

- `Rule::name()` so the variant returns the ESLint rule name
- `Rules::deserialize` so the ESLint rule string deserializes into your typed variant before the catch-all fallback

Example:

```rust
Self::MyPluginMyRule(_) => Cow::Borrowed("my-plugin/my-rule"),
```

```rust
"my-plugin/my-rule" => {
    if let Some(conf) = RuleConf::deserialize(ctx, &value, name) {
        result.insert(Rule::MyPluginMyRule(conf));
    }
}
```

Order matters in `Rules::deserialize`: put the explicit match before the fallback `rule_name =>` arm.

### Step 5: Wire the Rule Into `migrate_eslint_rule()`

Add a match arm in `crates/biome_cli/src/execute/migrate/eslint_to_biome.rs`.

Always call `migrate_eslint_any_rule()` first. It handles severity tracking, unsupported-rule reporting, and deduplication.

Pick the configuration type that matches the Biome rule:

- `RuleFixConfiguration::WithOptions` for fixable rules
- `RuleConfiguration::WithOptions` for non-fixable rules

Typical fixable rule pattern:

```rust
eslint_eslint::Rule::MyPluginMyRule(conf) => {
    if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
        let group = rules.style.get_or_insert_with(Default::default);
        if let SeverityOrGroup::Group(group) = group {
            group.my_biome_rule = Some(biome_config::RuleFixConfiguration::WithOptions(
                biome_config::RuleWithFixOptions {
                    level: conf.severity().into(),
                    fix: None,
                    options: conf.option_or_default().into(),
                },
            ));
        }
    }
}
```

Typical non-fixable rule pattern:

```rust
eslint_eslint::Rule::MyPluginMyRule(conf) => {
    if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
        let group = rules.style.get_or_insert_with(Default::default);
        if let SeverityOrGroup::Group(group) = group {
            group.my_biome_rule = Some(biome_config::RuleConfiguration::WithOptions(
                biome_config::RuleWithOptions {
                    level: conf.severity().into(),
                    options: conf.option_or_default().into(),
                },
            ));
        }
    }
}
```

Replace `rules.style` with the correct group (`a11y`, `complexity`, `correctness`, `nursery`, `performance`, `security`, `style`, `suspicious`).

### Step 6: Choose the Right `RuleConf` Access Pattern

Do not force every migrator into the same shape. The current codebase uses different access patterns depending on the ESLint rule schema.

Use the one that matches the source rule:

- `conf.option_or_default()` when the rule has one options object and severity-only configs should fall back to defaults
- `if let RuleConf::Option(severity, rule_options) = conf` when the migration should only attach options if the user explicitly provided the object
- `conf.into_vec()` when the rule uses array-style payloads that need custom aggregation or normalization

If unsure, inspect an existing migrator with a similar ESLint schema and copy that pattern.

## Common Pitfalls

- Adding a custom migrator when severity-only migration was enough
- Modeling the Biome options instead of the ESLint JSON shape
- Forgetting to update both `Rule::name()` and `Rules::deserialize`
- Putting the deserialization arm after the fallback arm
- Writing a custom match arm but skipping `migrate_eslint_any_rule()`
- Using `RuleFixConfiguration` for a rule that is not fixable, or the inverse
- Emitting empty option objects that change semantics compared with the default Biome config
- Ignoring ESLint fields during deserialization by leaving them out of the struct, causing valid configs to fail to deserialize

## Worked Example

`unicorn/numeric-separators-style` is a good reference because the names do not line up perfectly.

ESLint uses `number`; Biome uses `decimal`. ESLint also exposes `onlyIfContainsSeparator`, which Biome does not support, so the migrator ignores it.

```rust
#[derive(Clone, Debug, Default, Deserializable)]
pub(crate) struct NumericSeparatorsStyleOptions {
    number: EslintNumericSeparatorTypeOptions,
    binary: EslintNumericSeparatorTypeOptions,
    octal: EslintNumericSeparatorTypeOptions,
    hexadecimal: EslintNumericSeparatorTypeOptions,
}

#[derive(Clone, Debug, Default, Deserializable)]
pub(crate) struct EslintNumericSeparatorTypeOptions {
    minimum_digits: Option<u8>,
    group_length: Option<u8>,
}

impl From<NumericSeparatorsStyleOptions>
    for use_numeric_separators::UseNumericSeparatorsOptions
{
    fn from(value: NumericSeparatorsStyleOptions) -> Self {
        Self {
            binary: some_if_set(value.binary),
            octal: some_if_set(value.octal),
            decimal: some_if_set(value.number),
            hexadecimal: some_if_set(value.hexadecimal),
        }
    }
}

fn some_if_set(
    options: EslintNumericSeparatorTypeOptions,
) -> Option<use_numeric_separators::NumericLiteralSeparatorOptions> {
    if options.minimum_digits.is_some() || options.group_length.is_some() {
        Some(options.into())
    } else {
        None
    }
}
```

This is the pattern to follow when:

- ESLint names differ from Biome names
- nested objects may be partially unset
- empty nested config should collapse to `None`

## Testing Checklist

At minimum, verify all of these:

1. Severity-only ESLint config still migrates correctly.
2. ESLint config with options produces the expected Biome options.
3. Unsupported ESLint knobs do not break deserialization.
4. Empty or partially specified nested options do not emit incorrect Biome config.

Use the migrator spec fixtures in `crates/biome_cli/tests/specs/migrate_eslint/` as the default test path for custom migrators.

- Add one fixture file per case.
- Keep the fixture focused on `eslint` input and pre-migration `biome` config input.
- Let the generated test runner in `eslint_to_biome.rs` discover the file and write the adjacent `.snap.new`.
- Prefer adding or updating these fixture snapshots instead of writing a new full CLI test when you are verifying custom option migration behavior.
- After inspecting snapshot differences, use `cargo insta accept` to accept valid new snapshots, or `cargo insta reject` to reject invalid ones and keep iterating.

CLI tests in `crates/biome_cli/tests/commands/migrate_eslint.rs` should be treated as smoke coverage for command wiring and end-to-end behavior, not the primary place to test custom migrators.

Useful commands:

```shell
cargo check -p biome_cli
cargo test -p biome_cli migrate_eslint
```

When the rule itself has analyzer behavior tied to the options, run targeted analyzer tests too:

```shell
cargo test -p biome_js_analyze my_rule_name
```

## Review Checklist

Before finishing, confirm:

- the typed `Rule` variant exists
- `Rule::name()` returns the exact ESLint rule name
- `Rules::deserialize` has an explicit arm before the fallback
- the plugin-specific ESLint option structs match the real ESLint schema
- the `From` impl maps semantics correctly, not just names mechanically
- `migrate_eslint_any_rule()` is still called first
- the chosen Biome rule group and configuration type are correct
- migrator spec fixtures cover both severity-only and option-bearing configs when relevant

## References

- `crates/biome_cli/src/execute/migrate/`
- `crates/biome_cli/src/execute/migrate/eslint_eslint.rs`
- `crates/biome_cli/src/execute/migrate/eslint_to_biome.rs`
- `crates/biome_cli/src/execute/migrate/eslint_any_rule_to_biome.rs`
- `crates/biome_rule_options/src/`
- `xtask/codegen/src/generate_migrate_eslint.rs`
