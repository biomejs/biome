# Hono + Valibot

A project-management REST service built from long Hono method chains, nested
`.route()` mounts under `.basePath()`, typed `c.var` middleware, and Valibot
validators on JSON, query, path, and header targets, consumed by a typed `hc`
client. Times cold whole-module inference, excluding setup.

```sh
cargo bench -p biome_module_graph --bench type_inference -- hono_valibot
```

## Regenerate declarations

Run in an empty temporary directory with `fixture_dir` set to this fixture's
absolute path. Dependencies and MIT licenses are vendored;
`@hono/valibot-validator` is MIT but ships no license file.

```sh
npm install --ignore-scripts --no-audit --no-fund \
  hono@4.13.9 @hono/valibot-validator@0.6.1 valibot@1.5.0 rollup@4.34.8 \
  rollup-plugin-dts@6.5.1 typescript@5.7.3
cp "$fixture_dir/vendor/generate.mjs" ./generate.mjs
node generate.mjs "$fixture_dir/vendor"
cp node_modules/hono/LICENSE "$fixture_dir/vendor/hono.LICENSE"
cp node_modules/valibot/LICENSE.md "$fixture_dir/vendor/valibot.LICENSE"
```

`generate.mjs` applies no patches: the bundle's `Hono$1`, `HonoRequest$1`,
`Set$1`, and `Response$1` renames all point at the right local declarations.
Type-check samples with `--strict --skipLibCheck`, ES2022, and bundler
resolution.
