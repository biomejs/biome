---
"@biomejs/biome": patch
---

Fixed [#11899](https://github.com/biomejs/biome/issues/11899): disabling a domain no longer disables rules that also belong to another enabled domain. For example, with `"domains": { "react": "all", "next": "none" }`, [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) and [`useHookAtTopLevel`](https://biomejs.dev/linter/rules/use-hook-at-top-level/) are now enabled.
