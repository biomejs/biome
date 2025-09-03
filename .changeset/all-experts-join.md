---
"@biomejs/biome": patch
---

Fixed [#6648](https://github.com/biomejs/biome/issues/6648), where Biome's `noUselessFragments` contained inconsistencies with ESLint for fragments only containing text.

Previously, Biome would report that fragments with only text were unnecessary under the `noUselessFragments` rule. Further analysis of ESLint's behavior towards these cases revealed that text-only fragments (`<>A</a>`, `<React.Fragment>B</React.Fragment>`, `<RenamedFragment>B</RenamedFragment>`) would not have `noUselessFragments` emitted for them.

On the Biome side, instances such as these would emit `noUselessFragments`, and applying the suggested fix would turn the text content into a proper JS string.

```js
// Ended up as: - const t = "Text"
const t = <>Text</>

// Ended up as: - const e = t ? "Option A" : "Option B"
const e = t ? <>Option A</> : <>Option B</>

/* Ended up as: 
  function someFunc() {
    return "Content desired to be a multi-line block of text."
  }
*/
function someFunc() {
  return <>
    Content desired to be a multi-line
    block of text.
  <>
}
```

The proposed update was to align Biome's reaction to this rule with ESLint's; the aforementioned examples will now be supported from Biome's perspective, thus valid use of fragments.

```js
// These instances are now valid and won't be called out by noUselessFragments.

const t = <>Text</>
const e = t ? <>Option A</> : <>Option B</>

function someFunc() {
  return <>
    Content desired to be a multi-line
    block of text.
  <>
}
```
