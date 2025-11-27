---
"@biomejs/biome": patch
---

Fixed [#7927](https://github.com/biomejs/biome/issues/7927): [`noExtraNonNullAssertion`](https://biomejs.dev/linter/rules/no-extra-non-null-assertion) incorrectly flagged separate non-null assertions on both sides of an assignment.

The rule now correctly distinguishes between nested non-null assertions (still flagged) and separate non-null assertions on different sides of an assignment (allowed).

#### Examples

##### Valid (now allowed)

```ts
arr[0]! ^= arr[1]!;
```

##### Invalid (still flagged)

```ts
arr[0]!! ^= arr[1];
arr[0] ^= arr[1]!!;
```
