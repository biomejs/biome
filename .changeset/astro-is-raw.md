---
"@biomejs/biome": patch
---

Fixed the children of an Astro element carrying `is:raw` being parsed as markup instead of raw text. This now also covers `<script>` and `<style>`, whose contents Astro emits verbatim rather than processing, so they are no longer linted as JavaScript or CSS.

```astro
<article is:raw><% awesome %></article>
<script is:raw>{{ mustache }}</script>
```
