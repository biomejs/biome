---
"@biomejs/biome": minor
---

Added the `useHtmlLang` lint rule for HTML. The rule enforces that the `html` element has a `lang` attribute.

Invalid:

```html
<html></html>
<html lang></html>
<html lang=""></html>
```

Valid:

```html
<html lang="en"></html>
```
