# Crate publisher

CI publishes individual workspace crates marked `publish = true`. It does not publish language
bundles yet.

## Language bundles

The plan is to publish one crate per language without moving the source crates. For example,
the GraphQL source stays in separate workspace crates. The release would copy that code into
`biome_graphql` in a staging directory. Rust users would add one GraphQL dependency instead of
separate crates for parsing, formatting, and semantic analysis.

JSON and YAML are exceptions. Their parsers stay in the language bundles, but their formatters
stay separate. The formatters depend on the shared deserializer, which needs the parsers.
Bundling either formatter with its parser would create a dependency cycle.

`biome_db`, `biome_analyze`, and `biome_analyze_macros` stay public as separate crates. The
language-specific analyzers, module graph, and resolver stay in the workspace.

## Release workflow

Run the full locked Cargo dry-run verification on a clean release revision:

```shell
cargo run --locked -p xtask_publish_crates -- verify
```

Print the publication plan without uploading:

```shell
cargo run --locked -p xtask_publish_crates -- publish
```

Execute a previously verified Git revision. The verified revision must have the same Git tree as `HEAD`:

```shell
cargo run --locked -p xtask_publish_crates -- publish --execute --verified-revision <revision>
```

The publisher packages the complete public-crate set, compares local archive checksums with crates.io, skips matching versions, and publishes the remaining crates in dependency order. A rerun reconstructs progress from crates.io; it does not use a local checkpoint file.

## Inspect the bundle plan

```shell
cargo run --locked -p xtask_publish_crates -- bundle-plan
```

[`bundle-plan.toml`](bundle-plan.toml) lists the starting crates, bundle members, and planned exclusions.
The command first validates the current publishable crates. A dependency on a crate that is
already workspace-only fails that validation before the plan runs.

The command follows dependencies from the starting crates and prints the planned packages in
dependency order. It omits public crates that the plan doesn't reach unless they are part of a
selected bundle. If a planned exclusion is still public and the plan needs it, the command lists
it as a blocker. It checks for cycles but does not build or upload anything.

## Rate limits

The checked-in serialized GitHub Actions workflow may use crates.io's documented initial bursts: five new crates and thirty updates. New-crate tokens refill one every ten minutes, and update tokens refill one every minute.

Local runs and resumed releases do not assume remaining burst capacity. They start with no token and retain at most one token, enforcing the steady rate even after an idle period. Cargo remains responsible for server-directed upload retries and `Retry-After` handling.
