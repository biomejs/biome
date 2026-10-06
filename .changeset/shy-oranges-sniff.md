---
"@biomejs/biome": patch
---

Improved [`useBetterDomTraversing`](https://biomejs.dev/linter/rules/use-better-dom-traversing/) to offer its fix when the reported expression is followed by a comment, such as `element.childNodes[0] /* first */`. The fix keeps the comment.
