---
"@biomejs/biome": patch
---

Fixed the children of a `<script>` or `<style>` inside an Astro expression being read as JSX. Their contents are text, so braces and comparisons no longer have to be escaped.

```astro
{cond && <style>a { color: red }</style>}
{cond && <script>let x = {a: 1};</script>}
```
