---
"@biomejs/biome": patch
---

Added the nursery rule [`useRegexpTest`](https://biomejs.dev/linter/rules/use-regexp-test/) that enforces `RegExp.prototype.test()` over `String.prototype.match()` and `RegExp.prototype.exec()` in boolean contexts. `test()` returns a boolean directly, avoiding unnecessary computation of match results.

**Invalid**

``` js
if ("hello world".match(/hello/)) {}
```

**Valid**

```js
if (/hello/.test("hello world")) {}
```
