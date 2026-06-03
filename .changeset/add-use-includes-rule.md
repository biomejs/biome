---
"@biomejs/biome": minor
---

Added [`useIncludes`](https://biomejs.dev/linter/rules/use-includes/) to the nursery group. This rule flags comparisons of `String.prototype.indexOf()` or `Array.prototype.indexOf()` against `-1` and suggests replacing them with the clearer `includes()` / `!includes()` form.
