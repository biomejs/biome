---
"@biomejs/biome": patch
---

Fix `useExhaustiveDependencies` so JSX component identifiers (e.g. `<Sub />`) are detected as hook dependencies and included in autofix suggestions.
