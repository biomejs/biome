---
"@biomejs/biome": patch
---

Fixed the formatter duplicating comments inside an empty object, array, or class that is spread in JSX. Each formatting pass added another copy of the comment.

```diff
  <div
  	{
  		...{
  			/* comment */
- 		}/* comment */
+ 		}
  	}
  />
```
