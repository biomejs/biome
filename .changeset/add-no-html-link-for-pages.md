---
"@biomejs/biome": patch
---

Added the nursery rule [`noHtmlLinkForPages`](https://biomejs.dev/linter/rules/no-html-link-for-pages/), a port of `@next/next/no-html-link-for-pages`. It reports `<a>` elements whose `href` points to a page of the Next.js app, which should use `<Link />` from `next/link` instead. It finds the pages in the `app` and `pages` directories of the closest `package.json`, so it needs no configuration in monorepos.

```jsx
// app/about/page.jsx exists
<a href="/about">About</a>
```
