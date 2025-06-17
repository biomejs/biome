---
"@biomejs/biome": minor
---

Added a format option `expand` for Javascript and JSON formatters.
The option allows to enforce the formatting of arrays and objects on multiple lines, regardless of their length.
It has three options:

When set to `auto` (default), objects are expanded if the first property has a leading newline.
Arrays are collapsed when they fit to a single line.
For example, both styles below are considered as already formatted:

```js
const obj = {
  foo: "bar",
};
```

```js
const obj = { foo: "bar" };
```

When set to `always`, objects and arrays are always expanded.

When set to `never`, objects and arrays are never expanded when they fit in a single line.
It is equivalent to Prettier's [Object Wrap](https://prettier.io/docs/options#object-wrap) option with `collapse`.
