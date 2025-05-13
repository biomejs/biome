---
"@biomejs/biome": minor
---

Added the new rule [`useAdjacentGetterSetter`](https://biomejs.dev/linter/rules/use-adjacent-getter-setter), which enforces getters and setters for the same property
to be adjacent in class and object definitions.

Option `order` can be used to specify the expected ordering of getters and setters:

 - `"anyOrder"` (default): Accessors for the same property must be adjacent, but can be in any order
 - `"getBeforeSet"`: Getter must come before setter
 - `"setBeforeGet"`: Setter must come before getter
