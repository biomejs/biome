---
"@biomejs/biome": patch
---

Fixed [#8300](https://github.com/biomejs/biome/issues/8300): [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) now detects JSDoc tags on object properties.

```js
import type LinkOnObjectProperty from "mod";

const testLinkOnObjectProperty = {
	/**
	 * {@link LinkOnObjectProperty}
	 */
	property: 0,
};
```
