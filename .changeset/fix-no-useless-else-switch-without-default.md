---
"@biomejs/biome": patch
---

Fixed [#12529](https://github.com/biomejs/biome/issues/12529): [`noUselessElse`](https://biomejs.dev/linter/rules/no-useless-else/) no longer reports an `else` clause when the `if` block ends with a `switch` statement that has no `default` clause. If no case matches, such a `switch` doesn't exit, so the `else` clause is needed.
