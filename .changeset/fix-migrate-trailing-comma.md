---
"@biomejs/biome": patch
---

Fixed [#ISSUE](https://github.com/biomejs/biome/issues/ISSUE): `biome migrate` no longer emits an invalid trailing comma when a renamed rule (such as `noConsoleLog` → `noConsole`) is the last member of its rule group. Previously this produced malformed output that aborted the migration of a strict-JSON `biome.json` with a parsing error.
