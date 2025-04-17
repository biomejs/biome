---
"@biomejs/biome": patch
---

Fix [#5682](https://github.com/biomejs/biome/issues/5682): Object patterns with a nested assignment pattern no longer break properties.

For example, the following code:

```js
const { foo: { bar } = { bar: false } } = props;
```

is used to be formatted into:

```js
const {
  foo: { bar } = { bar: false },
} = props;
```

, while Prettier does not expand properties in this case.
