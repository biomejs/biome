---
"@biomejs/biome": patch
---

The [`noTailwindRawColors`](https://biomejs.dev/linter/rules/no-tailwind-raw-colors/) diagnostic now lists up to three classes to use instead. Each one sets a color from your Tailwind CSS theme that looks similar to the reported palette color. Set [`tailwind.stylesheet`](https://biomejs.dev/reference/configuration/#tailwindstylesheet) to the CSS file that holds your theme to get the list. Biome now scans your project when this rule is enabled, so that it can read that stylesheet.

For example, with `--color-danger: oklch(0.58 0.24 27)` in the theme, the diagnostic for `bg-red-600/50` lists `bg-danger/50`.
