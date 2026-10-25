---
"@biomejs/biome": patch
---

Fixed [#12679](https://github.com/biomejs/biome/issues/12679): when `markdown.parser.frontmatter` and `markdown.formatter.formatEmbeds` are enabled, the Markdown formatter keeps the frontmatter of a fenced Markdown code block as written. Previously, it formatted the frontmatter as a thematic break and a paragraph, which removed the indentation of its YAML.

````md
```markdown
---
verified:
  by: human
---
# Policy
```
````
