---
"@biomejs/biome": patch
---

Fixed a false positive in [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/). The rule no longer reports a variable that is only updated with a logical assignment (`??=`, `||=`, or `&&=`), because the operator reads the current value to decide whether to evaluate the right side.

```js
let request = null;
export function load() {
  request ??= fetch("/api");
}
```
