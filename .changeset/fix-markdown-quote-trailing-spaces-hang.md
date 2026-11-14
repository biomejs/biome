---
"@biomejs/biome": patch
---

Fixed a Markdown parser hang on block quote lines that contain only spaces after the `>`. For example, Biome no longer hangs when the second line below has two or more spaces after the `>`:

```md
- > - a
  >
```
