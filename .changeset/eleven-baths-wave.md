---
"@biomejs/biome": minor
---

Added new assist rule [`useSortedAttributes`](https://biomejs.dev/assist/actions/use-sorted-attributes/) for HTML, porting the existing JSX rule. This rule enforces sorted HTML attributes.

**Invalid**

```html
<input type="text" id="name" name="name" />
```
