---
"@biomejs/biome": patch
---

Fixed [#11644](https://github.com/biomejs/biome/issues/11644): [`useHeadingContent`](https://biomejs.dev/linter/rules/use-heading-content/) no longer reports Astro headings that render their text with the `set:html` or `set:text` directive.

```astro
<h1 set:html={heading} />
<h2 set:text={heading}></h2>
```
