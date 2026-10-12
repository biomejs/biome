---
"@biomejs/biome": patch
---

The HTML formatter now formats the JavaScript inside quoted Vue directive values, such as `:prop`, `v-bind`, `v-if`, `@event` handlers that reference a function, and `v-slot`. These values and `v-for` values are written between double quotes, and the strings inside them use single quotes.

```diff
- <div :bar="foo   + 1" v-if='isVisible&&ready'></div>
- <li v-for='item in ["a","b"]'></li>
+ <div :bar="foo + 1" v-if="isVisible && ready"></div>
+ <li v-for="item in ['a', 'b']"></li>
```
