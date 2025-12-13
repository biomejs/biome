---
"@biomejs/biome": patch
---

The [`useRegexpExec`](https://biomejs.dev/linter/rules/use-regexp-exec/) rule now covers cases where `new RegExp()` is called directly as an argument to `String#match`.

**Invalid:**

```js
"something".match(new RegExp(/thing/));
```
