---
"@biomejs/biome": patch
---

Added the nursery rule [`noHtmlLinkForPages`](https://biomejs.dev/linter/rules/no-html-link-for-pages/), a port of `@next/next/no-html-link-for-pages`. It reports `<a>` elements whose `href` points to a page of the Next.js app, which should use `<Link />` from `next/link` instead.

```jsx
// app/about/page.jsx exists
<a href="/about">About</a>
```
