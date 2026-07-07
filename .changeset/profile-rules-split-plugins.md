---
"@biomejs/biome": patch
---

Fixed [#10795](https://github.com/biomejs/biome/issues/10795): `--profile-rules` now reports timings for each plugin separately as `plugin/<pluginName>`, matching the naming used by plugin suppressions, instead of aggregating all plugins under a single `plugin/plugin` entry.
