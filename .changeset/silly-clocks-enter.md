---
"@biomejs/biome": patch
---

Fixed [#11182](https://github.com/biomejs/biome/issues/11182): a `biome-ignore` comment now suppresses diagnostics reported on attributes of an HTML element whose start tag spans multiple lines. Previously, the comment only suppressed diagnostics on the tag's first line.

```html
<!-- biome-ignore lint/a11y/noPositiveTabindex: third-party markup -->
<slot
  name="icon"
  tabindex="2"
></slot>
```

Additionally, the "Suppress rule" code action for such elements no longer inserts the suppression comment inside the start tag, which produced invalid HTML.
