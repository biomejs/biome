---
"@biomejs/biome": minor
---

Added the `useIframeTitle` lint rule for HTML. The rule enforces the usage of the `title` attribute for the `iframe` element.

Invalid:

```html
<iframe></iframe>
<iframe title=""></iframe>
```

Valid:

```html
<iframe title="title"></iframe>
```
