---
"@biomejs/biome": patch
---

Fixed [`useLayeredStyles`](https://biomejs.dev/linter/rules/use-layered-styles/) reporting style rules in SCSS mixin bodies, which take their cascade layer from where the mixin is included. The following code is now valid:

```scss
@mixin card {
  .card {
    color: red;
  }
}

@layer base {
  @include card;
}
```

Instead, the rule reports an `@include` outside a cascade layer when the mixin emits a style rule outside a cascade layer, directly or through the mixins it includes. Only mixins from the same file are checked.
