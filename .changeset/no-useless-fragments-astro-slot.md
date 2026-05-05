---
"@biomejs/biome": patch
---

[`noUselessFragments`](https://biomejs.dev/linter/rules/no-useless-fragments/) no longer flags a `Fragment` carrying a `slot` attribute.

Astro uses the `slot` prop on `<Fragment>` to forward children to a [named slot](https://docs.astro.build/en/basics/astro-components/#named-slots). The Fragment is the carrier for the slot prop and cannot be removed, so the rule must preserve it just as it already preserves `key`-keyed fragments.

```jsx
<Foo>
  <Fragment slot="a">text</Fragment>
  <Fragment slot="b">{value}</Fragment>
</Foo>
```
