---
"@biomejs/biome": patch
---

Fixed [#10039](https://github.com/biomejs/biome/issues/10039): [`useReadonlyClassProperties`](https://biomejs.dev/linter/rules/use-readonly-class-properties/) now detects unreassigned private members in class expressions and export default classes, not only in class declarations.

The following patterns are now correctly flagged:

```ts
const AnonClass = class {
  #prop = 123;
  constructor() { console.log(this.#prop); }
};

export default class {
  #prop = 123;
  constructor() { console.log(this.#prop); }
}
```
