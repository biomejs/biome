---
"@biomejs/biome": patch
---

Fixed [#9899](https://github.com/biomejs/biome/issues/9899): `--reporter=json` now normalizes file paths to use forward slashes on Windows. Previously, paths were emitted with backslashes (e.g., `typescript\src\file.tsx`), which was inconsistent with other reporters and could break tooling that expects forward slashes.
