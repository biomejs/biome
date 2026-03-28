---
"@biomejs/biome": patch
---

Fixed a bug where Vue directive attribute values like `v-bind:class="{'dynamic': true}"` were incorrectly parsed as JavaScript statements instead of expressions. Object literals inside directive values like `:class`, `v-if`, and `v-html` are now correctly parsed as expressions, preventing spurious parse errors.
