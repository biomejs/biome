---
"@biomejs/biome": patch
---

Fixed [#7340](https://github.com/biomejs/biome/issues/7340): The linter now allows the `navigation` property for view-transition in CSS.

Previously, the linter incorrectly flagged `navigation: auto` as an unknown property. This fix adds `navigation` to the list of known CSS properties, following the [CSS View Transitions spec](https://www.w3.org/TR/css-view-transitions-2/#view-transition-navigation-descriptor).

