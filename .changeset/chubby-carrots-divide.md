---
"@biomejs/biome": patch
---

Fixed [#6391](https://github.com/biomejs/biome/issues/6391): the rule [`noUselessFragments`](https://biomejs.dev/linter/rules/no-useless-fragments/) no longer reports a fragment that contains whitespaces which aren't trimmed by the runtime.
