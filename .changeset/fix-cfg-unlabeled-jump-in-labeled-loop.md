---
"@biomejs/biome": patch
---

Fixed the control flow analysis of an unlabeled `break` or `continue` inside a labeled loop or `switch`, such as `outer: for (const x of xs) { if (x) break; }`. Biome used to skip the whole enclosing function, so rules like [`noUnreachable`](https://biomejs.dev/linter/rules/no-unreachable/), [`useGetterReturn`](https://biomejs.dev/linter/rules/use-getter-return/) and [`noFallthroughSwitchClause`](https://biomejs.dev/linter/rules/no-fallthrough-switch-clause/) didn't report anything in that function.
