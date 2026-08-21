---
name: eslint-migrate-options
description: Preserve configurable ESLint rule options in `biome migrate eslint` by modeling source options, converting them to Biome options, wiring typed rule variants, and adding migration fixtures. Use for custom option migration; not for generated severity-only mapping or general rule-option design.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

# ESLint Option Migration

Use a custom migrator only when an existing Biome rule should preserve meaningful ESLint options. Severity-only migration belongs to the generated rule mapping.

## Preconditions

Confirm before editing:

1. The Biome rule and its option type already exist.
2. Rule metadata identifies the ESLint source rule.
3. Generated severity migration already exists or will be regenerated.
4. Official ESLint or plugin documentation establishes the exact option shape and defaults.
5. The supported Biome semantics are clear, including unsupported source options.

Load `lint-rule-development` when the Biome rule or its own options still need implementation.

## Pipeline

Custom migration has four parts:

1. A plugin-specific type deserializes the ESLint payload.
2. A conversion maps source semantics into the Biome option type.
3. The shared ESLint `Rule` enum recognizes the rule as a typed variant.
4. `migrate_eslint_rule()` installs the resulting Biome configuration after common severity handling.

## Key Files

| File | Responsibility |
| --- | --- |
| `eslint_eslint.rs` | Shared config model, `Rule`, `RuleConf<T>`, and dispatch |
| `eslint_unicorn.rs` | Unicorn option types and conversions |
| `eslint_typescript.rs` | TypeScript ESLint option types and conversions |
| `eslint_jsxa11y.rs` | JSX accessibility option types and conversions |
| `eslint_to_biome.rs` | Main conversion and custom migration arms |
| `eslint_any_rule_to_biome.rs` | Generated severity mapping |
| `tests/specs/migrate_eslint/` | Fixture-driven migration snapshots |

Use the plugin-specific module matching the source rule and inspect a current migrator with a similar payload and Biome configuration type.

## Model Source Options

Model the ESLint JSON shape rather than the Biome destination shape.

- Preserve source nesting until conversion.
- Use optional fields where the source allows omission.
- Match current deserialization naming and default conventions from neighboring option types.
- Model fields needed to deserialize supported real configurations.
- Verify unsupported source fields do not make valid configurations fail; whether they need explicit fields depends on the current deserializer contract.

Do not assume source and destination defaults are equivalent.

## Convert Semantics

Implement conversion next to the source option type.

- Rename concepts when ESLint and Biome use different terminology.
- Drop unsupported behavior deliberately.
- Avoid emitting empty nested option objects.
- Preserve omission when attaching Biome defaults would change severity-only behavior.
- Use a small helper when normalization or conditional construction has a domain meaning.

The conversion should make unsupported and default behavior obvious from its branches rather than relying on field-for-field assignment.

## Add the Typed Rule Variant

Update all typed dispatch points in `eslint_eslint.rs`:

- add `RuleConf<SourceOptions>` to `Rule`;
- return the exact ESLint rule name from `Rule::name()`;
- deserialize that name into the typed variant before the catch-all fallback.

Search the enum's matches after adding the variant so no dispatch site remains stale.

## Wire Migration

Add the custom arm in `migrate_eslint_rule()` and call `migrate_eslint_any_rule()` first. The common path owns severity tracking, unsupported-rule reporting, and deduplication.

Use the destination rule's actual group and configuration type:

- `RuleFixConfiguration::WithOptions` for fixable rules;
- `RuleConfiguration::WithOptions` for non-fixable rules.

Choose the `RuleConf` access pattern matching the source schema:

- `option_or_default()` when severity-only input should map through source defaults;
- explicit `RuleConf::Option` matching when options should attach only when supplied;
- `into_vec()` for array payloads requiring aggregation or normalization.

Copy a current analogous arm rather than relying on remembered struct fields.

## Tests

Add focused fixtures under `crates/biome_cli/tests/specs/migrate_eslint/` for:

- severity-only configuration;
- each supported option mode;
- unsupported fields present in otherwise valid input;
- empty and partially specified nested objects;
- differing ESLint and Biome defaults;
- pre-existing Biome configuration when merge behavior matters.

Run the focused migration tests and inspect adjacent snapshots. Run analyzer option tests too when migration output depends on rule semantics.

Typical commands:

```shell
cargo check -p biome_cli
cargo test -p biome_cli migrate_eslint
```

## Review Checklist

- Source types match official ESLint configuration.
- Conversion maps semantics rather than names mechanically.
- Severity-only behavior remains correct.
- Typed variant, name, deserialization, and migration matches are complete.
- Common severity migration runs before custom option installation.
- Group and fixability match the destination rule.
- Fixtures cover omission, defaults, unsupported fields, and nested options.

## References

- Migration implementation: `crates/biome_cli/src/execute/migrate/`
- Rule options: `crates/biome_rule_options/src/`
- Generated mapping: `xtask/codegen/src/generate_migrate_eslint.rs`
- ESLint documentation: https://eslint.org/docs/latest/use/configure/rules
