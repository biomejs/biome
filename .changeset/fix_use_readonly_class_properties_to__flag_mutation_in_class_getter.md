---
"@biomejs/biome": patch
---

Fixed [#6634](https://github.com/biomejs/biome/issues/6634): The `useReadonlyClassProperties` rule now correctly flags mutation in class getters.

Example:

```ts
class GetterWithMutationValue {
	#value: string;

	get value() {
		if (!this.#value) {
			this.#value = "defaultValue";
		}

		return this.#value;
	}
}
```
