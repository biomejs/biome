---
"@biomejs/biome": patch
---

Fixed [#11899](https://github.com/biomejs/biome/issues/11899): setting `linter.domains.next` to `"none"` no longer disables React's [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) and [`useHookAtTopLevel`](https://biomejs.dev/linter/rules/use-hook-at-top-level/) rules.
