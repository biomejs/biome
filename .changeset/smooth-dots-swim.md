---
"@biomejs/biome": minor
---

Added the new linter domain `types`. This is a domain that enables all rules that require the type inference engine.

As opposed to the `project` domain, which only enables rules that require the module graph to function.

The following **nursery** rules have been moved to the `types` domain:
- `useArraySortCompare`
- `useAwaitThenable`
- `useFind`
- `useRegexpExec`
- `noUnnecessaryConditions`
- `noMisusedPromises`
- `noFloatingPromises`
