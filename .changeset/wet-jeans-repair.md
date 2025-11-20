---
"@biomejs/biome": patch
---

Fixed [#6675](https://github.com/biomejs/biome/issues/6675): Now only flags
noAccumulatingSpread on Object.assign when a new object is being allocated on
each iteration. Before, all cases using Object.assign with reduce parameters
were warned despite not making new allocations.

The following code will no longer be a false positive:

```js
foo.reduce((acc, bar) => Object.assign(acc, bar), {})
```

The following cases which **do** make new allocations will continue to warn:

```js,expect_diagnostic
foo.reduce((acc, bar) => Object.assign({}, acc, bar), {})
```
