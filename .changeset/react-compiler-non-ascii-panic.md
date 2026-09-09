---
"@biomejs/biome": patch
---

Fixed [#11678](https://github.com/biomejs/biome/issues/11678): [`useReactCompiler`](https://biomejs.dev/linter/rules/use-react-compiler/) no longer panics on files that contain non-ASCII characters. The panic dropped the whole file from the run, so every diagnostic in it was silently lost. This bumps the vendored React Compiler crates to pick up the upstream fix from [react/react#37539](https://github.com/react/react/pull/37539).
