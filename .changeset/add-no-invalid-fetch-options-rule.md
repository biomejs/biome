---
"@biomejs/biome": patch
---

Added the new nursery rule [`noInvalidFetchOptions`](https://biomejs.dev/linter/rules/no-invalid-fetch-options/), which reports a `body` in the options of `fetch()` and `new Request()` when the method is `GET` or `HEAD`. These calls throw a `TypeError` at runtime. When no `method` is given, the request uses `GET`.

```js
await fetch("/", { method: "GET", body: "foo=bar" });
```
