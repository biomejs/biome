---
"@biomejs/biome": patch
---

Fixed [#7837](https://github.com/biomejs/biome/issues/7837), where Biome couldn't properly parse text expressions that contained nested curly brackets. This was breaking parsing in Astro and Svelte files.
