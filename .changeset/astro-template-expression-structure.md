---
"@biomejs/biome": patch
---

Fixed [#9944](https://github.com/biomejs/biome/issues/9944): adjacent elements inside an Astro expression now parse as an implicit fragment instead of raising an error.

```astro
{options.map(() =>
  <div />
  <div />
)}
```

Unclosed HTML void elements such as `<br>` and comment-only expressions are now accepted in Astro templates too.
