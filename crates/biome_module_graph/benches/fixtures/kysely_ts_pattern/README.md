# Kysely + ts-pattern

A fourteen-table marketplace schema (`schema.ts`) queried with Kysely aliased
self-joins, recursive CTEs, JSON helpers, `$if`/`$narrowType`, upserts, and
transactions (`queries.ts`), with rows and domain events classified by
ts-pattern (`reports.ts`). Times cold whole-module inference, excluding setup.

```sh
cargo bench -p biome_module_graph --bench type_inference -- kysely_ts_pattern
```

## Regenerate declarations

Run in an empty temporary directory with `fixture_dir` set to this fixture's
absolute path. Dependencies and MIT licenses are vendored.

```sh
npm install --ignore-scripts --no-audit --no-fund \
  kysely@0.29.6 ts-pattern@5.9.0 rollup@4.34.8 \
  rollup-plugin-dts@6.5.1 typescript@5.7.3
cp "$fixture_dir/vendor/generate.mjs" ./generate.mjs
node generate.mjs "$fixture_dir/vendor"
cp node_modules/kysely/LICENSE "$fixture_dir/vendor/kysely.LICENSE"
cp node_modules/ts-pattern/LICENSE "$fixture_dir/vendor/ts-pattern.LICENSE"
```

`kysely.d.ts` also bundles `kysely/helpers/postgres`. `generate.mjs` renames
ts-pattern's `infer` type, which is misparsed once hoisted out of `P`, and
repoints `P.narrow`/`P.infer` references that rollup-plugin-dts leaves behind.
Type-check samples with `--strict --skipLibCheck`, ES2022, and bundler
resolution.
