---
"@biomejs/biome": patch
---

Internal: added a parser-based class sort module for [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/) that derives sort keys directly from the `biome_tailwind_parser` CST instead of the hand-written class lexer. The rule itself still uses the existing implementation; the new module is wiring/preparation for a follow-up that swaps the codepaths.
