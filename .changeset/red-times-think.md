---
"@biomejs/biome": patch
---

Fixed [#7816](https://github.com/biomejs/biome/issues/7816): [`useHookAtTopLevel`](https://biomejs.dev/linter/rules/use-hook-at-top-level/) no longer reports methods named like hooks when called on another function's result, such as `Reactotron.configure(...).useReactNative(...)`.
