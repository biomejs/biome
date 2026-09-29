---
"@biomejs/biome": patch
---

Biome's type inference now evaluates `typeof` applied to a class to `"function"`, as it already did for functions.

[`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) can therefore report a comparison such as the one below:

```ts
class Service {}
if (typeof Service === "string") {} // now reported
```
