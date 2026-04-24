---
"@biomejs/biome": patch
---

Fixed [#9450](https://github.com/biomejs/biome/issues/9450): the HTML formatter now correctly preserves multiline formatting for nested `<template>` elements (e.g. `<template #body>`) when the source has children on separate lines. Previously, the children were collapsed onto a single line.

```diff
 <template>
   <UModal>
-    <template #body> <p>content</p> </template>
+    <template #body>
+      <p>content</p>
+    </template>
   </UModal>
 </template>
```
