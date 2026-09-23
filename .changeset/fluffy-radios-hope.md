---
"@biomejs/biome": patch
---

Fixed [#11841](https://github.com/biomejs/biome/issues/11841), where suppression comments didn't have effect on certain area of HTML-ish languages or snippets inside JavaScript files.

Now the following suppression works as expected:

```vue
<!-- biome-ignore lint/correctness/noUndeclaredVariables: intentionally external -->
<div :title="missingValue"></div>
```
