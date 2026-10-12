---
"@biomejs/biome": patch
---

Added the nursery rule [`noUselessAssignment`](https://biomejs.dev/linter/rules/no-useless-assignment/), which reports assignments whose value is never read before the variable is reassigned or stops being used.

```js
let id = "x1234"; // this value is never read
id = generateId();
doSomethingWith(id);
```
