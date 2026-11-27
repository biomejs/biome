---
"@biomejs/biome": patch
---

Fixed [#13360](https://github.com/biomejs/biome/issues/13360): [`noUnknownAttribute`](https://biomejs.dev/linter/rules/no-unknown-attribute/) no longer reports React 19's `precedence` prop on `<style>` and `<link>` when the project depends on React 19 or later.
