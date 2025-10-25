---
"@biomejs/biome": patch
---

Fixed [#7864](https://github.com/biomejs/biome/issues/7864): Biome now preserves component tag name casing in Svelte, Astro, and Vue files.

Previously, the HTML formatter incorrectly lowercased component names like `<Button>` to `<button>` when those names matched HTML element tag names. This broke imports from UI component libraries.

Now, tag casing is preserved in component framework files while maintaining the lowercasing behavior for pure HTML files:

**Svelte/Astro/Vue files:**
```svelte
<Button label="test" />  <!-- Preserved -->
<TextInput />            <!-- Preserved -->
<button>native</button>  <!-- Preserved -->
```

**HTML files (unchanged behavior):**
```html
<BUTTON>text</BUTTON>  <!-- Lowercased to <button> -->
```
