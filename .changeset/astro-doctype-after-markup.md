---
"@biomejs/biome": patch
---

Fixed [#10547](https://github.com/biomejs/biome/issues/10547): Biome now parses a `<!doctype html>` that follows other markup in an Astro file, such as a `<script is:inline>` tag, instead of reporting a parse error.
