---
"@biomejs/biome": minor
---

Added the new rule [`noImplicitCoercion`](https://biomejs.dev/linter/rules/no-implicit-coercion), which disallows shorthand type conversions in favor of explicit type conversion functions.

**Example (Invalid): Boolean conversion using double negation:**

```js
!!foo;
!!(foo + bar);
```

**Example (Invalid): Number conversion using unary operators:**

```js
+foo;
-(-foo);
foo - 0;
foo * 1;
foo / 1;
```

**Example (Invalid): String conversion using concatenation:**

```js
"" + foo;
foo + "";
`` + foo;
foo += "";
```

**Example (Invalid): Index checking using bitwise NOT:**

```js
~foo.indexOf(1);
~foo.bar.indexOf(2);
```

**Example (Valid): Using explicit type conversion functions:**

```js
Boolean(foo);
Number(foo);
String(foo);
foo.indexOf(1) !== -1;
```
