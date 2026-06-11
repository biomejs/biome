---
"@biomejs/biome": minor
---

Added the HTML lint rule [`useValidAutocomplete`](https://biomejs.dev/linter/rules/use-valid-autocomplete/), which enforces using valid values for the `autocomplete` attribute on `input` elements.

```html
<input autocomplete="incorrect"/>
```
