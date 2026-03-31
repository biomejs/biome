---
"@biomejs/biome": patch
---

Fixed Svelte <script> tag language detection when the generics attribute contains > characters (e.g., <script lang="ts" generics="T extends Record<string, unknown>">).
