---
"@biomejs/biome": patch
---

Svelte attributes now use the shorthand wherever `prettier-plugin-svelte` does. An attribute whose value is nothing but the expression `{name}` loses its quotes, and one whose expression is just the attribute's own name collapses:

```diff
- <div style:background="{color}"></div>
+ <div style:background={color}></div>

- <input value="{value}" class:active={active} style:color={color} />
+ <input {value} class:active style:color />

- <Widget let:foo={foo}>
+ <Widget let:foo>
```

This already worked for a plain attribute and for `bind:`, but only when the author had left the quotes off. It now also covers `class:`, `style:` and `let:`, and no longer depends on the quoting.
