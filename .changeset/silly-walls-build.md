---
"@biomejs/biome": patch
---

Improved the rules `useConst`, `noUnusedImports`, `useImportTypes` and `noUnusedVariables` inside
Vue, Svelte and Astro files when `experimentalFullSupportEnabled` is set to `true`.

Now variables and components that are imported or defined inside the files won't trigger false positives.

Until now, we suggested disabling these rules with an override. _Now the rules are more stable_; however, you might still experience
a few false positives. Those are probably issues caused by our parser.

**If you use `experimentalFullSupportEnabled`, you can remove the following override:**

```diff
{
-  "overrides": [
-    {
-      "includes": ["**/*.svelte", "**/*.astro", "**/*.vue"],
-      "linter": {
-        "rules": {
-          "style": {
-            "useConst": "off",
-            "useImportType": "off"
-          },
-          "correctness": {
-            "noUnusedVariables": "off",
-            "noUnusedImports": "off"
-          }
-        }
-      }
-    }
-  ]
}
```
