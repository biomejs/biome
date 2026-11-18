---
"@biomejs/biome": patch
---

Fixed [#13272](https://github.com/biomejs/biome/issues/13272): `biome migrate prettier --write` now formats the Biome configuration file with the migrated formatter options. Previously, the file kept the indentation of the configuration before the migration, so `biome check` reported a formatting error on it right after the migration.
