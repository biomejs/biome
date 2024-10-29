---
biome_deserialize: minor
---

## Remove `biome_deserialize::StringSet`

This is a **breaking change**.

`biome_deserialize::StringSet` is now removed.
Use `indexmap::IndexSet<String>` instead.

As a consequence the cargo feature `schema` has been removed.
