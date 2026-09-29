# Zod + TanStack Form

Nested checkout schemas and typed form fields. Deep field reads currently infer
as `Unknown`.
Times cold whole-module inference, excluding setup.

```sh
cargo bench -p biome_module_graph --bench type_inference -- zod_tanstack_form
```

## Regenerate declarations

Run in an empty temporary directory with `fixture_dir` set to this fixture's
absolute path. Dependencies and MIT licenses are vendored.

```sh
npm install --ignore-scripts --no-audit --no-fund \
  zod@3.24.2 @tanstack/form-core@1.0.0 @tanstack/store@0.7.0 \
  rollup@4.34.8 rollup-plugin-dts@6.1.1 typescript@5.7.3
cp "$fixture_dir/vendor/bundle.mjs" ./bundle.mjs
node bundle.mjs "$fixture_dir/vendor"
cp node_modules/zod/LICENSE "$fixture_dir/vendor/zod.LICENSE"
cp node_modules/@tanstack/form-core/LICENSE "$fixture_dir/vendor/tanstack-form.LICENSE"
cp node_modules/@tanstack/store/LICENSE "$fixture_dir/vendor/tanstack-store.LICENSE"
```
