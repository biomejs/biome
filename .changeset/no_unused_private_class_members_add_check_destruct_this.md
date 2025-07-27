---
"@biomejs/biome": patch
---

Fixed [#6974](https://github.com/biomejs/biome/issues/6974) and [#6933](https://github.com/biomejs/biome/issues/6933):
`noUnusedPrivateClassMembers` now does checks for prop destructors in class methods.

Example:

```typescript
export class BadExample {
	constructor(private something: string) {}

	example() {
		const { something } = this
		return something
	}
}
```
