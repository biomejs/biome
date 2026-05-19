---
"@biomejs/biome": patch
---

Fixed HTML formatting for a case where comments could cause the formatter to split up a closing tag, which would cause the resulting HTML to be syntactically invalid.

Input:
```html
<span><!-- 1
--><span>a</span><!-- 2
--><span>b</span><!-- 3
--></span>
```

Output:
```diff
  <span
	  ><!-- 1
- --> <span>a</span<!-- 2
- --> ><span>b</span><!-- 3
+ --><span>a</span><!-- 2
+ --><span>b</span><!-- 3
  --></span
  >
```
