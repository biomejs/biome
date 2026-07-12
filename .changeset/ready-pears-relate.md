---
"@biomejs/biome": patch
---

Fixed [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) so it correctly analyzes imported namespace exports, cyclic imports, aliased generic callables, shadowed generics, intersection members, computed properties, inherited static methods, awaited unions, explicitly typed arrays, generic class methods that access properties through `this`, and overloaded calls with incomplete or union arguments.

Also fixed [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) for overloaded and rest-parameter calls, [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) for bigint unions, [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) when type information is unavailable, and [`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) when return inference is incomplete. Type-aware rules, including [`useAwaitThenable`](https://biomejs.dev/linter/rules/use-await-thenable/) and [`noBaseToString`](https://biomejs.dev/linter/rules/no-base-to-string/), now suppress diagnostics and fixes when Promise, member, nullish, stringification, generic-constraint, normalization, or substitution traversal cannot complete instead of reporting from partial information.
