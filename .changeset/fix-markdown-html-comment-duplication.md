---
"@biomejs/biome": patch
---

Fixed [#13396](https://github.com/biomejs/biome/issues/13396): the Markdown formatter no longer duplicates an HTML comment at the start of an HTML block on every format when the HTML formatter is enabled.

```diff
- <!-- foo --><!-- foo -->*bar*
+ <!-- foo -->*bar*
```
