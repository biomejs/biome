---
"@biomejs/biome": patch
---

Fixed [#13272](https://github.com/biomejs/biome/issues/13272): `biome migrate prettier --write` now formats the Biome configuration files, including nested ones, with the migrated formatter options and the options they inherit through `extends`. Previously, the files kept the indentation of the configuration before the migration, so `biome check` reported a formatting error on them right after the migration.
