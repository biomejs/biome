---
"@biomejs/biome": patch
---

Improved type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) on objects: Biome now understands checks on discriminant properties, `in`, `instanceof`, and type guards, and the members that merged declarations and `declare module` add. For example, `result.data?.size` is now reported after `result.state === "loaded"` when only the other variants have `data: null`.
