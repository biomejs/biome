---
"@biomejs/biome": patch
---

Fixed a non-idempotency bug in `biome check --write` where multi-line template literals in Svelte `<script>` blocks and block comments in `<style>` blocks would gain extra indentation on every run when an `overrides` entry in `biome.json` matched the file.
