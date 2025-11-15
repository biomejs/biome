---
"@biomejs/biome": minor
---

Added support for Glimmer template files (`.gjs` and `.gts`). Biome can now parse, format, and lint Glimmer Component files used in Ember.js applications.

Glimmer templates are recognized using the `<template>...</template>` syntax and can appear in:
- Variable assignments: `const Tpl = <template>...</template>;`
- Class bodies: `class C { <template>...</template> }`
- Expression contexts
- Single unassigned templates (treated as default exports)

**Phase 1 Implementation Notes:**
- Template content is treated as **opaque tokens** - the content is preserved exactly as written without internal parsing or linting
- The template syntax itself is validated (e.g., checking for unclosed tags)
- Templates work with whitespace in the opening tag (e.g., `<template >`, `<template\n>`)
- LSP language IDs "gjs" and "gts" are now recognized
- Future phases will add internal template parsing and linting support

The template content is preserved as-is during formatting, and the parser provides diagnostics for unclosed template tags.
