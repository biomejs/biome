---
"@biomejs/biome": patch
---

Fixed [#12672](https://github.com/biomejs/biome/issues/12672): Astro and Svelte formatting now preserves comments and whitespace inside preformatted elements such as `<pre>` and `<textarea>`.

```diff
-<pre><code>Hello</code></pre>
+<pre>
+  <!-- hello -->
+  <code>Hello</code>
+</pre>
```
