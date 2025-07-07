---
"@biomejs/biome": patch
---

Biome's type inference now detects the type of properties with getters.

**Examples**

```ts
const sneakyObject2 = {
	get something() {
		return new Promise((_, reject) => reject("This is a floating promise!"));
	},
};
// We now detect this is a Promise:
sneakyObject2.something;
```
