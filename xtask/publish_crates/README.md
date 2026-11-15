# Crate publisher

This tool verifies and publishes every workspace crate whose manifest explicitly sets `publish = true`.

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

## Rate limits

The checked-in serialized GitHub Actions workflow may use crates.io's documented initial bursts: five new crates and thirty updates. New-crate tokens refill one every ten minutes, and update tokens refill one every minute.

Local runs and resumed releases do not assume remaining burst capacity. They start with no token and retain at most one token, enforcing the steady rate even after an idle period. Cargo remains responsible for server-directed upload retries and `Retry-After` handling.
