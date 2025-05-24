---
"@biomejs/biome": minor
---

Added the new rule [`useObjectSpread`](https://biomejs.dev/linter/rules/use-object-spread), which prefers object spread syntax over `Object.assign()` when constructing new objects.

**Example (Invalid): Using Object.assign with an empty object:**

```js
Object.assign({}, foo);
Object.assign({}, { foo: 'bar' });
```

**Example (Invalid): Using Object.assign with object literal as first argument:**

```js
Object.assign({ foo: 'bar' }, baz);
Object.assign({}, baz, { foo: 'bar' });
```

**Example (Valid): Using object spread syntax:**

```js
({ ...foo });
({ ...baz, foo: 'bar' });
```

**Example (Valid): Modifying existing objects is allowed:**

```js
Object.assign(foo, { bar: baz });
Object.assign(foo, bar, baz);
```
