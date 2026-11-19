---
"@biomejs/biome": patch
---

Added the nursery rule [`noJasmineGlobals`](https://biomejs.dev/linter/rules/no-jasmine-globals/), which reports Jasmine globals in Jest tests, such as `spyOn()`, `fail()`, `pending()`, and the `jasmine` object. Jest's default test runner doesn't provide them.

```js
jasmine.DEFAULT_TIMEOUT_INTERVAL = 5000;
```
