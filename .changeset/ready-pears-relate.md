---
"@biomejs/biome": patch
---

Fixed [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) so it correctly analyzes imported namespace exports, cyclic imports, aliased generic callables, shadowed generics, intersection members, computed properties, and awaited unions.

Also fixed [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) for bigint unions, [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) when type information is unavailable, and [`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) when return inference is incomplete.
