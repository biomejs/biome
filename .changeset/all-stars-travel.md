---
"@biomejs/biome": patch
---
Added the nursery lint rule [`useIncludes`](https://biomejs.dev/linter/rules/use-includes/).
Enforce the use of `.includes()` over `.indexOf()` when checking for the presence of an element.
The rule also suggests using `String.prototype.includes()` over `.test()` for simple regular expressions.

Invalid:
```js
"foo".indexOf("o") !== -1;
["a", "b", "c"].indexOf("a") === -1
/a/.test("abc")
```
