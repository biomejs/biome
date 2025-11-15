---
"@biomejs/biome": patch
---

Added support for dollar-sign-prefixed filenames in the [`useFilenamingConvention`](https://biomejs.dev/linter/rules/use-filenaming-convention/) rule.

Biome now allows filenames starting with the dollar-sign (e.g. `$postId.tsx`) by default to support naming conventions used by frameworks such as [TanStack Start](https://tanstack.com/start/latest/docs/framework/react/guide/routing#file-based-routing) for file-based-routing.
