---
"@biomejs/biome": patch
---

Reverted a behavior change in [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) that was accidentally included as part of the [#8802](https://github.com/biomejs/biome/issues/8802) fix. The change made method calls on objects (e.g., `props.data.forEach(...)`) report only the object (`props.data`) as a missing dependency instead of the full member expression. This behavior change will be reconsidered separately.
