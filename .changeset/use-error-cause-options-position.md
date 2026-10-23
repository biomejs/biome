---
"@biomejs/biome": patch
---

Fixed [#10987](https://github.com/biomejs/biome/issues/10987): [`useErrorCause`](https://biomejs.dev/linter/rules/use-error-cause/) no longer reports a custom error class that receives the `cause` after the second argument, such as `new ErrorWithCode("message", "E_CODE", { cause: err })`. It also accepts the options of `AggregateError` in their third position: `new AggregateError(errors, "message", { cause: err })`.
