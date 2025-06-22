---
"@biomejs/biome": patch
---

Fixed an issue where Biome formatter didn't format consistently CSS value separated by commas.

```diff
.font-heading {
- font-feature-settings: var(--heading-salt), var(--heading-ss06),
-   var(--heading-ss11), var(--heading-cv09), var(--heading-liga),
-   var(--heading-calt);

+  font-feature-settings:
+    var(--heading-salt), var(--heading-ss06), var(--heading-ss11),
+    var(--heading-cv09), var(--heading-liga), var(--heading-calt);
}

```
