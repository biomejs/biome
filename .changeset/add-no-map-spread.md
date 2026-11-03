---
"@biomejs/biome": patch
---

Added the nursery rule [`noMapSpread`](https://biomejs.dev/linter/rules/no-map-spread/), which disallows spreading objects or arrays in the values returned by `map()` and `flatMap()` callbacks. Each spread copies a whole object or array for every item, which is slow for large arrays. Its unsafe fix replaces an object that starts with a spread with `Object.assign()`.

```js
const users = getUsers();
const rows = users.map((user) => ({ ...user, role: getRole(user) }));
```
