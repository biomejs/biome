---
"@biomejs/biome": patch
---

Fixed a Markdown parser hang on block quote lines that contain only spaces after the `>`. For example, Biome no longer hangs when the second line below is `  >` followed by two or more spaces:

```md
- > - a
  >
```
