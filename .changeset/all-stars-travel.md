---
"@biomejs/biome": patch
---
Added the nursery lint rule `useIncludes`.

This rule enforces the use of `.includes()` over `.indexOf()` when checking for the presence of an element. It applies to built-in types like `Array` and `String`, as well as any user-defined object that has both `indexOf` and `includes` methods.

It also flags the use of simple regular expressions with `.test()` in favor of `String.prototype.includes()`.

Using `.includes()` is more readable and modern, as it was introduced in ES2015 (for Strings) and ES2016 (for Arrays) to simplify existence checks.

### Invalid

```js
"foo".indexOf("o") !== -1;
```

```js
["a", "b", "c"].indexOf("a") === -1
```

```js
/a/.test("abc")
```

### Valid

```js
"foo".includes("o");
```

```js
!["a", "b", "c"].includes("a");
```

```js
"abc".includes("a");
```
