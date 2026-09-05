---
"@biomejs/biome": patch
---

The [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) rule now avoids resolving complete type tables when raw type information is sufficient, reducing unnecessary type-inference work during linting.
