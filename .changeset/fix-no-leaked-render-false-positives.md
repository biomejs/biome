---
"@biomejs/biome": patch
---

Fixed [#8288](https://github.com/biomejs/biome/issues/8288) and [#10790](https://github.com/biomejs/biome/issues/10790): [`noLeakedRender`](https://biomejs.dev/linter/rules/no-leaked-render/) no longer falsely flags JSX ternary expressions, boolean state variables destructured from `useState()`, or expressions nested inside parentheses.
