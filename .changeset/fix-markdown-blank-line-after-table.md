---
"@biomejs/biome": patch
---

Fixed [#12663](https://github.com/biomejs/biome/issues/12663): the Markdown formatter no longer removes the blank line between a table and the list that follows it, matching Prettier.

```md
| Table |
| ----- |
| Value |

- Hello
```
