---
"@biomejs/biome": minor
---

Added the `useIframeTitle` lint rule for HTML. The rule enforces the usage of the `title` attribute for the `iframe` element.

In `.html` files, this rule matches `iframe` elements case-insensitively (e.g., `<IFRAME>`, `<IFrame>`).

In component-based frameworks (Vue, Svelte, Astro), only lowercase `<iframe>` is checked. PascalCase variants like `<Iframe>` are assumed to be custom components and are ignored.

Invalid:

```html
<iframe></iframe>
<iframe title></iframe>
<iframe title=""></iframe>
<IFRAME></IFRAME>
```

```vue
<iframe></iframe>
<iframe title=""></iframe>
```

```svelte
<iframe></iframe>
<iframe title=""></iframe>
```

```astro
<iframe></iframe>
<iframe title=""></iframe>
```

Valid:

```html
<iframe title="title"></iframe>
<Iframe title="title"></Iframe>
```

```vue
<iframe title="title"></iframe>
<Iframe />
```

```svelte
<iframe title="title"></iframe>
<Iframe />
```

```astro
<iframe title="title"></iframe>
<Iframe />
```
