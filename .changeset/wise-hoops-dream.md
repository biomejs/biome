---
"@biomejs/biome": patch
---

Added the new nursery rule [`noEmptyDocumentation`](https://biomejs.dev/linter/rules/no-empty-documentation), which disallows empty documentation (comments & descriptions) in HTML, CSS, JavaScript, JSONC & GraphQL.

```html
<!-- -->
<input/>
```

```css
/* */
.invalid {}
```

```js
/* */
let invalid = 1;
```

```jsonc
{
  /* */
  "name": "John Doe"
}
```

```graphql
" "
query {}
```
