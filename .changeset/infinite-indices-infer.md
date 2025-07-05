---
"@biomejs/biome": patch
---

Type inference now recognises _index signatures_ and their accesses when they
are being indexed as a string.

## Example

```ts
type BagOfPromises = {
    // This is an index signature definition. It declares that instances of type
    // `BagOfPromises` can be indexed using arbitrary strings.
    [property: string]: Promise<void>;
};

let bag: BagOfPromises = {};
// Because `bag.iAmAPromise` is equivalent to `bag["iAmAPromise"]`, this is
// considered an access to the string index, and a Promise is expected.
bag.iAmAPromise;
```
