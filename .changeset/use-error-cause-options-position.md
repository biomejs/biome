---
"@biomejs/biome": patch
---

Fixed [#10987](https://github.com/biomejs/biome/issues/10987): [`useErrorCause`](https://biomejs.dev/linter/rules/use-error-cause/) now finds the `cause` in the last argument of a custom error class whose second argument isn't an object, such as `new ErrorWithCode("message", "E_CODE", { cause: err })`. Built-in errors keep their usual options position, which is the third argument for `AggregateError`: `new AggregateError(errors, "message", { cause: err })`.
