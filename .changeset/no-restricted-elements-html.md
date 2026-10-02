---
"@biomejs/biome": minor
---

The rule [`noRestrictedElements`](https://biomejs.dev/linter/rules/no-restricted-elements/) now supports HTML, Vue, Svelte, and Astro files. For example, with `{ "elements": { "marquee": "Use CSS animations instead." } }`, the following code is now reported:

```html
<marquee>Breaking news</marquee>
```
