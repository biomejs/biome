---
"@biomejs/biome": patch
---

Fixed [#8254](https://github.com/biomejs/biome/issues/8254): The `noParameterAssign` rule with `propertyAssignment: "deny"` was incorrectly reporting an error when a function parameter was used on the right-hand side of an assignment to a local variable's property.

The rule should only flag assignments that modify the parameter binding or its properties (L-value), not the use of its value.

**Valid:**

```js
(input) => {
	const local = { property: 0 };
	local.property = input;
};
```
