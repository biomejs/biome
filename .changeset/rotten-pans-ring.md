---
"@biomejs/biome": patch
---

Fixed an issue with the HTML formatter where it wouldn't add a space before the `/>` in self closing elements. This brings the HTML formatter more in line with Prettier.

```diff
-<Component/>
+<Component />
```
