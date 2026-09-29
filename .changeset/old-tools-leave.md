---
"@biomejs/biome": patch
---

Added the nursery rule [`noMisplacedListElements`](https://biomejs.dev/linter/rules/no-misplaced-list-elements/) for HTML and JSX, which requires `<li>` elements with an HTML element parent to be children of `<ul>`, `<ol>`, or `<menu>`. For example, `<div><li>Item</li></div>` is invalid.
