---
"@biomejs/biome": patch
---

Added support for negative value utilities in [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/). The rule now correctly detects and sorts negative value utilities like `-ml-2`, `-mr-4`, `-top-2`, etc. This also allows custom utilities defined with a leading `-` prefix to be matched when the target is specified with a `-` prefix (e.g., `-test$` matches `-test`).
