---
"@biomejs/biome": patch
---

[`useDisposables`](https://biomejs.dev/linter/rules/use-disposables/) now detects undisposed Disposable values typed through a generic type alias.

```ts
type Id<T> = T;
declare function open(): Id<Disposable>;
const handle = open();
```
