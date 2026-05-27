---
"@biomejs/biome": patch
---

Fixed [#8590](https://github.com/biomejs/biome/issues/8590): [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) and [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) now correctly flag truly-unused imports and `let`/`const` declarations inside Svelte, Vue, and Astro `<script>` blocks (when `html.experimentalFullSupportEnabled` is on). Previously, the script's own top-level bindings were registered into the embedded-binding set, which self-suppressed every diagnostic; precise template-reference tracking now drives the suppression instead. The following Svelte component now correctly reports `unusedImport` and `unusedLet`.

```svelte
<script lang="ts">
  import { unusedImport } from "./other";
  let unusedLet = 42;
  import Button from "./Button.svelte";
  let name = "alice";
</script>
<Button>{name}</Button>
```

[`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) and [`noUnusedFunctionParameters`](https://biomejs.dev/linter/rules/no-unused-function-parameters/) now consult `EmbeddedValueReferences` too, so bindings used only in the template stay quiet. Template-reference extraction was extended to cover the Svelte constructs the parser leaves unattached or opaque, all of which previously produced false positives once the over-suppression was removed:

- Spread attributes — `<input {...props} />` (Svelte and Astro).
- `{expr}` interpolations inside quoted attribute values — `style="top: {top}px"`, `class="card {cls}"`.
- Directive names — `use:action`, `transition:fn`, `in:fn`, `out:fn`, `animate:fn`.
- Shorthand `bind:` — `<Dialog bind:open />` references the local `open`.
- `{#snippet foo(param)}` parameters referenced only in the snippet body.
