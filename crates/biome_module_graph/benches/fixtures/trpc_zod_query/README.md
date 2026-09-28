# tRPC + Zod + TanStack Query

A multi-tenant SaaS API: Zod 4 schemas with transforms, refinements, brands,
and discriminated unions feed a tRPC router whose middleware chain narrows the
context (auth, org membership, admin, billing). The client drives the router
through `createTRPCClient` links and `createTRPCOptionsProxy` query, infinite
query, mutation, and subscription options on a query-core `QueryClient`.
Times cold whole-module inference, excluding setup.

```sh
cargo bench -p biome_module_graph --bench type_inference -- trpc_zod_query
```

## Regenerate declarations

Run in an empty temporary directory with `fixture_dir` set to this fixture's
absolute path. Dependencies and MIT licenses are vendored.

```sh
npm install --ignore-scripts --no-audit --no-fund \
  @trpc/server@11.19.0 @trpc/client@11.19.0 @trpc/tanstack-react-query@11.19.0 \
  @tanstack/react-query@5.104.0 @tanstack/query-core@5.104.0 zod@4.6.5 \
  rollup@4.34.8 rollup-plugin-dts@6.5.1 typescript@5.7.3
cp "$fixture_dir/vendor/generate.mjs" ./generate.mjs
node generate.mjs "$fixture_dir/vendor"
cp node_modules/@trpc/server/LICENSE "$fixture_dir/vendor/trpc.LICENSE"
cp node_modules/@tanstack/query-core/LICENSE "$fixture_dir/vendor/tanstack-query.LICENSE"
cp node_modules/zod/LICENSE "$fixture_dir/vendor/zod.LICENSE"
```

Each package is bundled separately. Cross-package imports point at sibling
bundles, and each bundle exports what the samples and its dependents import.
`generate.mjs` applies no patches. It bundles Zod's flat exports for
`import * as z` because the bundled `z` namespace loses types like `z.ZodType`.
React is never resolved: `@trpc/tanstack-react-query` only uses it for
`createTRPCContext`, which is tree-shaken. Type-check samples with
`--strict --skipLibCheck`, ES2022, and bundler resolution.
