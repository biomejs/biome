---
"@biomejs/biome": patch
---

Fixed [#9899](https://github.com/biomejs/biome/issues/9899): `--reporter=json` now properly escapes backslashes in file paths. Previously, Windows paths containing backslashes were emitted unescaped, producing invalid JSON output.
