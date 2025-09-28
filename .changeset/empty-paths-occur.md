---
"@biomejs/biome": patch
---

Added `style` and `requireForObjectLiteral` options to the lint rule [`useConsistentArrowReturn`](https://biomejs.dev/linter/rules/use-consistent-arrow-return/).

This rule enforces a consistent return style for arrow functions. It can be configured with the following options:

  - `style`: (default: `asNeeded`)
    - `always`: enforces that arrow functions always have a block body.
    - `never`: enforces that arrow functions never have a block body, when possible.
    - `asNeeded`: enforces that arrow functions have a block body only when necessary (e.g. for object literals).

#### `style: "always"`

Invalid:

```js
const f = () => 1;
```

Valid:

```js
const f = () => {
  return 1;
};
```

#### `style: "never"`

Invalid:

```js
const f = () => {
  return 1;
};
```

Valid:

```js
const f = () => 1;
```

#### `style: "asNeeded"`

Invalid:

```js
const f = () => {
  return 1;
};
```

Valid:

```js
const f = () => 1;
```

#### `style: "asNeeded"` and `requireForObjectLiteral: true`

Valid:

```js
const f = () => {
  return { a: 1 }
};
```

