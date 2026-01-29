---
"@biomejs/biome": patch
---

Fixed [#8802](https://github.com/biomejs/biome/issues/8802): `useExhaustiveDependencies` now correctly suggests dependencies without including callback-scoped variables or method names.

When accessing object properties with a callback-scoped variable, only the object path is suggested:

```js
// Now correctly suggests `props.value` instead of `props.value[day]`
useMemo(() => {
    return WeekdayValues.filter((day) => props.value[day]);
}, [props.value]);
```

When calling methods on objects, only the object is suggested as a dependency:

```js
// Now correctly suggests `props.data` instead of `props.data.forEach`
useMemo(() => {
    props.data.forEach((item) => console.log(item));
}, [props.data]);
```
