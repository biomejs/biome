---
"@biomejs/biome": patch
---

Fixed [#9626](https://github.com/biomejs/biome/issues/9626): `noUnresolvedImports` now keeps namespace members reachable from `export =` type definitions during project scans, including no-inference scans used by the CLI.
