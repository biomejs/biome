---
"@biomejs/biome": patch
---

Fixed [#6569](https://github.com/biomejs/biome/issues/6569): Allow files to export from themselves with `noImportCycles`.

This means the following is now allowed:

**example.js**
```js
export function example1() {
  return 1;
}

export function example2() {
  return 2;
}

export * as Example from './test';
