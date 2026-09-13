---
"@biomejs/biome": patch
---

Added the nursery rule [`noSvelteAtDebugTags`](https://biomejs.dev/linter/rules/no-svelte-at-debug-tags/), which disallows Svelte's `{@debug}` tag.

```svelte
<!-- Invalid: leftover debugging tag -->
{@debug user}
```

The `{@debug}` tag is a debugging aid and should be removed once you no longer need it, as it should not remain in production code. The rule provides a safe fix that removes the tag.
