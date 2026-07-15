---
"@biomejs/biome": patch
---

Improved Biome's type inference for [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/). Biome now resolves aliased generic callables and preserves shadowed generic parameters. For example, `noFloatingPromises` now detects the unhandled Promise returned through this generic callable alias:

```ts
type AsyncCallback<T> = () => Promise<T>;
declare const callback: AsyncCallback<void>;
callback();
```
