---
"@biomejs/biome": patch
---

Fixed [`noInvalidUseBeforeDeclaration`](https://biomejs.dev/linter/rules/no-invalid-use-before-declaration/).
The rule now reports invalid use of classes, enums, and TypeScript's import-equals before their declarations.

The following code is now reported as invalid:

```js
new C();
class C {}
```
