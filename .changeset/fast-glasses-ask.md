---
"@biomejs/biome": patch
---

Add a new lint rule `useDisposables` for JavaScript, which detects a disposable object assigned to a variable without `using` or `await using` syntax. Disposable objects, which implements `Disposable` or `AsyncDisposable` interface, are intended to dispose after use. Not disposing them can lead some resource or memory leak depending on the implementation.

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
