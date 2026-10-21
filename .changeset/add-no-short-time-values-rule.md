---
"@biomejs/biome": patch
---

Added the nursery rule [`noShortTimeValues`](https://biomejs.dev/linter/rules/no-short-time-values/), which reports CSS animation and transition times that are shorter than the configured `minimum` number of milliseconds. The rule does nothing until `minimum` is set.

For example, the following code is invalid when `minimum` is `100`:

```css
a {
  transition-duration: 80ms;
}
```
