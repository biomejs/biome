---
"@biomejs/biome": patch
---

Fixed [#12027](https://github.com/biomejs/biome/issues/12027): Biome's test rules no longer mistake regular method calls named `test`, `it`, or `describe` for tests. For example, [`useValidTestTitle`](https://biomejs.dev/linter/rules/use-valid-test-title/) used to report the following regular expression check as a test with an invalid title:

```js
const isComment = /^\s*#/.test(line);
```
