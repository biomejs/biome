---
"@biomejs/biome": minor
---

The Biome analyzer now supports a new top-level suppression. These suppression have to be placed at the top of the file, and they must be followed by two newlines (`\n\n\`).

The analyzer rules specified inside the block comment will be suppressed for the whole file.

In the example, we suppress the rules `lint/style/useConst` and `lint/suspicious/noDebugger` for the whole file:

```js
// main.js
/**
 * biome-ignore-all lint/style/useConst: i like let
 * biome-ignore-all lint/suspicious/noDebugger: needed now
 */

let path = "/path";
let _tmp = undefined;
debugger
```

In this other example, we suppress `lint/suspicious/noEmptyBlock` for a whole CSS file:

```css
/**
/* biome-ignore-all lint/suspicious/noEmptyBlock: it's fine to have empty blocks
*/

a {}
span {}
```

A new diagnostic is emitted if `biome-ignore-all` suppression isn't placed at the top of the file:


```block
file.js:3:1 suppressions/incorrect ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ! Top level suppressions can only be used at the beginning of the file.

    2 │ let foo = 2;
  > 3 │ /**
      │ ^^^
  > 4 │ * biome-ignore-all lint/style/useConst: reason
  > 5 │ */
      │ ^^
    6 │ let bar = 33;

  i Rename this to biome-ignore

    2 │ let foo = 2;
    3 │ /**
  > 4 │ * biome-ignore-all lint/style/useConst: reason
      │   ^^^^^^^^^^^^^^^^
    5 │ */
    6 │ let bar = 33;


```
