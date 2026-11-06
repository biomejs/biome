---
"@biomejs/biome": patch
---

Added the nursery rule [`useNextScriptForGoogleAnalytics`](https://biomejs.dev/linter/rules/use-next-script-for-google-analytics/), which reports `<script>` elements that load Google Analytics or Google Tag Manager. Use the `GoogleAnalytics` and `GoogleTagManager` components from `@next/third-parties/google` instead, which load these scripts after the page becomes interactive.

```jsx
<script async src="https://www.googletagmanager.com/gtag/js?id=G-XYZ" />
```

The rule belongs to the `next` domain.
