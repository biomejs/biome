---
"@biomejs/biome": patch
---

Add a new lint rule `useDisposables` for JavaScript, which detects disposable objects assigned to variables without `using` or `await using` syntax. Disposable objects that implement the `Disposable` or `AsyncDisposable` interface are intended to be disposed of after use. Not disposing them can lead to resource or memory leaks, depending on the implementation.

**Invalid:**

```js
function createDisposable(): Disposable {
  return {
    [Symbol.dispose]() {
      // do something
    },
  };
}

const disposable = createDisposable();
```

**Valid:**

```js
function createDisposable(): Disposable {
  return {
    [Symbol.dispose]() {
      // do something
    },
  };
}

using disposable = createDisposable();
```
