---
"@biomejs/biome": patch
---

Fixed the HTML formatter printing a comment twice when it ended the line of the last element in a document:

```diff
- text<!-- a --><!-- a -->
+ text<!-- a -->
```
