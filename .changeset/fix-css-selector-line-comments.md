---
"@biomejs/biome": patch
---

Fixed CSS formatter output for selector lists with `allowWrongLineComments` and `//` comments after a selector comma. Biome now keeps the selector before the line comment inline instead of breaking it across descendant combinators.

```diff
-.powerPathNavigator
-  .helm
-  button.pressedButton, // pressed
+.powerPathNavigator .helm button.pressedButton, // pressed
 .powerPathNavigator .helm button:active:not(.disabledButton) {
 }
```
