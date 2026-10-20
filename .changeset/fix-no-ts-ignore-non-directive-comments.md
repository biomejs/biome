---
"@biomejs/biome": patch
---

Fixed [#12445](https://github.com/biomejs/biome/issues/12445): [`noTsIgnore`](https://biomejs.dev/linter/rules/no-ts-ignore/) no longer reports comments that merely mention `@ts-ignore`, such as `// see the note on @ts-ignore below` or JSDoc prose. The rule now follows TypeScript and only reports `@ts-ignore` at the start of a comment, or at the start of the last line of a block comment. The fix now rewrites only the directive.
