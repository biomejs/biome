---
"@biomejs/biome": patch
---

Fixed [#7211](https://github.com/biomejs/biome/issues/7211): [`useOptionalChain`](https://biomejs.dev/linter/rules/use-optional-chain/) now detects negated logical OR chains. The following code is now considered invalid:

```js
!foo || !foo.bar
```
