---
"@biomejs/biome": patch
---

Fixed [#8148](https://github.com/biomejs/biome/issues/8148). [`noInvalidUseBeforeDeclaration`](https://biomejs.dev/linter/rules/no-invalid-use-before-declaration/) no longer reports some valid use before declarations.

The following code is no longer reported as invalid:

```ts
class classA {
	C = C;
}
const C = 0;
```
