---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUselessDateGetTime`](https://biomejs.dev/linter/rules/no-useless-date-get-time/), which reports dates converted with `.getTime()` before being passed to the `Date` constructor. Passing the date directly, as in `new Date(date)`, already creates a copy.

```js
const copy = new Date(date.getTime());
```
