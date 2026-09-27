# Effect + ArkType

ArkType scopes with cyclic aliases, morphs, generics, and discriminated event
unions, consumed by an Effect order service built from tagged errors, context
tags, layers, generators, retry schedules, and `Match`.
Times cold whole-module inference, excluding setup.

```sh
cargo bench -p biome_module_graph --bench type_inference -- effect_arktype
```

## Regenerate declarations

Run in an empty temporary directory with `fixture_dir` set to this fixture's
absolute path. Dependencies and MIT licenses are vendored.

```sh
npm install --ignore-scripts --no-audit --no-fund \
  effect@3.22.2 arktype@2.2.5 rollup@4.34.8 \
  rollup-plugin-dts@6.5.1 typescript@5.7.3
cp "$fixture_dir/vendor/generate.mjs" ./generate.mjs
node generate.mjs "$fixture_dir/vendor"
cp node_modules/effect/LICENSE "$fixture_dir/vendor/effect.LICENSE"
cp node_modules/arktype/LICENSE "$fixture_dir/vendor/arktype.LICENSE"
```

`generate.mjs` patches known rollup-plugin-dts renaming bugs and drops module
augmentations that cannot apply inside a bundle. Type-check samples with
`--strict --skipLibCheck`, ES2022, and bundler resolution.
