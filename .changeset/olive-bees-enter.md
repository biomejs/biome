---
"@biomejs/biome": patch
---

Fixed [#11317](https://github.com/biomejs/biome/issues/11317): [`noSvgWithoutTitle`](https://biomejs.dev/linter/rules/no-svg-without-title/) no longer reports an `svg` that uses the boolean shorthand `aria-hidden` (equivalent to `aria-hidden={true}` in React).
