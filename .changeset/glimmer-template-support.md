---
"@biomejs/biome": minor
---

Added support for Glimmer template files (`.gjs` and `.gts`). Biome can now parse, format, and lint Glimmer Component files used in Ember.js applications.

Glimmer templates are recognized using the `<template>...</template>` syntax and can appear in:
- Variable assignments: `const Tpl = <template>...</template>;`
- Class bodies: `class C { <template>...</template> }`
- Expression contexts
- Single unassigned templates (treated as default exports)

The template content is preserved as-is during formatting, and the parser provides diagnostics for unclosed template tags.
