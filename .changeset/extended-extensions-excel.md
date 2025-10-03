---
"@biomejs/biome": patch
---

Fixed [#7638](https://github.com/biomejs/biome/issues/7638): [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) no longer emits diagnostics on valid import paths that end with a query or hash.

#### Example

```js
// This no longer warns if `index.css` exists:
import style from '../theme/index.css?inline';
```
