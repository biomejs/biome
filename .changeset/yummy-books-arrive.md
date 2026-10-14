---
"@biomejs/biome": patch
---

Fixed [#12320](https://github.com/biomejs/biome/issues/12320): [`useReactCompiler`](https://biomejs.dev/linter/rules/use-react-compiler/) now analyzes files that contain an exported type alias, interface, or enum declaration, such as `export type Props = { id: string };`. Previously, the rule silently skipped these files.
