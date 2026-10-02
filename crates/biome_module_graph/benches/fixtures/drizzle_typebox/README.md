# Drizzle + TypeBox

Seven related tables, nested queries, joins, aggregates, transactions, and generated
validation schemas. Times cold whole-module inference, excluding setup.
Query results currently infer as `Unknown`; TypeBox outputs remain unresolved.

```sh
cargo bench -p biome_module_graph --bench type_inference -- drizzle_typebox
```

## Regenerate declarations

Run in an empty temporary directory with `fixture_dir` set to this fixture's
absolute path. Drizzle uses Apache-2.0; TypeBox uses MIT. Licenses are vendored.

```sh
npm install --ignore-scripts --no-audit --no-fund \
  drizzle-orm@0.45.2 drizzle-typebox@0.3.3 @sinclair/typebox@0.34.52 \
  rollup@4.34.8 rollup-plugin-dts@6.5.1 dts-bundle-generator@9.5.1 \
  typescript@5.7.3 @types/node@22.10.10
cp "$fixture_dir/vendor/generate.mjs" ./generate.mjs
node generate.mjs "$fixture_dir/vendor"
curl -fsSL https://raw.githubusercontent.com/drizzle-team/drizzle-orm/0.45.2/LICENSE \
  -o "$fixture_dir/vendor/drizzle.LICENSE"
cp node_modules/@sinclair/typebox/license "$fixture_dir/vendor/typebox.LICENSE"
```

Type-check samples with `--strict --skipLibCheck`, ES2022, bundler resolution,
and Node types. `skipLibCheck` bypasses upstream Drizzle declaration errors.
