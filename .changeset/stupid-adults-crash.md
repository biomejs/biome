---
"@biomejs/biome": minor
---

Add a new assist rule - `useSortedObjectProperties` which enforces ordering of a JS object properties.
This rule will consider spread/calculated keys e.g `[k]: 1` as non-sortable.
Instead, whenever it encounters a non-sortable key, it will sort all the
previous sortable keys up until the nearest non-sortable key, if one exist.
This prevents breaking the override of certain keys using spread keys.

Source: https://perfectionist.dev/rules/sort-objects

```js
// Base
// from
const obj = {
  b: 1,
  a: 1,
  ...g,
  ba: 2,
  ab: 1,
  set aab(v) {
    this._aab = v;
  },
  [getProp()]: 2,
  aba: 2,
  abc: 3,
  abb: 3,
  get aaa() {
    return "";
  },
};
// to
const obj = {
  a: 1,
  b: 1,
  ...g,
  set aab(v) {
    this._aab = v;
  },
  ab: 1,
  ba: 2,
  [getProp()]: 2,
  get aaa() {
    return "";
  },
  aba: 2,
  abb: 3,
  abc: 3,
};
// partionByNewLine: true
// from
const obj = {
  b: 1,
  a: 1,

  ba: 2,
  ab: 1,

  aba: 2,
  abc: 3,
  abb: 3,
};
// to
const obj = {
  a: 1,
  b: 1,

  ab: 1,
  ba: 2,

  aba: 2,
  abb: 3,
  abc: 3,
};
// partionByComment: true
// from
const obj = {
  b: 1,
  a: 1,
  // Partion 1
  ba: 2,
  ab: 1,
  // Partion 2
  aba: 2,
  abc: 3,
  abb: 3,
};
// to
const obj = {
  a: 1,
  b: 1,
  // Partion 1
  ab: 1,
  ba: 2,
  // Partion 2
  aba: 2,
  abb: 3,
  abc: 3,
};
```
