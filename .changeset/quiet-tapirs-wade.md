---
"@biomejs/biome": patch
---

[`useAwaitThenable`](https://biomejs.dev/linter/rules/use-await-thenable/) no longer reports a false positive when awaiting a thenable typed through a generic type alias.

```ts
type Wrap<T> = T;
async function run(task: Wrap<Promise<void>>) {
	await task;
}
```
