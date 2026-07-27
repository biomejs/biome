---
"@biomejs/biome": patch
---

Fixed the HTML formatter putting a line break between a self-closing element and a sibling that touches it, which renders as a space the source didn't have.

```diff
- <img src="./1.jpg" /><img src="./1.jpg" /><img src="./1.jpg" />
- <img src="./1.jpg" /><img src="./1.jpg" />
+ <img src="./1.jpg" /><img src="./1.jpg" /><img src="./1.jpg" /><img
+   src="./1.jpg"
+ /><img src="./1.jpg" />
```
