---
"@biomejs/biome": patch
---

Fixed [#7344](https://github.com/biomejs/biome/issues/7344). [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer reports interfaces defined in global declarations.

Interfaces declared in global declarations augment existing interfaces.
Thus, they must be ignored.

In the following example, `useNamingConvention` reported `HTMLElement`.
It is now ignored.

```ts
export {};
declare global {
  interface HTMLElement {
    foo(): void;
  }
}
```
