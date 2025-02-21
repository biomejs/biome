---
"@biomejs/biome": minor
---

Introduce a new option `objectWrap` for JavaScript and JSON formatters.
It does the same thing as Prettier's [Object Wrap](https://prettier.io/docs/options#object-wrap) option.

For example, the following code is considered as already formatted when `objectWrap` is `preserve` (default):

```js
const obj = {
  foo: "bar",
};
```

However, when `objectWrap` is `collapse`, it will be formatted to the following output:

```js
const obj = { foo: "bar" };
```

This option is also available in a CLI flag `--javascript-formatter-object-wrap=<preserve|collapse>`.
