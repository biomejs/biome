  ---
"@biomejs/biome": patch
---

Improved YAML formatting for bare block scalars at the document root: literal (`|`) and folded (`>`) scalars are now re-indented to the configured `indent_width` and chomping indicators (`+`, `-`, clip) are
honored on trailing blank lines. Explicit indentation indicators (e.g. `>2-`) are preserved so the parsed value does not change.

  ```diff
   >
  -    line 1
  -    line 2
  -    line 3
  +  line 1
  +  line 2
  +  line 3
