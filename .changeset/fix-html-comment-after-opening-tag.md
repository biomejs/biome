---
"@biomejs/biome": patch
---

Fixed the HTML formatter printing a comment twice when it ends the line of an element's opening tag. For example, `<div><!-- comment -->` followed by `<i>x</i></div>` on the next line is now formatted as:

```diff
- <div><!-- comment -->
+ <div>
  	<!-- comment -->
  	<i>x</i>
  </div>
```
