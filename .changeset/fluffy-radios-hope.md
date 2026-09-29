---
"@biomejs/biome": patch
---

Fixed [#11841](https://github.com/biomejs/biome/issues/11841), where suppression comments had no effect on some parts of HTML-ish files and on snippets embedded in JavaScript files.

Now the following suppression works as expected:

```vue
<!-- biome-ignore lint/correctness/noUndeclaredVariables: intentionally external -->
<div :title="missingValue"></div>
```
