---
"@biomejs/biome": patch
---

Fixed [#8179](https://github.com/biomejs/biome/issues/8179): The [`useConsistentArrowReturn`](https://biomejs.dev/linter/rules/use-consistent-arrow-return/) rule now correctly handles multiline expressions in its autofix when the `style` option is set to `"always"`.

Previously, when converting arrow functions with multiline expressions to block statements, the autofix would place a newline after the `return` keyword. This triggered JavaScript's Automatic Semicolon Insertion (ASI), causing the function to return `undefined` instead of the intended value.

For example, this code:

```js
const foo = (l) =>
  l
    .split('\n')
```

Was incorrectly fixed to:

```js
const foo = (l) => {
  return
  l.split('\n');
}
```

Biome now correctly produces:

```js
const foo = (l) => {
  return l.split('\n');
}
