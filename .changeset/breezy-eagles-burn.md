---
"@biomejs/biome": patch
---

Fixed [withastro/compiler-rs#221](https://github.com/withastro/compiler-rs/issues/221) so Astro template expressions containing comment-only JSX expression children, such as `<span>{<!-- x -->}</span>`, parse without diagnostics.
