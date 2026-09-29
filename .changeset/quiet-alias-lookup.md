---
"@biomejs/biome": patch
---

Fixed [#11810](https://github.com/biomejs/biome/issues/11810) and [#11813](https://github.com/biomejs/biome/issues/11813): type-aware rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) and [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) no longer take several seconds when a member is accessed on a recursive generic type alias, such as react-hook-form's `FieldPathValue` or zustand's `Mutate`.
