---
"@biomejs/biome": patch
---

[`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) now detects floating Promises whose type comes from a generic type alias.

```ts
type MaybeAsync<T> = Promise<T> | undefined;
declare function getWork(): MaybeAsync<string>;
async function main() {
	getWork();
}
```
