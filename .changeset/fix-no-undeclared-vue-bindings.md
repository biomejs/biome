---
"@biomejs/biome": patch
---

Fixed a false positive where `noUndeclaredVariables` reported bindings from Vue `<script setup>` as undeclared when used in `<template>`.

This change ensures embedded bindings collected from script snippets (like imports and `defineModel` results) are respected by the rule.
