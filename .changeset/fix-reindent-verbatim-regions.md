---
"@biomejs/biome": patch
---

Fixed [#10515](https://github.com/biomejs/biome/issues/10515): `biome check --write` was not idempotent on Svelte files — multi-line template literals in `<script>` blocks and block comments in `<style>` blocks gained an extra indent level on every run.
