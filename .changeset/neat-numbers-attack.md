---
"@biomejs/biome": patch
---

Fixed [#11605](https://github.com/biomejs/biome/issues/11605): Type inference now infers the type of an unannotated callback parameter from the signature of the function the callback is passed to, and honours explicit type arguments on call expressions. This improves type-aware analysis for [`noBaseToString`](https://biomejs.dev/linter/rules/no-base-to-string/), [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/), [`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/), [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/), [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/), [`noUnsafePlusOperands`](https://biomejs.dev/linter/rules/no-unsafe-plus-operands/), [`noUselessTypeConversion`](https://biomejs.dev/linter/rules/no-useless-type-conversion/), [`useArrayFind`](https://biomejs.dev/linter/rules/use-array-find/), [`useArraySortCompare`](https://biomejs.dev/linter/rules/use-array-sort-compare/), [`useAwaitThenable`](https://biomejs.dev/linter/rules/use-await-thenable/), [`useDisposables`](https://biomejs.dev/linter/rules/use-disposables/), [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/), [`useIncludes`](https://biomejs.dev/linter/rules/use-includes/), [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/), [`useRegexpExec`](https://biomejs.dev/linter/rules/use-regexp-exec/), and [`useStringStartsEndsWith`](https://biomejs.dev/linter/rules/use-string-starts-ends-with/). For example, `noFloatingPromises` can now detect Promises reached through such parameters:

```ts
interface Context {
  doSomething(): Promise<void>;
}

declare function test(callback: (ctx: Context) => Promise<void>): void;

test(async (ctx) => {
	ctx.doSomething(); // now reported as a floating promise
});
```
