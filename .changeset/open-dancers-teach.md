---
"@biomejs/biome": patch
---

Added the nursery rule [`noHtmlLinkForPages`](https://biomejs.dev/linter/rules/no-html-link-for-pages/) to the Next.js domain.
This rule prevents usage of `<a>` elements to navigate to internal Next.js pages.

The following code is invalid:

```jsx
export const Page = () => {
  return (
    <div>
      <a href='/about'>About</a>
    </div>
  );
}
```
