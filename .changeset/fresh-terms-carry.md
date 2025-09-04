---
"@biomejs/biome": patch
---

Range suppressions are now supported for Grit plugins.

For JavaScript, you can suppress a plugin as follows:

```js
// biome-ignore-start lint/plugin/preferObjectSpread: reason
Object.assign({ foo: 'bar'}, baz);
// biome-ignore-end lint/plugin/preferObjectSpread: reason
```

For CSS, you can suppress a plugin as follows:

```css
body {
  /* biome-ignore-start lint/plugin/useLowercaseColors: reason */
  color: #FFF;
  /* biome-ignore-end lint/plugin/useLowercaseColors: reason */
}
```
