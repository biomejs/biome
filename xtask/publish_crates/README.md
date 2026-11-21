# Crate publisher

CI publishes individual workspace crates marked `publish = true`. It does not publish language
bundles yet.

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

## Bundle plan

[`bundle-plan.toml`](bundle-plan.toml) describes which workspace crates would ship together as
one package. It models the release without moving source files or changing what CI publishes.
The file has three required keys:

- `roots`: A nonempty list of existing publishable crate names to start from. Their workspace
  dependencies are included automatically. List source crate names here, not bundle names.
- `private`: A list of package names to leave out. This does not change a crate's `publish` flag.
  If the plan still needs a package on this list, the command reports it as a blocker instead of
  silently dropping it. Use `[]` if there are no planned exclusions.
- `[bundles]`: A table that maps each planned package name to at least two publishable source
  crates. If the plan reaches any member, it includes all members and their dependencies. A
  source crate can belong to only one bundle, a bundle name cannot match an existing publishable
  crate, and every bundle must be reachable from a root.

For example, `biome_graphql_parser` is a root and a member of the `biome_graphql` bundle. The
plan lists one `biome_graphql` package instead of separate GraphQL syntax, factory, parser,
formatter, and semantic crates. JSON and YAML are not in `[bundles]`, so their parser, formatter,
syntax, and factory crates keep their own package names. Their analyzers remain workspace-only.

Run the planner from the Biome checkout:

```shell
cargo run --locked -p xtask_publish_crates -- bundle-plan
```

It first validates the current publishable crates. A normal or build dependency on a
workspace-only crate fails that validation, even if the dependency is optional. The planner then
follows workspace dependencies from `roots`, including dev dependencies with a version
requirement. It omits packages it doesn't reach, rejects cycles, and prints the result in
dependency order.

CI runs `bundle-plan --check` before the publish dry run. The `--check` flag makes blockers fail
the command. Neither mode assembles, builds, or uploads bundles.

## Verify a staged bundle

From the Biome checkout, point `verify` at a staged crate outside the checkout:

```shell
cargo run --locked -p xtask_publish_crates -- verify --staged-bundle /path/to/staged/crate
```

The staged crate needs its sources, a manifest, and a lockfile (its own or its staging workspace's).
Verification replaces the staged `README.md` with one that has the shared Biome banner and sets
`readme = "README.md"` in the staged manifest. It checks that Cargo will include the README, then
packages and builds the crate under the staging directory's `target/package`. It does not upload
anything. CI still runs `verify` without this option.

## Rate limits

The checked-in serialized GitHub Actions workflow may use crates.io's documented initial bursts: five new crates and thirty updates. New-crate tokens refill one every ten minutes, and update tokens refill one every minute.

Local runs and resumed releases do not assume remaining burst capacity. They start with no token and retain at most one token, enforcing the steady rate even after an idle period. Cargo remains responsible for server-directed upload retries and `Retry-After` handling.
