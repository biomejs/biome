---
"@biomejs/biome": patch
---

[`useBetterDomTraversing`](https://biomejs.dev/linter/rules/use-better-dom-traversing/) now offers its fix when the reported expression is followed by a comment, such as `element.childNodes[0] /* first */`. The fix keeps the comment.
