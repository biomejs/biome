---
"@biomejs/biome": patch
---

Fixed [#8590](https://github.com/biomejs/biome/issues/8590): [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) and [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) now correctly flag truly-unused imports and `let`/`const` declarations inside Svelte, Vue, and Astro `<script>` blocks (when `html.experimentalFullSupportEnabled` is on). Previously, the script's own top-level bindings were registered into the embedded-binding set, which self-suppressed every diagnostic; precise template-reference tracking now drives the suppression instead. The following Svelte component now correctly reports `unusedImport` and `unusedLet`:

```svelte
<script lang="ts">
  import { unusedImport } from "./other";
  let unusedLet = 42;
  import Button from "./Button.svelte";
  let name = "alice";
</script>
<Button>{name}</Button>
```

[`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) also now consults `EmbeddedValueReferences` so imports used only as Svelte component tags (`<Button />`) or inside template expressions stay quiet. As part of the same fix, spread attributes (`<input {...props} />` in Svelte and Astro) now count as a template reference to `props`.
