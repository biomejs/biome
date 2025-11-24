---
"@biomejs/biome": minor
---

Added the `useButtonType` lint rule for HTML. The rule enforces that the `type` attribute is present and valid on all button elements.

Invalid:

```html
<button>Do something</button>
<button type="incorrectType">Do something</button>
<button type>Do something</button>
<button type />
```

Valid:

```html
<button type="button">Do something</button>
<button type="reset">Do something</button>
<button type="submit" />
```
