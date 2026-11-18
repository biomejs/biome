---
"@biomejs/biome": patch
---

Made [`noArrayForEach`](https://biomejs.dev/linter/rules/no-array-for-each/), [`noArraySortForMinMax`](https://biomejs.dev/linter/rules/no-array-sort-for-min-max/), [`noBooleanSortComparator`](https://biomejs.dev/linter/rules/no-boolean-sort-comparator/), [`noNegatedArrayPredicate`](https://biomejs.dev/linter/rules/no-negated-array-predicate/), [`noReturnArrayPush`](https://biomejs.dev/linter/rules/no-return-array-push/), and [`useIteratorZip`](https://biomejs.dev/linter/rules/use-iterator-zip/) agree on which values are known not to be arrays. They now all ignore calls on literals, object literals, functions, classes, `new` expressions other than `new Array()` or a typed array, and variables whose type annotation, type alias, or `const` value is one of those or a built-in type such as `Map`, `Set`, or `string | undefined`. A type declared in the file with the same name as a built-in type, such as `type Set<T> = T[]`, is no longer treated as the built-in type.

```js
// No longer reported by noArrayForEach
new Collection().forEach((value) => console.log(value));
```
