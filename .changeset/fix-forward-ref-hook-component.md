---
"@biomejs/biome": patch
---

Fixed [#9195](https://github.com/biomejs/biome/issues/9195): [`useHookAtTopLevel`](https://biomejs.dev/linter/rules/use-hook-at-top-level/) no longer reports hooks in named `forwardRef` components that receive a `ref` parameter.
