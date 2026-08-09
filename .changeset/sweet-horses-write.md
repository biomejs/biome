---
"@biomejs/biome": patch
---

Fixed [#11178](https://github.com/biomejs/biome/issues/11178): [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/) no longer reports Vue's built-in instance properties, such as `$slots` and `$attrs`, in template expressions or `$event` in inline event-handler expressions. The instance properties are still reported inside `<script setup>`, where they are not defined.
