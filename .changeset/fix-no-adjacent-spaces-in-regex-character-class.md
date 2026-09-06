---
"@biomejs/biome": patch
---

Fixed [#11419](https://github.com/biomejs/biome/issues/11419): [`noAdjacentSpacesInRegex`](https://biomejs.dev/linter/rules/no-adjacent-spaces-in-regex/) no longer applies its fix inside a regex character class (`[...]`), where it was changing what the class matches instead of just clarifying the spacing.

For example, Biome used to "fix" this regex into matching different characters:

```js
const before = /[\s   ]/;
// Biome used to rewrite this to /[\s {3}]/, which is a different regex:
// {3} isn't a quantifier inside a character class, it's three more
// characters to match — the fixed regex now also matches "1", "2", "{", "}".
```

Adjacent spaces outside a character class are still merged into a quantifier as before.
