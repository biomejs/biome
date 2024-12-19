---
biome_deserialize: minor
---

# Add the `indexmap` Cargo feature

This is a **breaking change**.

Previously, `biome_deserialize` required `serde` and `indexmap` as dependencies.
Its dependencies are now optional.

If you need the implementation of `Deserializable` for `indexmap::IndexMap` and `indexmap::IndexSet`, then you have to use the `indexmap` feature.
Update your `Cargo.toml` as follows:

```diff
  [dependencies]
- biome_deserialize = { version: "<version>" }
+ biome_deserialize = { version: "<version>", features = ["indexmap"] }
```
