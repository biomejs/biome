---
"@biomejs/biome": patch
---

Fixed the contents of Astro expressions being read as JSX where Astro reads them as text. `<script>`, `<style>` and `is:raw` children are now raw text, HTML comments are recognized, and a bare `>` stays text.

```astro
{cond && <script>if (a > b) {}</script>}
```

Unquoted and template literal attribute values are now accepted in Astro expressions as well.
