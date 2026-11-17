---
"@biomejs/biome": patch
---

Fixed the CSS and SCSS formatter indenting a selector one level too deep when a comment comes before it in the selector list of a nested rule. The selector now lines up with its comment.

```diff
 .a {
   & > .b,
   /* comment */
-    & > .c {
+  & > .c {
     display: none;
   }
 }
```
