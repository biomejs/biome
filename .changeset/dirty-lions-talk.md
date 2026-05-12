---
"@biomejs/biome": minor
---

The Biome Language server now supports the "go-to definition" feature.

When the cursor of the mouse is hovering an entity (variable, CSS class, type, etc.), and the command <kbd>CTRL</kbd> + click is triggered, the editor jumps to where this entity is defined, if the language server can find it.

Here's what Biome is able to resolve:
- Variables and types used in JavaScript modules, defined in the same file or imported from another module.
- JSX Components used in JavaScript modules, defined in the same file or imported from another module.
- CSS classes used in JSX and HTML-ish files (Vue, Svelte and Astro), and defined in CSS files.
- Components used in HTML-ish files and defined in other HTML-ish.
- Variables used in HTML-ish files and defined in the same file or imported from another module (JavaScript or HTML-ish).
