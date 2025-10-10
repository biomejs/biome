---
"@biomejs/biome": patch
---
Added the nursery lint rule `useErrorCause`.

This rule enforces that errors caught in a `catch` clause are not rethrown without wrapping them in a new `Error` object and specifying the original error as the `cause`. This helps preserve the error’s stack trace and context for better debugging.

It can be configured with the following option:

- `requireCatchParameter`: (default: `true`)
  - When `true`, the rule requires that `catch` clauses have a parameter. If a `throw` statement appears inside a `catch` clause without a parameter, it will be flagged.

**Invalid examples**:

```js
try {
  foo();
} catch {
  throw new Error("fail");
}
```

```js
try {
  foo();
} catch (err) {
  throw new Error(err.message);
}
```

**Valid examples:**

```js
try {
  foo();
} catch (err) {
  throw new Error("fail", { cause: err });
}
```

```js
try {
  foo();
} catch (error) {
  throw new Error("Something went wrong", { cause: error });
}
```

**Valid example** when `requireCatchParameter` is `false`:

Valid:

```js
try {
  foo();
} catch {
  throw new Error("fail");
}
```
