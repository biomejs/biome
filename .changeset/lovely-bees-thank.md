---
"@biomejs/biome": patch
---

Fixed [#10330](https://github.com/biomejs/biome/issues/10330): Vue interpolation delimiters now stay attached to whitespace-sensitive element boundaries and adjacent inline siblings, wrapping their expression when needed to fit the configured line width. Interpolations followed by text now also converge after one formatting pass.

```diff
-<v-btn v-if="store.state.user" variant="text" to="/my-rooms"
-  >{{ $t("nav.my-rooms") }}</v-btn
->
+<v-btn v-if="store.state.user" variant="text" to="/my-rooms">{{
+  $t("nav.my-rooms")
+}}</v-btn>
```
