---
"@biomejs/biome": patch
---

Fixed [#6796](https://github.com/biomejs/biome/issues/6796): Fixed a false positive that happened in `noFloatingPromises` when calling functions that were declared as part of `for ... of` syntax inside `async` functions.

Instead, the variables declared inside `for ... of` loops are now correctly
inferred if the expression being iterated evaluates to an `Array` (support for other iterables will follow later).

**Invalid example**

```tsx
const txStatements: Array<(tx) => Promise<any>> = [];

db.transaction((tx: any) => {
    for (const stmt of txStatements) {
        // We correctly flag this resolves to a `Promise`:
        stmt(tx)
    }
});
```

**Valid example**

```tsx
async function valid(db) {
    const txStatements: Array<(tx: any) => void> = [(tx) => tx.insert().run()];

    db.transaction((tx: any) => {
        for (const stmt of txStatements) {
            // We don't flag a false positive here anymore:
            stmt(tx)
        }
    });
}
```
