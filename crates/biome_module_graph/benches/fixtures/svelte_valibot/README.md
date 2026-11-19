# Svelte + Valibot

Nested validation schemas and chained Svelte stores, using TypeScript without
Svelte markup. Unsupported types may infer as `Unknown`.
Times cold whole-module inference, excluding setup.

```sh
cargo bench -p biome_module_graph --bench type_inference -- svelte_valibot
```

## Regenerate declarations

Run in an empty temporary directory with `fixture_dir` set to this fixture's
absolute path. Dependencies and MIT licenses are vendored.

```sh
npm install --ignore-scripts --no-audit --no-fund \
  svelte@5.57.0 valibot@1.5.0 rollup@4.34.8 \
  rollup-plugin-dts@6.1.1 typescript@5.7.3
cp "$fixture_dir/vendor/generate.mjs" ./generate.mjs
node generate.mjs "$fixture_dir/vendor"
cp node_modules/svelte/LICENSE.md "$fixture_dir/vendor/svelte.LICENSE"
cp node_modules/valibot/LICENSE.md "$fixture_dir/vendor/valibot.LICENSE"
```
