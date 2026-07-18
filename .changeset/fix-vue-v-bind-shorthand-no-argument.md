---
"@biomejs/biome": patch
---

Fixed [#10622](https://github.com/biomejs/biome/issues/10622): the HTML/Vue parser no longer panics on the argument-less `v-bind` shorthand (`:="props"`).

This syntax is valid Vue and equivalent to `v-bind="props"`, so the parser now accepts it (along with the longhand `v-bind:="props"`) instead of crashing while building a diagnostic for a missing argument.
