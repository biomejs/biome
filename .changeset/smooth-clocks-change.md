---
"@biomejs/biome": minor
---

Added the `noDistractingElements` lint rule for HTML. The rule enforces that no distracting elements are used.

Invalid:

```html
<marquee /> 
<blink />
```

Valid:

```html
<div />
```
