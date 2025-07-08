---
"@biomejs/biome": patch
---

Fixed [#6719](https://github.com/biomejs/biome/issues/6719): The `noInvalidUseBeforeDeclaration` rule covers additional use cases.

Examples:

```ts
type Bar = {[BAR]: true;};
const BAR = 'bar';
```

```ts
interface Bar {child:  {grandChild: {[BAR]: typeof BAR; enumFoo: EnumFoo}}}
const BAR = 'bar';
enum EnumFoo {BAR = 'bar'}
```
