---
"@biomejs/biome": patch
---

Added the nursery rule [`useConsistentBooleanProps`](https://biomejs.dev/linter/rules/use-consistent-boolean-props/). When enabled, it requires `<Foo bar={true} />` by default. Set `mode` to `"implicit"` to require the shorthand `<Foo bar />`.
