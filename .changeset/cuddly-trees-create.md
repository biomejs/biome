---
"@biomejs/biome": patch
---

Added the [`useImageSize`](https://biomejs.dev/linter/rules/use-image-size) rule to Biome.

The `useImageSize` rule enforces the use of width and height attributes on `<img>` elements for performance reasons. This rule is intended to prevent layout shifts and improve Core Web Vitals by ensuring images have explicit dimensions.

**Invalid:**

```jsx
<img src="/image.png" />
<img src="https://example.com/image.png" />
<img src="/image.png" width="200" />
<img src="/image.png" height="200" />
```

**Valid:**

```jsx
<img width="200" height="600" src="/static/images/portrait-01.webp" />
<img width="100" height="100" src="https://example.com/image.png" />
```

