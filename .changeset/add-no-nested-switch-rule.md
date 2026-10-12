---
"@biomejs/biome": patch
---

Added the new nursery rule [`noNestedSwitch`](https://biomejs.dev/linter/rules/no-nested-switch/), which disallows `switch` statements inside other `switch` statements.

```js
switch (n) {
  case 0:
    switch (m) {} // reported
}
```
