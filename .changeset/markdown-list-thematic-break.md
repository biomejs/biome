---
"@biomejs/biome": patch
---

Fixed the Markdown formatter turning a list item that starts with a thematic break into a standalone thematic break. The thematic break now uses a character that differs from the list marker.

```diff
- - ---
+ - ***
```

A list whose item starts with a thematic break now keeps the usual list marker instead of switching to `+`. For example, `+ ---` is now formatted as `- ***`.
