---
"@biomejs/biome": patch
---

Fixed [#11605](https://github.com/biomejs/biome/issues/11605): Type inference now infers the type of an unannotated callback parameter from the signature of the function the callback is passed to, and honours explicit type arguments on call expressions. Rules such as [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) can now detect Promises reached through such parameters:

```ts
interface Context {
  doSomething(): Promise<void>;
}

declare function test(callback: (ctx: Context) => Promise<void>): void;

test(async (ctx) => {
	ctx.doSomething(); // now reported as a floating promise
});
```
