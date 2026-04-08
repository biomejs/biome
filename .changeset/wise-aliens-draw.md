---
"@biomejs/biome": patch
---

Fixed [#9690](https://github.com/biomejs/biome/issues/9690): `biome check --write` is now idempotent on HTML files that contain embedded `<style>` or `<script>` blocks. Previously, each run reported "Fixed 1 file" even when the file content did not actually change, because the embedded language formatter's output was not re-indented to match the surrounding HTML block.
