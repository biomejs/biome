---
"@biomejs/biome": patch
---

The HTML parser is now able to parse vue directives. This enables us to write/port Vue lint rules that require inspecting the `<template>` section. However, this more complex parsing may result in parsing errors where there was none before. For those of you that have opted in to the experimental support (aka `experimentalFullSupportEnabled`), we greatly appreciate your help testing this out, and your bug reports.
