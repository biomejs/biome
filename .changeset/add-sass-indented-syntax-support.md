---
"@biomejs/biome": minor
---

Added initial support for the indented Sass syntax in `.sass` files. Biome now parses and formats style rules, declarations, nesting, variables, and the common `@mixin`, `@include`, `@if`, `@else`, and `@media` at-rules, which use indentation for blocks and line breaks instead of semicolons.

The indented syntax is not fully supported yet. Multiline statements and the `@keyframes`, `@page`, and `@font-feature-values` blocks are not handled.
