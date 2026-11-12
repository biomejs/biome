---
"@biomejs/biome": patch
---

Fixed [#13088](https://github.com/biomejs/biome/issues/13088): [`useHeadingContent`](https://biomejs.dev/linter/rules/use-heading-content/) no longer reports headings inside Astro template expressions when they have `set:html` or `set:text` directives.
