---
"@biomejs/biome": minor
---

Added the HTML version of the [`useHeadingContent`](https://biomejs.dev/linter/rules/use-heading-content/) rule. The rule now enforces that heading elements (`h1`-`h6`) have content accessible to screen readers in HTML, Vue, Svelte, and Astro files.

```html
<!-- Invalid: empty heading -->
<h1></h1>

<!-- Invalid: heading hidden from screen readers -->
<h1 aria-hidden="true">invisible content</h1>

<!-- Valid: heading with text content -->
<h1>heading</h1>

<!-- Valid: heading with accessible name -->
<h1 aria-label="Screen reader content"></h1>
```
