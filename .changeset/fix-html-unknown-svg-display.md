---
"@biomejs/biome": patch
---

Fixed the HTML formatter incorrectly applying SVG block formatting to unknown elements whose names matched SVG element names.

```diff
-<foreignobject>
-  <div>content</div>
-</foreignobject>
+<foreignobject><div>content</div></foreignobject>
```
