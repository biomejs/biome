---
"@biomejs/biome": patch
---

Fixed [#10514](https://github.com/biomejs/biome/issues/10514): [`noLeakedRender`](https://biomejs.dev/linter/rules/no-leaked-render/) no longer reports typed values that can never leak a falsy primitive into the rendered output.
