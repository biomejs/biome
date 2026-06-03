---
name: svelte-script-only-embedding
description: Biome's Svelte handler lints only the <script> block; template expressions are invisible to JS semantic analysis
metadata:
  type: reference
---

`SvelteFileHandler` (crates/biome_service/src/file_handlers/svelte.rs) extracts **only** the `<script>` block via the `SVELTE_FENCE` regex and parses that in isolation as JS. Template expressions — event handlers (`on:click={() => count++}`), `bind:`, mustaches, member mutations on the template side — are NOT part of the embedded JS that the semantic model sees.

**Why it matters:** any JS lint rule reasoning about whether a binding is reassigned/reactive cannot rely on `all_writes`/reference analysis to be complete — a `let` (or a mutated `const` object) may be reassigned from the template, invisibly. eslint-plugin-svelte does not have this limitation because svelte-eslint-parser produces a unified AST.

**How to apply:** for Svelte-domain JS rules, treat only template-independent facts as reliable — e.g. `const` initialized with a literal/function, and `import`s are truly immutable; `let`/`var`/props/`const`-objects must be assumed potentially reactive to avoid false positives. This is exactly how `noSvelteImmutableReactiveStatements` (crates/biome_js_analyze/src/lint/nursery/) was written.

Reactive statements appear as `JsLabeledStatement` with label `$`; gate on `ctx.source_type::<JsFileSource>().as_embedding_kind().is_svelte()`. See also `SemanticFlavor::Svelte` for `$store` handling.
