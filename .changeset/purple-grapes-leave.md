---
"@biomejs/biome": patch
---

Fixed [#9099](https://github.com/biomejs/biome/issues/9099): the HTML formatter collapsing non-text children (inline elements, Svelte expressions, comments) onto a single line when the source had them on separate lines. Biome now preserves the user's intended line breaks for exclusively non-text children.

For example, the following Svelte snippet is now preserved instead of being collapsed to `<div>{name}<!-- comment --></div>`:

```svelte
<div>
  {name}<!-- comment -->
</div>
```

Similarly, HTML elements like `<span>` inside a `<div>` are now preserved when written on their own line:

```html
<div>
  <span>text</span>
</div>
```
