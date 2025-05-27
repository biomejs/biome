---
"@biomejs/biome": minor
---

[useImportExtensions](https://biomejs.dev/linter/rules/use-import-extensions/) now checks imports with sub extensions.
```js
- import 'styles.css'
+ import 'styles.css.ts'
```
