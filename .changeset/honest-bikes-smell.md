---
"@biomejs/biome": minor
---

Added the new rule [useSingleJsDocAsterisk](https://biomejs.dev/linter/rules/use-single-js-doc-asterisk/) which enforces JSDoc comment lines to start with a single asterisk.
```js
// Invalid
/**
 ** Description
 */

// Valid
/**
 * Description
 */
```
