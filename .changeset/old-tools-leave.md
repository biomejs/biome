---
"@biomejs/biome": patch
---

Added the nursery rule [`useLiContainer`](https://biomejs.dev/linter/rules/use-li-container/) for HTML and JSX, which requires `<li>` elements with an HTML element parent to be children of `<ul>`, `<ol>`, or `<menu>`. For example, `<div><li>Item</li></div>` is invalid.
