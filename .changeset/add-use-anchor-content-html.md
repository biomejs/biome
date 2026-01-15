---
"@biomejs/biome": minor
---

Added the rule [`useAnchorContent`](https://biomejs.dev/linter/rules/use-anchor-content/) for HTML to enforce that anchor elements have accessible content for screen readers.

#### Examples of invalid code:

```html
<a></a>
<a aria-hidden="true">content</a>
<a><img /></a>
<a><br /></a>
```

The rule flags empty anchors, anchors with only whitespace, content hidden with `aria-hidden`, and void elements that don't provide accessible content (e.g., `<img>` without `alt`, `<br>`, `<hr>`). Anchors with `aria-label` or `title` attributes, or `<img>` elements with non-empty `alt` text, are considered valid.
