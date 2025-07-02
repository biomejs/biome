---
"@biomejs/biome": patch
---

Fixed [#6634](https://github.com/biomejs/biome/issues/6634): The `useReadonlyClassProperties` rule now correctly flags mutations in class getters and in arrow functions within class properties.

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

```ts
class ClassPropertyArrowFunctionWithMutation {
  private bar: string | null = null;

  readonly action = () => {
    this.bar = 'init';
  };
}
```
