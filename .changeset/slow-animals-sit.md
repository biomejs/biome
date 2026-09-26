---
"@biomejs/biome": patch
---

Added the nursery rule [`noTailwindRestyledComponents`](https://biomejs.dev/linter/rules/no-tailwind-restyled-components/), which reports Tailwind appearance overrides on JSX and HTML components, such as `<Button className="rounded-none" />`. No appearance categories are allowed by default. Layout utilities without a category, such as `w-full` and `mt-4`, are ignored.
