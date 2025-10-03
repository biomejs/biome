---
"@biomejs/biome": patch
---

Fixed [#7628](https://github.com/biomejs/biome/issues/7628): the `noArguments` rule now correctly recognizes the scope of the `arguments` object, as Biome now properly resolves the implicit `arguments` object.

```js
let arguments = 0;
function func() {
    return arguments[0];  // <- now invalid
}
```
