---
"@biomejs/biome": patch
---

Added parsing support for Svelte's new [comments-in-tags](https://github.com/sveltejs/svelte/pull/17671) feature.

The HTML parser will now accept JS style comments in tags in Svelte files.
```svelte
<button
  // single-line comment
  onclick={doTheThing}
>click me</button>

<div
  /* block comment */
  class="foo"
>text</div>
```
