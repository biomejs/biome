## Overall Metrics

**Average compatibility**: 94.65

<details>
    <summary>Definition</summary>

    $$average = \frac\{\sum_{file}^\{files}compatibility_\{file}}\{files}$$
</details>

**Compatible lines**: 95.67

<details>
    <summary>Definition</summary>

    $$average = \frac{\sum_{file}^{files}matching\_lines_{file}}{max(lines_{rome}, lines_{prettier})}$$
</details>

[Metric definition discussion](https://github.com/rome/tools/issues/2555#issuecomment-1124787893)

## Test cases

### js/arrays/numbers-with-holes.js
```diff
 const numberWithHoles1 = [
   7234932941,
   7234932722,
   7234932312,
   ,
   // comment before a hole 1
   7234932841,
   ,
   7234932843,
   ,
   // comment after a hole 1
   7234932436,
 ];
 
 const numberWithHoles2 = [
   0x234932941,
   0x234932722,
   0x234932312,
-
   ,
   // comment before a hole 2
   0x234932841,
   ,
   0x234932843,
   ,
+
   // comment after a hole 2
   0x234932436,
 ];

```

**Prettier Similarity**: 96.43%


### js/arrows/comment.js
```diff
 /**
  * Curried function that ends with a BEM CSS Selector
  *
  * @param {String} block - the BEM Block you'd like to select.
  * @returns {Function}
  */
 export const bem =
   (block) =>
   /**
    * @param {String} [element] - the BEM Element within that block; if undefined, selects the block itself.
    * @returns {Function}
    */
   (element) =>
   /**
    * @param {?String} [modifier] - the BEM Modifier for the Block or Element; if undefined, selects the Block or Element unmodified.
    * @returns {String}
    */
   (modifier) =>
     [
       ".",
       css(block),
       element ? `__${css(element)}` : "",
       modifier ? `--${css(modifier)}` : "",
     ].join("");
 
 <FlatList
   renderItem={(
     info, // $FlowExpectedError - bad widgetCount type 6, should be Object
   ) => <span>{info.item.widget.missingProp}</span>}
   data={data}
 />;
 
 func(
   () =>
     // comment
     a,
 );
 func(
   () => () =>
     // comment
     a,
 );
 func(
   () => () => () =>
     // comment
     a,
 );
 
 func(() =>
   // comment
   a ? b : c,
 );
 func(
   () => () =>
     // comment
     a ? b : c,
 );
 func(
   () => () => () =>
     // comment
     a ? b : c,
 );
 
 func(
   () =>
-    // comment
-    (a, b, c),
+    (
+      // comment
+      a, b, c
+    ),
 );
 func(
   () => () =>
-    // comment
-    (a, b, c),
+    (
+      // comment
+      a, b, c
+    ),
 );
 func(
   () => () => () =>
-    // comment
-    (a, b, c),
+    (
+      // comment
+      a, b, c
+    ),
 );

```

**Prettier Similarity**: 85.71%


### js/arrows/currying-4.js
```diff
 Y(() => (a ? b : c));
 
 Y(() => () => (a ? b : c));
 
 Y(() => () => () => (a ? b : c));
 
 Y(() =>
   longlonglonglonglonglonglonglonglonglongCondition
     ? "Prettier is an opinionated code formatter."
     : "Prettier takes your code and reprints it from scratch by taking the line length into account.",
 );
 
 Y(
   () => () =>
     longlonglonglonglonglonglonglonglonglongCondition
       ? "Prettier is an opinionated code formatter."
       : "Prettier takes your code and reprints it from scratch by taking the line length into account.",
 );
 
 Y(
   () => () => () =>
     longlonglonglonglonglonglonglonglonglongCondition
       ? "Prettier is an opinionated code formatter."
       : "Prettier takes your code and reprints it from scratch by taking the line length into account.",
 );
 
 const x1 = () => [
   "The",
   "green",
   "dragon",
   "liked",
   "to",
   "knit",
   "sweaters",
   "for",
   "the",
   "fluffy",
   "clouds",
   "in",
   "the",
   "sky.",
 ];
 
 const x2 = () => () => [
   "The",
   "green",
   "dragon",
   "liked",
   "to",
   "knit",
   "sweaters",
   "for",
   "the",
   "fluffy",
   "clouds",
   "in",
   "the",
   "sky.",
 ];
 
 const x3 = () => () => () => [
   "The",
   "green",
   "dragon",
   "liked",
   "to",
   "knit",
   "sweaters",
   "for",
   "the",
   "fluffy",
   "clouds",
   "in",
   "the",
   "sky.",
 ];
 
 f((a) => (1, 2, 3) /* a */);
 f((a) => (b) => (1, 2, 3) /* b */ /* a */);
-f((a) => (b) => (c) => (1, 2, 3) /* c */ /* b */ /* a */);
+f((a) => (b) => (c) => (1, 2, 3) /* b */ /* c */ /* a */);
 
 f((a) => (1 ? 2 : 3) /* a */);
 f((a) => (b) => (1 ? 2 : 3) /* b */ /* a */);
-f((a) => (b) => (c) => (1 ? 2 : 3) /* c */ /* b */ /* a */);
+f((a) => (b) => (c) => (1 ? 2 : 3) /* b */ /* c */ /* a */);
 
 a(
   "",
   "",
   ({}) =>
     () =>
     () =>
     () =>
     () =>
     () =>
     () =>
       test,
 );
 a(
   "",
   "",
   ({}) =>
     () =>
     () =>
     () =>
     () =>
     () =>
     () =>
       test ? 1 : 2,
 );

```

**Prettier Similarity**: 98.17%


### js/assignment-expression/property-key.js
```diff
 a = {
-  [(this.resource = resource)]: 1,
+  [this.resource = resource]: 1,
 };
 
 class A {
-  [(this.resource = resource)] = 1;
+  [this.resource = resource] = 1;
 
-  [(this.resource = resource)]() {}
+  [this.resource = resource]() {}
 }

```

**Prettier Similarity**: 66.67%


### js/binary-expressions/comment.js
```diff
 a =
   // Comment 1
   Math.random() * (yRange * (1 - minVerticalFraction)) +
   minVerticalFraction * yRange -
   offset;
 
 a +
   a +
   a + // comment
   a +
   a;
 
 a &&
   longLongLongLongLongLongLongLongLong &&
   longLongLongLongLongLongLongLongLong && // comment
   longLongLongLongLongLongLongLongLong &&
   longLongLongLongLongLongLongLongLong;
 
 a ||
   longLongLongLongLongLongLongLongLong ||
   longLongLongLongLongLongLongLongLong || // comment
   longLongLongLongLongLongLongLongLong ||
   longLongLongLongLongLongLongLongLong;
 
 var a = x(
   abifornCringerMoshedPerplexSawder +
     kochabCooieGameOnOboleUnweave + // f
     glimseGlyphsHazardNoopsTieTie +
     bifornCringerMoshedPerplexSawder,
 );
 
 foo[
   a +
     a + // comment
     a +
     bar[
       b +
         b +
         b + // comment
         b +
         b
     ]
 ];
 
 !(
   a +
   a + // comment
   a +
   !(
     b +
     b +
     b + // comment
     b +
     b
   )
 );
 
 const logical_expression =
   aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa /* trailing comment */ &&
   /* leading comment */ bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb &&
   // line comment
   cccccccccccccccccccccccc;
 
 const bc = a /* internal before */ || /* internal after */ b;
 
 a +
   /**/
-  a.a().a();
+  a
+    .a()
+    .a();

```

**Prettier Similarity**: 95.71%


### js/binary-expressions/mutiple-comments/17192.js
```diff
-ErrorLike =
-  SerializedProps &
-  // cause is a new addition to Error that is not yet available in all runtimes. We have added
-  // it to try and pinpoint additional reasoning for failures such as Node's fetch.
-  // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause
-  { cause: unknown };
+// FIXME
+// TODO: reformat issue
+// ErrorLike =
+//   SerializedProps &
+//   // cause is a new addition to Error that is not yet available in all runtimes. We have added
+//   // it to try and pinpoint additional reasoning for failures such as Node's fetch.
+//   // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause
+//   { cause: unknown };
 
-ErrorLike =
-  SerializedProps & // cause is a new addition to Error that is not yet available in all runtimes. We have added
-  // it to try and pinpoint additional reasoning for failures such as Node's fetch.
-  // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause
-  { cause: unknown };
+// ErrorLike =
+//   SerializedProps & // cause is a new addition to Error that is not yet available in all runtimes. We have added
+//   // it to try and pinpoint additional reasoning for failures such as Node's fetch.
+//   // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause
+//   { cause: unknown };

```

**Prettier Similarity**: 7.14%


### js/binary-expressions/parentheses/bitwise.js
```diff
 // https://github.com/prettier/prettier/issues/18145
-1 << bit % 8;
+1 << (bit % 8);
 1 >> (bit - 8);
-1 | bit % 8;
-1 & bit % 8;
-1 ^ bit % 8;
-1 >>> bit % 8;
+1 | (bit % 8);
+1 & (bit % 8);
+1 ^ (bit % 8);
+1 >>> (bit % 8);
 (a + b) % c;
 a % (b + c);
 1 << (bit * 8);
 1 >> (bit / 8);
 1 << (bit + 8);
 1 >> (bit - 8);

```

**Prettier Similarity**: 61.54%


### js/call/boolean/boolean.js
```diff
-a = Boolean(
-  a_long_long_long_long_condition ||
-    a_long_long_long_long_condition ||
-    a_long_long_long_long_condition,
-);
-a = Boolean(
-  a_long_long_long_long_condition ||
-    a_long_long_long_long_condition ||
-    a_long_long_long_long_condition,
-)?.toString();
-a = new Boolean(
-  a_long_long_long_long_condition ||
-    a_long_long_long_long_condition ||
-    a_long_long_long_long_condition,
-);
-a = !(
-  a_long_long_long_long_condition ||
-  a_long_long_long_long_condition ||
-  a_long_long_long_long_condition
-);
-a = !!(
-  a_long_long_long_long_condition ||
-  a_long_long_long_long_condition ||
-  a_long_long_long_long_condition
-);
+// FIXME
+// TODO: reformat issue
+// a = Boolean(
+//   a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition,
+// );
+// a = Boolean(
+//   a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition,
+// )?.toString();
+// a = new Boolean(
+//   a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition,
+// );
+// a = !(
+//   a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition
+// );
+// a = !!(
+//   a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition
+// );
 
-a = Boolean(
-  a_long_long_long_long_condition ||
-    a_long_long_long_long_condition ||
-    a_long_long_long_long_condition,
-  anotherArgument,
-);
-a = Boolean(
-  a_long_long_long_long_condition ||
-    a_long_long_long_long_condition ||
-    a_long_long_long_long_condition,
-  anotherArgument,
-);
-a = new Foo(
-  a_long_long_long_long_condition ||
-    a_long_long_long_long_condition ||
-    a_long_long_long_long_condition,
-  anotherArgument,
-);
+// a = Boolean(
+//   a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition,
+//   anotherArgument,
+// );
+// a = Boolean(
+//   a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition,
+//   anotherArgument,
+// );
+// a = new Foo(
+//   a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition,
+//   anotherArgument,
+// );
 
-// Different operators
-a = Boolean(
-  a_long_long_long_long_condition &&
-    a_long_long_long_long_condition &&
-    a_long_long_long_long_condition,
-);
-a = Boolean(
-  a_long_long_long_long_condition ??
-    a_long_long_long_long_condition ??
-    a_long_long_long_long_condition,
-);
-a = Boolean(
-  a_long_long_long_long_condition +
-    a_long_long_long_long_condition +
-    a_long_long_long_long_condition,
-);
+// // Different operators
+// a = Boolean(
+//   a_long_long_long_long_condition && a_long_long_long_long_condition && a_long_long_long_long_condition,
+// );
+// a = Boolean(
+//   a_long_long_long_long_condition ?? a_long_long_long_long_condition ?? a_long_long_long_long_condition,
+// );
+// a = Boolean(
+//   a_long_long_long_long_condition + a_long_long_long_long_condition + a_long_long_long_long_condition,
+// );
 
-// Not argument
-a = (
-  a_long_long_long_long_condition ||
-  a_long_long_long_long_condition ||
-  a_long_long_long_long_condition
-)(Boolean);
-a = new (a_long_long_long_long_condition ||
-  a_long_long_long_long_condition ||
-  a_long_long_long_long_condition)(Foo);
+// // Not argument
+// a = (
+//   a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition
+// )(Boolean);
+// a = new (
+//   a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition
+// )(Foo);
 
-// Not `Boolean`
-a = not_Boolean(
-  a_long_long_long_long_condition ||
-    a_long_long_long_long_condition ||
-    a_long_long_long_long_condition,
-);
+// // Not `Boolean`
+// a = not_Boolean(
+//   a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition,
+// );
 
-// Nested
-a = Boolean(
-  (a_long_long_long_long_condition ||
-    a_long_long_long_long_condition ||
-    a_long_long_long_long_condition) &&
-    (a_long_long_long_long_condition ||
-      a_long_long_long_long_condition ||
-      a_long_long_long_long_condition),
-);
-a = Boolean(
-  a_long_long_long_long_condition ||
-    a_long_long_long_long_condition ||
-    a_long_long_long_long_condition ||
-    a_long_long_long_long_condition ||
-    a_long_long_long_long_condition ||
-    a_long_long_long_long_condition,
-);
-a = Boolean(
-  Boolean(
-    a_long_long_long_long_condition ||
-      a_long_long_long_long_condition ||
-      a_long_long_long_long_condition,
-  ),
-);
+// // Nested
+// a = Boolean(
+//   (a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition)
+//   &&
+//   (a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition)
+// );
+// a = Boolean(
+//   (a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition)
+//   ||
+//   (a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition)
+// );
+// a = Boolean(Boolean(
+//   a_long_long_long_long_condition || a_long_long_long_long_condition || a_long_long_long_long_condition,
+// ));

```

**Prettier Similarity**: 4.85%


### js/class-comment/superclass.js
```diff
 class A // comment 1
   // comment 2
   extends B {}
 
 class A1 extends B {
   // comment1
   // comment2
   // comment3
 }
 
 class A2 /* a */ extends B {}
 class A3 extends B /* a */ {}
 class A4 extends /* a */ B {}
 
 (class A5 // comment 1
   // comment 2
   extends B {});
 
 (class A6 extends B {
   // comment1
   // comment2
   // comment3
 });
 
 (class A7 /* a */ extends B {});
 (class A8 extends B /* a */ {});
 (class A9 extends /* a */ B {});
 
 class a extends b {
   // comment
   constructor() {}
 }
 
 class c extends d {
   // comment2
   constructor() {}
 }
 
 class C2 // comment
   extends Base
 {
   foo() {}
 }
 
-(class A10
-  extends /* a */ /* prettier-ignore */ /* a */ /* prettier-ignore */ B {});
-class A10
-  extends /* a */ /* prettier-ignore */ /* a */ /* prettier-ignore */ B {}
+(class A10 extends /* a */ /* prettier-ignore */ B {});
+class A10 extends /* a */ /* prettier-ignore */ B {}

```

**Prettier Similarity**: 91.67%


### js/classes/multiple-static.js
```diff
 class C {
-  static static;
-  static a() {}
+  static
+  static
+  static
+  a() {}
 }

```

**Prettier Similarity**: 33.33%


### js/comments-closure-typecast/comment-in-the-middle.js
```diff
 var a =
   /**
    * bla bla bla
    * @type {string |
    * number
    * }
    * bla bla bla
    */
   //2
-  window["s"].toString();
+  (window["s"]).toString();
 console.log(a.foo());

```

**Prettier Similarity**: 90.91%


### js/comments/15661.js
```diff
 !(
   (
     x || // foo
     y || // bar
     z
   ) /*
    * comment
    */
 );
 
 !(
-  cond1 || // foo
-  cond2 || // bar
-  cond3 // baz
+  (
+    cond1 || // foo
+    cond2 || // bar
+    cond3
+  ) // baz
 );
 
 !(
-  (a && // alpha
-    b) || // bravo
-  c // charlie
+  (
+    (a && // alpha
+      b) || // bravo
+    c
+  ) // charlie
 );
 
 !(
-  x || // foo
-  (y && z) // bar
+  (
+    x || // foo
+    (y && z)
+  ) // bar
 );
 
 !(
-  a || // first condition
-  b ||
-  c || // second condition
-  d // third condition
+  (
+    a || // first condition
+    b ||
+    c || // second condition
+    d
+  ) // third condition
 );
 
 void (
   (
     (p && // first
       q) || // second
     (r && // third
       s)
   ) // fourth
 );
 
 void (
-  cond1 || // foo
-  (cond2 && // bar
-    cond3) || // baz
-  cond4 // qux
+  (
+    cond1 || // foo
+    (cond2 && // bar
+      cond3) || // baz
+    cond4
+  ) // qux
 );
 
 !(
-  (cond1 && cond2) || // multi-cond1
-  cond3 ||
-  cond4 // multi-cond2
+  (
+    (cond1 && cond2) || // multi-cond1
+    cond3 ||
+    cond4
+  ) // multi-cond2
 );
 
 !(
-  ((cond1 || cond2) && // complex-cond1
-    (cond3 || cond4)) || // complex-cond2
-  cond5 // complex-cond3
+  (
+    ((cond1 || cond2) && // complex-cond1
+      (cond3 || cond4)) || // complex-cond2
+    cond5
+  ) // complex-cond3
 );
 
 void (
-  ((condA || condB) && // test A
-    (condC || condD)) || // test B
-  condE // test C
+  (
+    ((condA || condB) && // test A
+      (condC || condD)) || // test B
+    condE
+  ) // test C
 );
 
 void (
-  (x || y) && // nested
-  (z || w) // comment for w
+  (
+    (x || y) && // nested
+    (z || w)
+  ) // comment for w
 );
 
 !(
-  a && // begin nested
-  (b || c) // end nested
+  (
+    a && // begin nested
+    (b || c)
+  ) // end nested
 );

```

**Prettier Similarity**: 49.48%


### js/comments/16398.js
```diff
 if (foo) a = b;
 /* foo */ else foo.split;
 
 if (foo) a = b;
-else /* foo */ foo.split;
+/* foo */ else foo.split;

```

**Prettier Similarity**: 80.00%


### js/comments/empty-statements.js
```diff
-a; /* a */ // b
-/* c */
-foo; // first
-// second
-// third
-function x() {} // first
-// second
+a; /* a */ /* c */ // b
+
+foo; // first // second // third
+
+function x() {} // first // second
+
 a =
-  b + // 1
-  // 2
-  c + // 3
-  // 4
-  d + // 5
-  /* 6 */
+  b + // 1 // 2
+  c + // 3 // 4
+  d /* 6 */ + // 5
   e; // 7

```

**Prettier Similarity**: 13.33%


### js/comments/export.js
```diff
 export //comment
- {};
+{};
 
 export /* comment */ {};
 
 const foo = "";
 export {
   foo, // comment
 };
 
 const bar = "";
 export {
   // comment
   bar,
 };
 
 const fooo = "";
 const barr = "";
 export {
   fooo, // comment
   barr, // comment
 };
 
 const foooo = "";
 const barrr = "";
 export {
   foooo,
   // comment
   barrr as baz,
 } from "foo";
 
 const fooooo = "";
 const barrrr = "";
 export {
   fooooo,
   // comment
   barrrr as bazz,
 };

```

**Prettier Similarity**: 97.37%


### js/comments/html-like/comment.js
```diff
 <!--
-alert(1); 
+alert(1)
 -->

```

**Prettier Similarity**: 66.67%


### js/comments/multi-comments-on-same-line.js
```diff
 /*========= All on same line =========*/
 a;
 /*1*/ /*2*/ /*3*/
 b;
 
 a; /*1*/ /*2*/ /*3*/
 b;
 
 a;
 /*1*/ /*2*/ /*3*/ b;
 
 a;
 /*
 1*/ /*2*/ /*3
  */
 b;
 
 a; /*
 1*/ /*2*/ /*3
  */
 b;
 
-a;
-/*
+a; /*
 1*/ /*2*/ /*3
- */ b;
+ */
+b;
 
 /*========= First two on same line =========*/
 a;
 /*1*/ /*2*/
 /*3*/
 b;
 
 a; /*1*/ /*2*/
 /*3*/
 b;
 
 a;
 /*1*/ /*2*/
 /*3*/ b;
 
 a;
 /*
 1*/ /*2*/
 /*3
  */
 b;
 
 a; /*
 1*/ /*2*/
 /*3
  */
 b;
 
 a; /*
 1*/ /*2*/
 /*3
  */ b;
 
 /*========= Last two on same line =========*/
 a;
 /*1*/
 /*2*/ /*3*/
 b;
 
 a; /*1*/
 /*2*/ /*3*/
 b;
 
 a;
 /*1*/
 /*2*/ /*3*/ b;
 
 a;
 /*
 1*/
 /*2*/ /*3
  */
 b;
 
 a; /*
 1*/
 /*2*/ /*3
  */
 b;
 
 a; /*
 1*/
 /*2*/ /*3
  */ b;

```

**Prettier Similarity**: 96.67%


### js/comments/return-statement.js
```diff
 function jsx() {
   return (
     // Comment
     <div />
   );
 }
 
 function unary() {
   return (
     // Comment
     !!x
   );
 }
 
 function numericLiteralNoParen() {
   return 1337; // Comment
 }
 
 function logical() {
   return (
     // Reason for 42
     42 && 84
   );
 }
 
 function binary() {
   return (
     // Reason for 42
     42 * 84
   );
 }
 
 function binaryInBinaryLeft() {
   return (
     // Reason for 42
     42 * 84 + 2
   );
 }
 
 function binaryInBinaryRight() {
   return (
     // Reason for 42
     42 + 84 * 2
   );
 }
 
 function conditional() {
   return (
     // Reason for 42
     42 ? 1 : 2
   );
 }
 
 function binaryInConditional() {
   return (
     // Reason for 42
     42 * 3 ? 1 : 2
   );
 }
 
 function call() {
   return (
     // Reason for a
     a()
   );
 }
 
 function memberInside() {
   return (
     // Reason for a.b
     a.b.c
   );
 }
 
 function memberOutside() {
   return (
     // Reason for a
     a.b.c
   );
 }
 
 function memberInAndOutWithCalls() {
-  return aFunction
-    .b // Reason for a
-    ()
-    .c.d();
+  return (
+    // Reason for a
+    aFunction
+      .b()
+      .c.d()
+  );
 }
 
 function excessiveEverything() {
   return (
     // Reason for stuff
     a.b() * 3 + 4 ? ((a`hi`, 1) ? 1 : 1) : 1
   );
 }
 
 // See https://github.com/prettier/prettier/issues/2392
 // function sequenceExpression() {
 //   return (
 //     // Reason for a
 //     a
 //   ), b
 // }
 
 function sequenceExpressionInside() {
   return (
     // Reason for a
-    (a, b)
+    a, b
   );
 }
 
 function taggedTemplate() {
   return (
     // Reason for a
     a`b`
   );
 }
 
 function inlineComment() {
   return /* hi */ 42 || 42;
 }
 
 function multilineBlockSameLine() {
   return (
     /**
      * @type {string}
      */ "result"
   );
 }
 
 function multilineBlockNextLine() {
   return (
     /**
      * @type {string}
      */
     "result"
   );
 }
 
 function multilineBlockSameLineJsx() {
   return (
     /**
      * JSX Same line
      */ <div></div>
   );
 }
 
 function multilineBlockNextLineJsx() {
   return (
     /**
      * JSX Next line
      */
     <div></div>
   );
 }
 
 function singleLineBlockSameLine() {
   return /** Result -> */ "result";
 }
 
 function singleLineBlockNextLine() {
   return (
     /** Result below */
     "result"
   );
 }

```

**Prettier Similarity**: 95.81%


### js/comments/tagged-template-literal.js
```diff
 foo``; // comment 1
 
 foo // comment 2
 ``;
 
 foo // comment 3
 `
 `;
 
-foo /* comment 4 */ `
+foo /* comment 4 */`
 `;
 
-foo /* comment 5 */ `
+foo /* comment 5 */`
 `;

```

**Prettier Similarity**: 85.71%


### js/comments/tagged-template-literal/11662.js
```diff
-foo
-// TEST
+foo`// TEST
 // 1
 // 2
 // 3
 // 4
 // 5
-`x`;
+x`;

```

**Prettier Similarity**: 62.50%


### js/conditional/comments.js
```diff
 var inspect =
   4 === util.inspect.length
     ? // node <= 0.8.x
       function (v, colors) {
         return util.inspect(v, void 0, void 0, colors);
       }
     : // node > 0.8.x
       function (v, colors) {
         return util.inspect(v, { colors: colors });
       };
 
 var inspect =
   4 === util.inspect.length
     ? // node <= 0.8.x
       function (v, colors) {
         return util.inspect(v, void 0, void 0, colors);
       }
     : // node > 0.8.x
       function (v, colors) {
         return util.inspect(v, { colors: colors });
       };
 
 const extractTextPluginOptions = shouldUseRelativeAssetPaths
   ? // Making sure that the publicPath goes back to to build folder.
     { publicPath: Array(cssFilename.split("/").length).join("../") }
   : {};
 
 const extractTextPluginOptions2 = shouldUseRelativeAssetPaths
   ? // Making sure that the publicPath goes back to to build folder.
     { publicPath: Array(cssFilename.split("/").length).join("../") }
   : {};
 
 const extractTextPluginOptions3 = shouldUseRelativeAssetPaths // Making sure that the publicPath goes back to to build folder.
   ? { publicPath: Array(cssFilename.split("/").length).join("../") }
   : {};
 
 const { configureStore } =
   process.env.NODE_ENV === "production"
     ? require("./configureProdStore") // a
     : require("./configureDevStore"); // b
 
 test /* comment
   comment
       comment
 */
   ? foo
   : bar;
 
 test
   ? /* comment
           comment
     comment
           comment
   */
     foo
   : bar;
 
 test
   ? /* comment
        comment
        comment
        comment
     */
     foo
   : test
     ? /* comment
   comment
     comment */
       foo
     : bar;
 
 test ? /* comment */ foo : bar;
 
 test
   ? foo
   : /* comment
          comment
      comment
            comment
     */
     bar;
 
 test
   ? foo
   : /* comment
          comment
      comment
            comment
       A newline will be added after this comment, unfortunately – but it can be removed manually, see next statement.
     */
     test
     ? foo
     : /* comment
   comment
     comment
    */
       bar;
 
 // It is at least possible to delete the extra newline that was
 // unfortunately added before the second condition above:
 test
-  ? foo /* comment
+  ? foo
+  : /* comment
          comment
      comment
            comment
     */
-  : test
+    test
     ? foo
     : /* comment
   comment
     comment
    */
       bar;
 
 test ? foo : /* comment */ bar;
 
 test
   ? test /* c
 c */
     ? foo
     : bar
   : bar;

```

**Prettier Similarity**: 97.56%


### js/conditional/postfix-ternary-regressions.js
```diff
 // concatened string in consequent should be visually distinguishable from alternate
 // … or maybe this is okay, because the colon is enough?
 const avatar = has_ordered
   ? "https://marmelab.com/posters/avatar/longer-word-that-breaks-consequent-" +
     numberOfCustomers +
     ".jpeg"
   : undefined;
 
 // Similarly, in the alternate:
 const redirectUrl = pathName
   ? pathName
   : nextPathName + nextSearch ||
     defaultAuthParams.afterLoginUrl.makeThisLongerSoItBreaks;
 
 // And another, more pathological case of the above:
 const isEmpty = (obj) =>
   obj instanceof Date
     ? false
     : obj === "" ||
       obj === null ||
       obj === undefined ||
       obj === somethingThatIsLonger ||
       shallowEqual(obj, {});
 
 // Again, this case is a bit hard to distinguish the alternate.
 const eventsFromOrders =
   orderIds && orders
     ? orderIds.map((id) => ({
         type: "order",
         date: orders[id].date,
         data: orders[id],
       }))
     : [];
 
 // Kinda weird to have dedents to the level of "return" in a function.
 function foo() {
   return !linkTo
     ? false
     : typeof linkTo === "function"
       ? linkTo(record, reference)
       : linkToRecord(rootPath, sourceId, linkTo_as_string);
 }
 function foo2() {
   return React.isValidElement(emptyText)
     ? React.cloneElement(emptyText)
     : emptyText === ""
       ? " " // em space, forces the display of an empty line of normal height
       : translate(emptyText, { _: emptyText });
 }
 
 // Function call ideally wouldnt break break
 const matchingReferencesError = isMatchingReferencesError(matchingReferences)
   ? translate(matchingReferences.error, {
       _: matchingReferences.error,
     })
   : null;
 
 // This one is kinda confusing any way you slice it…
 const obj = {
   error:
     matchingReferencesError &&
     (!input.value ||
       (input.value && selectedReferencesDataStatus === REFERENCES_STATUS_EMPTY))
       ? translate("ra.input.references.all_missing", {
           _: "ra.input.references.all_missing",
         })
       : null,
 };
 
 // I think we should indent after the inner || on this, and do better wtih the parens around the &&
 const obj2 = {
   warning:
     matchingReferencesError ||
     (input.value && selectedReferencesDataStatus !== REFERENCES_STATUS_READY)
       ? matchingReferencesError ||
         translate("ra.input.references.many_missing", {
           _: "ra.input.references.many_missing",
         })
       : null,
 };
 
 // The boolean conditions in the test should look cohesive.
 const selectedReferencesDataStatus =
   !isEmpty(value) && typeof value === "string" && !pattern.test(value)
     ? getMessage(message, { pattern }, value, values)
     : undefined;
 
 // Would be nice if these two nested ternaries didn't look like a single one.
 resolveRedirectTo(
   redirectTo,
   basePath,
   payload
     ? payload.id || (payload.data ? payload.data.id : null)
     : requestPayload
       ? requestPayload.id
       : null,
   payload && payload.data
     ? payload.data
     : requestPayload && requestPayload.data
       ? requestPayload.data
       : null,
 );
 
 const delayedDataProvider = new Proxy(restProvider, {
   get: (target, name, self) =>
-    name === "then" // as we await for the dataProvider, JS calls then on it. We must trap that call or else the dataProvider will be called with the then method
-      ? self
+    name === "then"
+      ? // as we await for the dataProvider, JS calls then on it. We must trap that call or else the dataProvider will be called with the then method
+        self
       : (resource, params) =>
           new Promise((resolve) =>
             setTimeout(
               () => resolve(restProvider[name](resource, params)),
               500,
             ),
           ),
 });
 
 function foo4() {
   return !match || match.length < 5
     ? line
     : match[1] + match[2] + match[3] + match[4];
 }
 
 function foo5() {
   return !match || match.length < 5
     ? foo(line)
     : match[1] + match[2] + match[3] + match[4];
 }
 
 function foo6() {
   return !match || match.length < 5
     ? linethatisverylongandbreaksthelinehooray
     : match[1] + match[2] + match[3] + match[4];
 }
 
 function foo7() {
   return !match || match.length < 5
     ? linethatisverylongandbreaksthelinehoorayjustabitlonger
     : match[1] + match[2] + match[3] + match[4];
 }
 
 const badComments = schema.model
   ? schema
   : // If model is an array where the items schema is a referred model then we need to use that
     schema.type === "array"
     ? schema.items
     : schema;
 
 const anotherBadComment = refModel
   ? // If we're in a shared params file then reference the model name directly
     inSharedParamsFile
     ? refModel
     : // If we're not in a shared params file then reference the in-file type
       classRef()
   : // We don't have a model name, use the in-file name
     classRef();

```

**Prettier Similarity**: 98.08%


### js/discard-binding/array-pattern.js
```diff
 {
   // destructuring binding
-  const [, void] = value;
+  const [, void, ] = value;
 }
 {
   // for-of destructuring binding
   for (const [void] of []);
 }
 {
   // destructuring assignment
   [void] = [];
 }
 {
   // for-of destructuring assignment;
   for ([void] of []);
 }
 {
   // function param;
-  function f([[[void], void], void]) {}
+  function f([[[void], void, ], void]) {}
 }
 {
   // arrow function param
-  ([[[void], void], void]) => {};
+  ([[[void], void, ], void])
+  =>
+  {
+  }
 }
 {
   // async arrow function param
-  async ([[[void], void], void]) => {};
+  async ([[[void], void, ], void])
+  =>
+  {
+  }
 }
 {
   // destructuring assignment in async call";
   async(([void] = []));
 }
 {
   // catch param
-  try {
-  } catch ([void]) {}
+  try {} catch ([void]) {}
 }

```

**Prettier Similarity**: 73.81%


### js/discard-binding/basic.js
```diff
-const [void] = ([, void] = [void] = []);
+const [void] = [,void] = [void,] = [];
 
-function f(void, { p: void }, [void]) {}
+function f(void, { p: void }, [ void ]) {}
 
-(void, { p: void }, [void]) => {};
+(void, { p: void }, [ void ])
+=>
+{
+}
 
-async (void, { p: void }, [void]) => {};
+async (void, { p: void }, [ void ])
+=>
+{
+}

```

**Prettier Similarity**: 23.08%


### js/discard-binding/discard-binding-arrow-params.js
```diff
-(void) => {};
+(void)
+=>
+{
+}

```

**Prettier Similarity**: 0.00%


### js/discard-binding/discard-binding-async-arrow-params.js
```diff
-async (void) => {};
+async (void)
+=>
+{
+}

```

**Prettier Similarity**: 0.00%


### js/discard-binding/discard-binding-for-await-using-binding.js
```diff
 async () => {
-  for (await using void of []);
+  for(await using void of []);
 };

```

**Prettier Similarity**: 66.67%


### js/discard-binding/discard-binding-for-bindings.js
```diff
-for (const { p: void } of []);
+for(const { p: void } of []);

```

**Prettier Similarity**: 0.00%


### js/discard-binding/discard-binding-for-using-binding.js
```diff
 {
-  for (using void of []);
+  for(using void of []);
 }

```

**Prettier Similarity**: 66.67%


### js/discard-binding/function-parameter.js
```diff
 {
   // function parameter
   function f(void) {}
 }
 {
   // arrow function parameter
-  (x, void) => {};
+  (x, void)
+  =>
+  {
+  }
 }
 {
   // async arrow function parameter
-  async (void, x) => {};
+  async (void, x)
+  =>
+  {
+  }
 }
 {
   // object method parameter
   ({ f(x, void, y) {} });
 }
 {
   // class method parameter
   class C {
-    m(void) {}
+    m(void,) {}
   }
 }

```

**Prettier Similarity**: 67.86%


### js/discard-binding/object-pattern.js
```diff
 {
   // destructuring binding
   const { void: void } = value;
 }
 {
   // for-of destructuring binding
   for (const { p: void } of []);
 }
 {
   // destructuring assignment
   ({ p: void } = {});
 }
 {
   // for-of destructuring assignment
   for ({ p: void } of []);
 }
 {
   // function param
-  function f({
-    q: {
-      q: { p: void },
-      p: void,
-    },
-    p: void,
-  }) {}
+  function f({ q: { q: { p: void }, p: void }, p: void }) {}
 }
 {
   // arrow function param
-  ({
-    q: {
-      q: { p: void },
-      p: void,
-    },
-    p: void,
-  }) => {};
+  ({ q: { q: { p: void }, p: void }, p: void }) => {}
 }
 {
   // async arrow function param
-  async ({
-    q: {
-      q: { p: void },
-      p: void,
-    },
-    p: void,
-  }) => {};
+  async ({ q: { q: { p: void }, p: void }, p: void }) => {}
 }
 {
   // destructuring assignment in async call
   async(({ p: void } = {}));
 }
 {
   // catch param
-  try {
-  } catch ({ p: void }) {}
+  try {} catch ({ p: void }) {}
 }

```

**Prettier Similarity**: 58.18%


### js/discard-binding/unary-expression-void.js
```diff
 // expr
 void [
   // pattern
-  ([void] = [
-    {
-      // expr
-      [void []]:
-        // pattern
-        // expr
-        [void] = void [
-          // pattern
-          ([void]) => [],
-        ],
-      // expr
-    },
-  ] =
-    [void []]),
-];
+  [ void ] = [{
+    // expr
+    [ void [] ]:
+    // pattern
+    [ void ] =
+    // expr
+    void [
+      // pattern
+      ([void]) => []
+    ]
+    // expr
+  }] = [void []]
+]

```

**Prettier Similarity**: 16.67%


### js/discard-binding/using-variable-declarator.js
```diff
 {
   // using 1 declarator
-  using void = f();
+  using;
+  void = f();
 }
 {
   // using 2 declarators
-  using void = f(),
-    void = g();
+  using;
+  (void = f()), (void = g());
 }
 {
   // using void declarator and normal declarator
-  using void = f(),
-    x = g();
+  using;
+  (void = f()), (x = g());
 }
 {
   // using declarator in for-of
-  for (using void of []);
+  for(using void of []);
 }
 async () => {
   {
     // await using 1 declarator
-    await using void = f();
+    await using;
+    void = f();
   }
   {
     // await using 2 declarators
-    await using void = f(),
-      void = g();
+    await using;
+    (void = f()), (void = g());
   }
   {
     // await using void declarator and normal declarator
-    await using void = f(),
-      x = g();
+    await using;
+    (void = f()), (x = g());
   }
   {
     // await using declarator in for-of
-    for (await using void of []);
+    for(await using void of []);
   }
 };

```

**Prettier Similarity**: 65.00%


### js/discard-binding/using.js
```diff
 {
-  using void = f();
+  using;
+  void = f();
+}
+async (void)
+=>
+{
+  await using;
+  void = f();
 }
-async (void) => {
-  await using void = f();
-};

```

**Prettier Similarity**: 20.00%


### js/explicit-resource-management/valid-await-using-binding-escaped.js
```diff
 async function f() {
-  await using ab = c;
+  await using \u0061b = c;
 }

```

**Prettier Similarity**: 66.67%


### js/explicit-resource-management/valid-await-using-comments.js
```diff
 async function f() {
   {
-    /*0*/ await using /*1*/ /*2*/ b /*3*/ = /*4*/ f(); /*5*/
+    /*0*/ await using /*1*/ /*2*/ b /*3*/ = /*4*/ f() /*5*/;
   }
   {
     /*0*/ for (
       /*1*/ /*2*/ await using /*3*/ /*4*/ b /*5*/ =
           /*6*/ x /*7*/ /*8*/ /*9*/ /*10*/;
       ;
-
     );
   }
   {
     /*0*/ for (/*1*/ /*2*/ await using /*3*/ /*4*/ b /*5*/ of /*6*/ x /*7*/ /*8*/);
   }
   {
     /*0*/ for await (/*1*/ /*2*/ /*3*/ await using /*4*/ /*5*/ b /*6*/ of /*7*/ x /*8*/ /*9*/);
   }
 }

```

**Prettier Similarity**: 89.47%


### js/explicit-resource-management/valid-using-binding-escaped.js
```diff
 {
-  using ab = c;
+  using \u0061b = c;
 }

```

**Prettier Similarity**: 66.67%


### js/export/blank-line-between-specifiers.js
```diff
 export {
   // a
   foo1,
-
   // b
   bar1,
   baz1,
 } from "mod";
 
 const foo2 = 1;
 const bar2 = 1;
 const baz2 = 1;
 
 export {
   // a
   foo2,
-
   // b
   bar2,
   baz2,
 };

```

**Prettier Similarity**: 90.48%


### js/for/9812-unstable.js
```diff
-for (x of y) { // comment
+// comment
+for (x of y) {
   bar();
 }
 
-for (x of y) { // comment
+// comment
+for (x of y) {
 }
 
-for (x of y); // comment
+// comment
+for (x of y);

```

**Prettier Similarity**: 45.45%


### js/for/continue-and-break-comment-without-blocks.js
```diff
 for (;;) continue;
 // comment
 
 for (;;) break;
 // comment
 
 for (const f of []) continue;
 // comment
 
 for (const f of []) break;
 // comment
 
 for (const f in {}) continue;
 // comment
 
 for (const f in {}) break;
 // comment
 
 for (;;) continue; // comment
 
 for (;;) break; // comment
 
 for (const f of []) continue; // comment
 
 for (const f of []) break; // comment
 
 for (const f in {}) continue; // comment
 
 for (const f in {}) break; // comment
 
 for (;;) continue; /* comment */
 
 for (;;) break; /* comment */
 
 for (const f of []) continue; /* comment */
 
 for (const f of []) break; /* comment */
 
 for (const f in {}) continue; /* comment */
 
 for (const f in {}) break; /* comment */
 
 for (;;) continue;
 /* comment */
 
 for (;;) break;
 /* comment */
 
 for (const f of []) continue;
 /* comment */
 
 for (const f of []) break;
 /* comment */
 
 for (const f in {}) continue;
 /* comment */
 
 for (const f in {}) break;
 /* comment */
 
-label1: for (;;) continue label1 /* comment */;
+label1: for (;;) continue label1; /* comment */
 
 label1: for (;;) continue label1;
 /* comment */
 
 label1: for (;;) continue label1; // comment
 
 label1: for (;;) continue label1;
 // comment

```

**Prettier Similarity**: 98.55%


### js/for/for-in-with-initializer.js
```diff
 // https://github.com/babel/babel/blob/HEAD/packages/babel-generator/test/fixtures/parentheses/in-inside-for/input.js
 
 for (var a = (b in c) in {});
-for (var a = (1 || b in c) in {});
-for (var a = (1 + (2 || b in c)) in {});
-for (var a = (() => b in c) in {});
-for (var a = (1 || (() => b in c)) in {});
-for (var a = (() => {
+for (var a = 1 || (b in c) in {});
+for (var a = 1 + (2 || (b in c)) in {});
+for (var a = () => (b in c) in {});
+for (var a = 1 || (() => (b in c)) in {});
+for (var a = () => {
   b in c;
-}) in {});
-for (var a = ([b in c]) in {});
-for (var a = ({ b: b in c }) in {});
+} in {});
+for (var a = [(b in c)] in {});
+for (var a = { b: (b in c) } in {});
 // Meriyah can't parse
 // for (var a = (x = b in c) => {} in {});
-for (var a = (class extends (b in c) {}) in {});
-for (var a = (function (x = b in c) {}) in {});
+for (var a = class extends (b in c) {} in {});
+for (var a = function (x = (b in c)) {} in {});

```

**Prettier Similarity**: 37.50%


### js/for/parentheses.js
```diff
 // https://github.com/babel/babel/blob/HEAD/packages/babel-generator/test/fixtures/parentheses/in-inside-for/input.js
 
 for (var a = (b in c); ; );
 for (var a = 1 || (b in c); ; );
 for (var a = 1 + (2 || (b in c)); ; );
 for (var a = () => (b in c); ; );
 for (var a = 1 || (() => (b in c)); ; );
 for (
   var a = () => {
-    (b in c);
+    b in c;
   };
   ;
-
 );
 for (var a = [(b in c)]; ; );
 for (var a = { b: (b in c) }; ; );
 for (var a = (x = (b in c)) => {}; ; );
 for (var a = class extends (b in c) {}; ; );
 for (var a = function (x = (b in c)) {}; ; );
 
 for (var a in b in c);
 for (var a in 1 || b in c);
 for (var a in 1 + (2 || b in c));
 for (var a in () => b in c);
 for (var a in 1 || (() => b in c));
 for (var a in () => {
   b in c;
 });
 for (var a in [b in c]);
 for (var a in { b: b in c });
 for (var a in (x = b in c) => {});
 for (var a in class extends (b in c) {});
 for (var a in function (x = b in c) {});
 
 for (; (a = b in c); );
 for (; (a = 1 || b in c); );
 for (; (a = 1 + (2 || b in c)); );
 for (; (a = () => b in c); );
 for (; (a = 1 || (() => b in c)); );
 for (
   ;
   (a = () => {
     b in c;
   });
-
 );
 for (; (a = [b in c]); );
 for (; (a = { b: b in c }); );
 for (; (a = (x = b in c) => {}); );
 for (; (a = class extends (b in c) {}); );
 for (; (a = function (x = b in c) {}); );

```

**Prettier Similarity**: 94.12%


### js/identifier/parentheses/let.js
```diff
 let.a = 1;
 
 let.a[0] = 1;
 
 (let)[a] = 1;
 
 (let)[a].b.c.e = 1;
 
 foo[let[a]] = 1;
 
 (let)[let[a]] = 1;
 
 (let)[a] ??= 1;
 
 foo = let[a];
 
 let()[a] = 1;
 
 foo(let)[a] = 1;
 
 foo(let[a])[a] = 1;
 
 (let)[0] = 1;
 
 (let)["a"] = 1;
 
 let = 1;
 
 var let = 1;
 
 [let[a]] = 1;
 
 ({ a: let[a] } = 1);
 
 alert((let[0] = 1));
 
 ((let)[0] = 1) || 2;
 
-(((let)[0] = 1), 2);
+((let)[0] = 1), 2;
 
 ((let)[0] = 1) ? a : b;
 
 if ((let[0] = 1));
 
 while ((let[0] = 1));
 
 do {} while ((let[0] = 1));
 
 var a = (let[0] = 1);
 
 ((let)[0] = 1) instanceof a;
 
 void (let[0] = 1);
 
 ((let)[0] = 1)();
 
 new (let[0] = 1)();
 
 ((let)[0] = 1)``;
 
 ((let)[0] = 1).toString;
 
 ((let)[0] = 1)?.toString;
 
 [...(let[0] = 1)];
 
 foo = () => (let[0] = 1);
 
 function* foo() {
   yield (let[0] = 1);
 }
 
 async function foo() {
   await (let[0] = 1);
 }
 
 function foo() {
   return (let[0] = 1);
 }
 
 while (true) (let)[0] = 1;
 
 throw (let[0] = 1);
 
 ({ foo: (let[0] = 1) });
 
 [(let[0] = 1)];
 
 for ((let)[0] = 1; ; );
 for ((let)[0] in {});
 for ((let)[0] of []);
 
 switch ((let[0] = 1)) {
 }
 
 switch (foo) {
   case (let[0] = 1):
 }
 
 with ((let[0] = 1));
 
 (let)[x].foo();
 
 let.let[x].foo();
 
 a = let[x].foo();
 
 (let)[2];
 
 a[1] + (let[2] = 2);

```

**Prettier Similarity**: 99.09%


### js/if/comment-between-condition-and-body.js
```diff
 if (1) {
   // foo may not exist
   doThing(foo);
 }
 if (1) {
 } else {
   // foo may not exist
   doThing(foo);
 }
 
 if (2) {
   // foo may not exist
   doThing(foo);
 }
 if (2) {
-} // foo may not exist
+}
+// foo may not exist
 else {
   doThing(foo);
 }
 
 if (3) {
   // foo may not exist
   doThing(foo);
 }
 if (3) {
 } // foo may not exist
 else {
   doThing(foo);
 }
 
 if (4) {
   /* foo may not exist */ doThing(foo);
 }
 if (4) {
 } /* foo may not exist */ else {
   doThing(foo);
 }

```

**Prettier Similarity**: 94.74%


### js/if/expr_and_same_line_comments.js
```diff
 if (a === 0)
   doSomething(); // comment A1
 else if (a === 1)
   doSomethingElse(); // comment B1
 else if (a === 2) doSomethingElse(); // comment C1
 
 if (a === 0) doSomething(); /* comment A2 */
 else if (a === 1) doSomethingElse(); /* comment B2 */
 else if (a === 2) doSomethingElse(); /* comment C2 */
 
 if (a === 0)
   expr; // comment A3
 else if (a === 1)
   expr; // comment B3
 else if (a === 2) expr; // comment C3
 
 if (a === 0) expr; /* comment A4 */
 else if (a === 1) expr; /* comment B4 */
 else if (a === 2) expr; /* comment C4 */
 
 if (a === 0)
   looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment A5
 else if (a === 1)
   looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment B5
 else if (a === 2)
   looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment C5
 
 function a() {
   if (a) return; /* comment 6a */
   else return 2;
 
   if (a) return 1; /* comment 6b */
   else return 2;
 
   if (a) throw e; /* comment 6d */
   else return 2;
 
   if (a) var a = 1; /* comment 6e */
   else return 2;
 
   if (a)
-    if (b /* comment 6f */);
+    if (b); /* comment 6f */
     else return 2;
 }

```

**Prettier Similarity**: 97.73%


### js/if/non-block.js
```diff
 if (foo)
   for (i = 2; i > 0; i--) console.log(i); // foo
 else bar();
 
 if (foo)
   do {
     console.log(i);
-  } while (i--);
-// foo
+  } while (i--); // foo
 else bar();

```

**Prettier Similarity**: 80.00%


### js/ignore/class-expression-decorator.js
```diff
-// prettier-ignore
 (
+  // prettier-ignore
   @decorator
   class {}
 );

```

**Prettier Similarity**: 80.00%


### js/ignore/issue-11077.js
```diff
 function HelloWorld(x) {
-  // prettier-ignore
-  // eslint-disable-next-line
-  (x.a |
-    x.b).call(null);
+  (
+    // prettier-ignore
+    // eslint-disable-next-line
+    x.a |
+    x.b
+  ).call(null)
 }
 
 function HelloWorld(x) {
   // prettier-ignore
+
   (
     // eslint-disable-next-line
     x.a |
     x.b
   ).call(null)
 }

```

**Prettier Similarity**: 61.11%


### js/ignore/issue-13737.js
```diff
 oneArgument(
   // prettier-ignore
   (0, 1),
 );
 
 a =
-  // prettier-ignore
-  (0, 1);
+  (
+ // prettier-ignore
+ (0, 1)
+);

```

**Prettier Similarity**: 60.00%


### js/ignore/issue-14404.js
```diff
 async function foo() {
-  // prettier-ignore
-  // b
-  (await thing()).blah;
+  (
+  	// prettier-ignore
+  	// b
+	await thing()
+).blah
 }

```

**Prettier Similarity**: 28.57%


### js/ignore/issue-9877.js
```diff
 export default function test() {
   return {
     matrix: // prettier-ignore
-      new Float32Array([
-      0, 0,
-      1, 0,
-      1, 1,
-      0, 1
-    ]),
+      new Float32Array([0, 0, 1, 0, 1, 1, 0, 1]),
   };
 }

```

**Prettier Similarity**: 45.45%


### js/import-assertions/bracket-spacing/empty.js
```diff
-export * as bar from "bar.json" assert {};
+export * as bar from "bar.json";
+assert;
+{
+}

```

**Prettier Similarity**: 0.00%


### js/import-assertions/bracket-spacing/re-export.js
```diff
-export { default as foo2 } from "foo.json" assert { type: "json" };
+export { default as foo2 } from "foo.json";
+assert;
+{
+  type: "json";
+}

```

**Prettier Similarity**: 0.00%


### js/import-assertions/bracket-spacing/static-import.js
```diff
-import json from "./foo.json" assert { type: "json" };
+import json from "./foo.json";
+assert;
+{
+  type: "json";
+}

```

**Prettier Similarity**: 0.00%


### js/import-assertions/empty.js
```diff
 export * as foo from "foo.json";
-export * as bar from "bar.json" assert {};
-export * as baz from "baz.json" /* comment */ assert {};
+export * as bar from "bar.json";
+assert;
+{
+}
+export * as baz from "baz.json";
+assert;
+{
+  /* comment */
+}
 
 import * as foo from "foo.json";
-import * as bar from "bar.json" assert {};
-import * as baz from "baz.json" /* comment */ assert {};
+import * as bar from "bar.json";
+assert;
+{
+}
+import * as baz from "baz.json";
+assert;
+{
+  /* comment */
+}

```

**Prettier Similarity**: 14.29%


### js/import-assertions/keyword-detect.js
```diff
-import "./test.json" /* with */ /* with */ assert { type: "json" };
-import { default as b } from "./test.json" /* with */ /* with */ assert { type: "json" };
+import "./test.json"; /* with */
+assert; /* with */
+{
+  type: "json";
+}
+import { default as b } from "./test.json"; /* with */
+assert; /* with */
+{
+  type: "json";
+}
 
-export { default as e } from "./test.json" /* with */ /* with */ assert { type: "json" };
+export { default as e } from "./test.json"; /* with */
+assert; /* with */
+{
+  type: "json";
+}
 
-export * from "./test.json" /* with */ /* with */ assert { type: "json" };
+export * from "./test.json"; /* with */
+assert; /* with */
+{
+  type: "json";
+}

```

**Prettier Similarity**: 9.09%


### js/import-assertions/multi-types.js
```diff
-import json from "./foo.json" assert { type: "json", type: "bar" };
+import json from "./foo.json";
+assert;
+{
+  type: "json", type;
+  : "bar"
+}

```

**Prettier Similarity**: 0.00%


### js/import-assertions/non-type.js
```diff
-import foo from "foo.json" assert { lazy: "true" };
+import foo from "foo.json";
+assert;
+{
+  lazy: "true";
+}

```

**Prettier Similarity**: 0.00%


### js/import-assertions/re-export.js
```diff
-export { default as foo2 } from "foo.json" assert { type: "json" };
-export * from "foo.json" assert { type: "json" };
-export * as foo3 from "foo.json" assert { type: "json" };
+export { default as foo2 } from "foo.json";
+assert;
+{
+  type: "json";
+}
+export * from "foo.json";
+assert;
+{
+  type: "json";
+}
+export * as foo3 from "foo.json";
+assert;
+{
+  type: "json";
+}

```

**Prettier Similarity**: 0.00%


### js/import-assertions/static-import.js
```diff
-import json from "./foo.json" assert { type: "json" };
+import json from "./foo.json";
+assert;
+{
+  type: "json";
+}

```

**Prettier Similarity**: 0.00%


### js/import-assertions/without-from.js
```diff
-import "foo" assert { type: "json" };
+import "foo";
+assert;
+{
+  type: "json";
+}

```

**Prettier Similarity**: 0.00%


### js/import-attributes/bracket-spacing/empty.js
```diff
-export * as bar from "bar.json" with {};
+export * as bar from "bar.json";

```

**Prettier Similarity**: 0.00%


### js/import-attributes/empty.js
```diff
 export * as foo from "foo.json";
-export * as bar from "bar.json" with {};
-export * as baz from "baz.json" /* comment */ with {};
+export * as bar from "bar.json";
+export * as baz from "baz.json" /* comment */;
 
 import * as foo from "foo.json";
-import * as bar from "bar.json" with {};
-import * as baz from "baz.json" /* comment */ with {};
+import * as bar from "bar.json";
+import * as baz from "baz.json" /* comment */;

```

**Prettier Similarity**: 42.86%


### js/import-attributes/keyword-detect.js
```diff
-import "./test.json" /* assert */ /* assert */ with { type: "json" };
-import a from "./test.json" /* assert */ /* assert */ with { type: "json" };
+import "./test.json" /* assert */ with { /* assert */ type: "json" };
+import a from "./test.json" /* assert */ with { /* assert */ type: "json" };
 
-export { default as c } from "./test.json" /* assert */ /* assert */ with { type: "json" };
+export { default as c } from "./test.json" /* assert */ with {
+  /* assert */ type: "json",
+};
 
-export * from "./test.json" /* assert */ /* assert */ with { type: "json" };
+export * from "./test.json" /* assert */ with { /* assert */ type: "json" };

```

**Prettier Similarity**: 25.00%


### js/import-attributes/long-sources.js
```diff
 import a11 from "./aaaaaaaaaa.json" with { type: "json" };
 import a12 from "./aaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
 import a13 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
-import a14 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
-import a15 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
-import a16 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
-import a17 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
-import a18 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
+import a14 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  type: "json",
+};
+import a15 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  type: "json",
+};
+import a16 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  type: "json",
+};
+import a17 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  type: "json",
+};
+import a18 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  type: "json",
+};
 
-import a21 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { // comment
-type: "json" };
-import a22 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type:
+import a21 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  // comment
+  type: "json",
+};
+import a22 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
   // comment
-  "json" };
-import a23 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" }; // comment
+  type: "json",
+};
+import a23 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  type: "json", // comment
+};
 
-import a31 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { /* comment */
-type: "json" };
-import a32 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type:
+import a31 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  /* comment */
+  type: "json",
+};
+import a32 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
   /* comment */
-  "json" };
-import a33 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" /* comment */ };
+  type: "json",
+};
+import a33 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  type: "json" /* comment */,
+};
 
 import("./aaaaaaaaaa.json", { with: { type: "json" } });
 import("./aaaaaaaaaaaaaaaaaaaa.json", { with: { type: "json" } });
 import("./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json", { with: { type: "json" } });
 import("./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json", {
   with: { type: "json" },
 });
 import("./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json", {
   with: { type: "json" },
 });
 import("./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json", {
   with: { type: "json" },
 });
 import(
   "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
   { with: { type: "json" } }
 );
 import(
   "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
   { with: { type: "json" } }
 );

```

**Prettier Similarity**: 44.44%


### js/import-attributes/multiple.js
```diff
-import syntaxImportAssertions from "@babel/plugin-syntax-import-assertions" with { BABEL_8_BREAKING: "false", USE_ESM: "true", IS_STANDALONE: "false" };
+import syntaxImportAssertions from "@babel/plugin-syntax-import-assertions" with {
+  BABEL_8_BREAKING: "false",
+  USE_ESM: "true",
+  IS_STANDALONE: "false",
+};
 
-import a1 from "foo" with { BABEL_8_BREAKING: "false", USE_ESM: "true", IS_STANDALONE: "false" };
-import a2 from "foo" with { BABEL_8_BREAKING: "false", USE_ESM: "true", IS_STANDALONE: "false" };
+import a1 from "foo" with {
+  BABEL_8_BREAKING: "false",
+  USE_ESM: "true",
+  IS_STANDALONE: "false",
+};
+import a2 from "foo" with {
+  BABEL_8_BREAKING: "false",
+  USE_ESM: "true",
+  IS_STANDALONE: "false",
+};
 import a3 from "foo" with { BABEL_8_BREAKING: "false" };
 import a4 from "foo" with { BABEL_8_BREAKING: "false" };

```

**Prettier Similarity**: 16.67%


### js/import/empty-import.js
```diff
 import {} from "@types/googlemaps";
 import "a";
 import /* comment */ "a";
 import // comment
 "a";
+import {/* comment */} from "a";
+import /* comment */ {} from "a";
+import {} /* comment */ from "a";
 import {} from /* comment */ "a";
-import {} from /* comment */ "a";
-import {} from /* comment */ "a";
-import {} from /* comment */ "a";
-import {} from /* comment */ /* comment */ /* comment */ /* comment */ "a";
+import /* comment */ {/* comment */} /* comment */ from /* comment */ "a";
+import {
+  // comment
+} from "a";
+import // comment
+{} from "a";
 import {} from // comment
 "a";
 import {} from // comment
 "a";
-import {} from // comment
-"a";
-import {} from // comment
-"a";
-import {} from // comment
-// comment
-// comment
+import // comment
+{
+  // comment
+} from // comment
 // comment
 "a";
 
 import // {} from
 "a";
 import {} from // comment ends with from
 "a";
 import {} from /* comment ends with from */ "a";
 import {} from // comment not ends with from ___
 "a";
 import {} from /* comment not ends with from ___ */ "a";
 
 import // comment ends with from
 "a";
 import /* comment ends with from */ "a";
 import // comment not ends with from ___
 "a";
 import /* comment not ends with from ___ */ "a";

```

**Prettier Similarity**: 68.29%


### js/last-argument-expansion/dangling-comment-in-arrow-function.js
```diff
-foo(() =>
-  // foo
-  {},
+foo(
+  (
+    // foo
+  ) => {},
 );

```

**Prettier Similarity**: 20.00%


### js/last-argument-expansion/issue-18143.js
```diff
 assert.deepStrictEqual(
   linesCollection.getViewLinesIndentGuides____(-1, -1),
   [],
 );
-assert.deepStrictEqual(linesCollection.getViewLinesIndentGuides___(-1, -1), [
-  1,
-]);
 assert.deepStrictEqual(
+  linesCollection.getViewLinesIndentGuides___(-1, -1),
+  [1],
+);
+assert.deepStrictEqual(
   linesCollection.getViewLinesIndentGuides(-1, -1),
   [1, 2],
 );

```

**Prettier Similarity**: 66.67%


### js/logical-assignment/inside-call/18171.js
```diff
 fn(
   _,
   glimseGlyphsHazardNoopsTieTie === averredBathersBoxroomBuggyNurl &&
     // Should this line indent?
     anodyneCondosMalateOverateRetinol
     ? annularCooeedSplicesWalksWayWay
     : kochabCooieGameOnOboleUnweave,
 );
 
 new fn(
   _,
   glimseGlyphsHazardNoopsTieTie === averredBathersBoxroomBuggyNurl &&
-  // Should this line indent?
-  anodyneCondosMalateOverateRetinol
+    // Should this line indent?
+    anodyneCondosMalateOverateRetinol
     ? annularCooeedSplicesWalksWayWay
     : kochabCooieGameOnOboleUnweave,
 );
 
 fn(
   glimseGlyphsHazardNoopsTieTie === averredBathersBoxroomBuggyNurl &&
     // Should this line indent?
     anodyneCondosMalateOverateRetinol
     ? annularCooeedSplicesWalksWayWay
     : kochabCooieGameOnOboleUnweave,
 );
 
 new fn(
   glimseGlyphsHazardNoopsTieTie === averredBathersBoxroomBuggyNurl &&
-  // Should this line indent?
-  anodyneCondosMalateOverateRetinol
+    // Should this line indent?
+    anodyneCondosMalateOverateRetinol
     ? annularCooeedSplicesWalksWayWay
     : kochabCooieGameOnOboleUnweave,
 );
 
 // https://github.com/typescript-eslint/typescript-eslint/blob/ea2ee6b65a2f14dd2c3fc8d12be969cbeaef80a8/packages/typescript-estree/src/parseSettings/resolveProjectList.ts#L75C1-L80C7
 {
   {
     RESOLUTION_CACHE = new ExpiringCache(
       options.singleRun
         ? "Infinity"
         : (options.cacheLifetime?.glob ??
-          DEFAULT_TSCONFIG_CACHE_DURATION_SECONDS),
+            DEFAULT_TSCONFIG_CACHE_DURATION_SECONDS),
     );
     RESOLUTION_CACHE = new_ExpiringCache(
       options.singleRun
         ? "Infinity"
         : (options.cacheLifetime?.glob ??
             DEFAULT_TSCONFIG_CACHE_DURATION_SECONDS),
     );
   }
 }

```

**Prettier Similarity**: 90.20%


### js/logical-expressions/multiple-comments/17192.js
```diff
-ErrorLike = SerializedProps &&
-  // cause is a new addition to Error that is not yet available in all runtimes. We have added
-  // it to try and pinpoint additional reasoning for failures such as Node's fetch.
-  // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause
-  { cause: unknown };
+// FIXME
+// TODO: reformat issue
+// ErrorLike =
+//   SerializedProps &&
+//   // cause is a new addition to Error that is not yet available in all runtimes. We have added
+//   // it to try and pinpoint additional reasoning for failures such as Node's fetch.
+//   // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause
+//   { cause: unknown };
 
-ErrorLike = SerializedProps && // cause is a new addition to Error that is not yet available in all runtimes. We have added
-  // it to try and pinpoint additional reasoning for failures such as Node's fetch.
-  // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause
-  { cause: unknown };
+// ErrorLike =
+//   SerializedProps && // cause is a new addition to Error that is not yet available in all runtimes. We have added
+//   // it to try and pinpoint additional reasoning for failures such as Node's fetch.
+//   // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause
+//   { cause: unknown };

```

**Prettier Similarity**: 7.14%


### js/no-semi/issue2006.js
```diff
 switch (n) {
   case 11:
     var c = a.e;
-    ((i.a += Ga(c.e)), F(i, c.i, 0));
+    (i.a += Ga(c.e)), F(i, c.i, 0);
 }
 
 var c = a.e;
-((i.a += Ga(c.e)), F(i, c.i, 0));
+(i.a += Ga(c.e)), F(i, c.i, 0);

```

**Prettier Similarity**: 75.00%


### js/no-semi/no-semi.js
```diff
 // with preexisting semi
 
 x;
 [1, 2, 3].forEach(fn);
 x;
 [a, b, ...c] = [1, 2];
 x;
 /r/i.test("r");
 x;
 +1;
 x;
 -1;
 x;
 ("h" + "i").repeat(10);
 x;
-(1, 2);
+1, 2;
 x;
 (() => {})();
 x;
 ({ a: 1 }).entries();
 x;
 ({ a: 1 }).entries();
 x;
 <Hello />;
 x;
 `string`;
 x;
 (x, y) => x;
 
 // doesn't have to be preceded by a semicolon
 
 class X {}
 [1, 2, 3].forEach(fn);
 
 // don't semicolon if it doesn't start statement
 
 if (true) (() => {})();
 
 // check indentation
 
 if (true) {
   x;
   (() => {})();
 }
 
 // check statement clauses
 
 do break;
 while (false);
 if (true)
   do break;
   while (false);
 
 if (true) 1;
 else 2;
 for (;;);
 for (x of y);
 
 debugger;
 
 // check that it doesn't break non-ASI
 
 1 - 1;
 
 1 + 1;
 
 1 / 1;
 
 arr[0];
 
 fn(x);
 
 !1;
 
 1 < 1;
 
 tag`string`;
 
 x;
 (x) => x;
 
 x;
 (a || b).c++;
 
 x;
 ++(a || b).c;
 
 while (false) (function () {})();
 
 aReallyLongLine012345678901234567890123456789012345678901234567890123456789 *
   (b + c);

```

**Prettier Similarity**: 98.90%


### js/preserve-line/member-chain.js
```diff
 fooBar
   .doSomething("Hello World")
   .doAnotherThing("Foo", { foo: bar })
 
   // App configuration.
   .doOneMoreThing(config)
 
   .run(() => console.log("Bar"));
 
 bigDeal
 
   .doSomething("Hello World")
 
   // Hello world
   .doAnotherThing("Foo", { foo: bar })
 
   // App configuration.
   .doOneMoreThing(config)
 
   .run(() => console.log("Bar"));
 
 foo.bar.baz
 
   .doSomething("Hello World")
 
   // Hello world
   .foo.bar.doAnotherThing("Foo", { foo: bar })
 
   .doOneMoreThing(config)
   .bar.run(() => console.log("Bar"));
 
 (somethingGood ? thisIsIt : maybeNot)
 
   // Hello world
   .doSomething("Hello World")
 
   .doAnotherThing("Foo", { foo: bar }) // Run this
   .run(() => console.log("Bar")); // Do this
 
 helloWorld
 
   .text()
 
   .then((t) => t);
 
 (
   veryLongVeryLongVeryLong ||
   anotherVeryLongVeryLongVeryLong ||
   veryVeryVeryLongError
 )
 
   .map((tickets) => TicketRecord.createFromSomeLongString())
 
   .filter((obj) => !!obj);
 
 const sel = this.connections
 
   .concat(this.activities.concat(this.operators))
   .filter((x) => x.selected);
 
-Object.entries(obj)
+Object.entries(obj).forEach((e) => console.log(e));
 
-  .forEach((e) => console.log(e));
-
-this.fetch("/foo")
-
-  .then((response) => response.json());
+this.fetch("/foo").then((response) => response.json());

```

**Prettier Similarity**: 91.04%


### js/quotes/objects.js
```diff
 const obj = {
   a: true,
   b: true,
-  "𐊧": true,
+  𐊧: true,
 };

```

**Prettier Similarity**: 80.00%


### js/range/boundary-2.js
```diff
 function a(
 ){
-  a();
-  b();
-  c();
-  d();
+a (
+);
+b();
+c(); d(
+);
+
 }

```

**Prettier Similarity**: 33.33%


### js/range/boundary-3.js
```diff
 a (
 );
-b (
-);                 c (
-); d(
+b();
+c(); d(
 );

```

**Prettier Similarity**: 50.00%


### js/range/boundary.js
```diff
-foo = 1.0000;bar = 1.0;
-baz = 1.0;
+foo = 1.0000;bar = 1.0;baz=1.0000;
 // The range will be 13~26
 // `foo` ends at 13, should not format
 // `bar` ends at 26, should format

```

**Prettier Similarity**: 60.00%


### js/range/class-declaration.js
```diff
+
+
 class a {
   b() {}
 }
 
-let x;
+let    x

```

**Prettier Similarity**: 57.14%


### js/range/multiple-statements2.js
```diff
 call(
   1, 2,3
 );
 
 call(1, 2, 3);
 
 call(1, 2, 3);
 
-call(1, 2, 3);
+call(
+  1, 2,3
+);

```

**Prettier Similarity**: 72.73%


### js/range/nested3.js
```diff
 try {
-  1;
-  if (condition) {
-    body;
-  }
+1;
+if (condition) {
+  body;
+}
 }
 catch (err) {}

```

**Prettier Similarity**: 42.86%


### js/range/whitespace.js
```diff
- 

```

**Prettier Similarity**: 0.00%


### js/reserved-word/interfaces.js
```diff
 foo.interface;
 interface.foo;
 new interface();
 ({ interface: "foo" });
-(interface, "foo");
+interface, "foo";
 void interface;
 var interface = "foo";

```

**Prettier Similarity**: 85.71%


### js/reserved-word/let.js
```diff
 foo.let;
 let.foo;
 new let();
 ({ let: "foo" });
-(let, "foo");
+let, "foo";
 void let;
 var let = "foo";

```

**Prettier Similarity**: 85.71%


### js/reserved-word/yield.js
```diff
 foo.yield;
 yield.foo;
 new yield();
 ({ yield: "foo" });
-(yield, "foo");
+yield, "foo";
 void yield;
 var yield = "foo";

```

**Prettier Similarity**: 85.71%


### js/return-outside-function/return-outside-function.js
```diff
-return (
-  someVeryLongStringA &&
-  someVeryLongStringB &&
-  someVeryLongStringC &&
-  someVeryLongStringD
-);
+return someVeryLongStringA && someVeryLongStringB && someVeryLongStringC && someVeryLongStringD

```

**Prettier Similarity**: 0.00%


### js/sequence-break/break.js
```diff
 const f = (argument1, argument2, argument3) => (
   doSomethingWithArgument(argument1),
   doSomethingWithArgument(argument2),
   argument1
 );
 (function () {
   return (
     aLongIdentifierName,
     aLongIdentifierName,
     aLongIdentifierName,
     aLongIdentifierName
   );
 });
 (function () {
   throw (
-    aLongIdentifierName,
+    (aLongIdentifierName,
     aLongIdentifierName,
     aLongIdentifierName,
-    aLongIdentifierName
+    aLongIdentifierName)
   );
 });
-(aLongIdentifierName,
+aLongIdentifierName,
   aLongIdentifierName,
   aLongIdentifierName,
-  aLongIdentifierName);
+  aLongIdentifierName;
 a.then(
   () => (
     aLongIdentifierName,
     aLongIdentifierName,
     aLongIdentifierName,
     aLongIdentifierName
   ),
 );
 for (
   aLongIdentifierName = 0,
     aLongIdentifierName = 0,
     aLongIdentifierName = 0,
     aLongIdentifierName = 0;
   test;
   update
 ) {}
-((a = b
+(a = b
   ? c
   : function () {
       return 0;
     }),
   (a = b
     ? c
     : function () {
         return 0;
       }),
   (a = b
     ? c
     : function () {
         return 0;
       }),
   (a = b
     ? c
     : function () {
         return 0;
       }),
   (a = b
     ? c
     : function () {
         return 0;
-      }));
+      });

```

**Prettier Similarity**: 90.91%


### js/sequence-expression/expression.js
```diff
-(a, b);
+a, b;
 
-(a, b);
+a, b;

```

**Prettier Similarity**: 33.33%


### js/sequence-expression/ignore.js
```diff
-+(
-  // prettier-ignore
-  ((
++// prettier-ignore
+(
+    (
       first
     )
     ,
     (
       last
-    ))
-);
+    )
+  );

```

**Prettier Similarity**: 50.00%


### js/sequence-expression/ignored.js
```diff
-const a = () => () =>
-  // prettier-ignore
-  (a,b);
+// FIXME
+// TODO: reformat issue
+// const a = ()=>()=>
+// // prettier-ignore
+// (a,b)

```

**Prettier Similarity**: 0.00%


### js/sequence-expression/no-semi/expression.js
```diff
 a;
-(+1, b);
++1, b;

```

**Prettier Similarity**: 50.00%


### js/sequence-expression/parenthesized.js
```diff
 console.log(
   /* 1 */
+
   /* 2 */
+
   /* 3 */
   (first,
   /* 4 */
   /* 5 */
   /* 6 */
+
   /* 7 */
   last),
   /* 8 */
   /* 9 */
   /* 10 */
 );

```

**Prettier Similarity**: 82.35%


### js/sequence-expression/return.js
```diff
 function a() {
-  return (a, b);
-  return (a, b);
+  return a, b;
+  return a, b;
 }

```

**Prettier Similarity**: 50.00%


### js/sloppy-mode/function-declaration-in-while.js
```diff
-while (false) function foo() {}
+while (false) function foo(){}

```

**Prettier Similarity**: 0.00%


### js/strings/non-octal-eight-and-nine.js
```diff
 // https://github.com/babel/babel/pull/11852
 
-("\8", "\9");
+"\8", "\9";
 () => {
   "use strict";
-  ("\8", "\9");
+  "\8", "\9";
 };

```

**Prettier Similarity**: 71.43%


### js/template-literals/expressions.js
```diff
 const long1 = `long ${
   a.b //comment
 } long longlong ${a.b.c.d.e} long longlong ${a.b.c.d.e} long longlong ${a.b.c.d.e} long long`;
 const long2 = `long ${a.b.c.d.e} long longlong ${loooooooooooooooooong} long longlong ${loooooooooooooooooong} long longlong ${loooooooooooooooooong} long long`;
 
 const long3 = `long long long long long long long long long long long ${a.b.c.d.e} long long long long long long long long long long long long long`;
 
 const description = `The value of the ${cssName} css of the ${this._name} element`;
 
 const foo = `such a long template string ${foo.bar.baz} that prettier will want to wrap it`;
 
 const shouldWrapForNow = `such a long template string ${foo().bar.baz} that prettier will want to wrap it`;
 
 const shouldNotWrap = `simple expressions should not break ${this} ${variable} ${a.b.c} ${this.b.c} ${a[b].c} ${a.b[c]} ${a.b["c"]} ${a?.b?.c}`;
 
 console.log(
   chalk.white(
     `Covered Lines below threshold: ${coverageSettings.lines}%. Actual: ${coverageSummary.total.lines.pct}%`,
   ),
 );
 
 x = `mdl-textfield mdl-js-textfield ${className} ${
   content.length > 0 ? "is-dirty" : ""
 } combo-box__input`;
 
 function testing() {
   const p = {};
   // faking some tabs since I can't paste my real code in
   if (true) {
     if (false) {
       return `${process.env.OPENID_URL}/something/something/something?${Object.keys(
         p,
       )
         .map((k) => `${encodeURIComponent(k)}=${encodeURIComponent(p[k])}`)
         .join("&")}`;
     }
   }
 }
 
 console.log(
   `Trying update appcast for ${app.name} (${app.cask.appcast}) -> (${app.cask.appcastGenerated})`,
 );
 
 console.log(
   `brew cask audit --download ${_.map(definitions, "caskName").join(" ")}`,
 );
 
 console.log(
   `\nApparently jetbrains changed the release artifact for ${app.name}@${app.jetbrains.version}.\n`,
 );
 
 descirbe("something", () => {
   test(`{pass: false} expect(${small}).toBeGreaterThanOrEqual(${big})`, () => {});
 });
 
 throw new Error(
   `pretty-format: Option "theme" has a key "${key}" whose value "${value}" is undefined in ansi-styles.`,
 );
 
-a = `${[
-  [1, 2, 3],
-  [4, 5, 6],
-]}`;
+a = `${[[1, 2, 3], [4, 5, 6]]}`;

```

**Prettier Similarity**: 93.65%


### js/ternaries/parenthesis/await-expression.js
```diff
-stopDirectory = await (
-  useCache ? memoizedFindProjectRoot : findProjectRootWithoutCache
-)(path.dirname(path.resolve(filePath)));
+stopDirectory = await (useCache
+  ? memoizedFindProjectRoot
+  : findProjectRootWithoutCache)(path.dirname(path.resolve(filePath)));

```

**Prettier Similarity**: 0.00%


### js/test-declarations/angularjs_inject.js
```diff
 beforeEach(inject(($fooService, $barService) => {
   // code
 }));
 
 afterAll(inject(($fooService, $barService) => {
   console.log("Hello");
 }));
 
 it("should create the app", inject(($fooService, $barService) => {
   //code
 }));
 
 it("does something really long and complicated so I have to write a very long name for the test", inject(() => {
   // code
 }));
 
-it("does something really long and complicated so I have to write a very long name for the test", inject((
-  $fooServiceLongName,
-  $barServiceLongName,
-) => {
+it("does something really long and complicated so I have to write a very long name for the test", inject(($fooServiceLongName, $barServiceLongName) => {
   // code
 }));
 
 /*
  * isTestCall(parent) should only be called when parent exists
  * and parent.type is CallExpression. This test makes sure that
  * no errors are thrown when calling isTestCall(parent)
  */
 function x() {
   inject(() => {});
 }

```

**Prettier Similarity**: 87.10%


### js/test-declarations/optional.js
```diff
-describe?.(
-  "some string some string some string some string some string some string some string some string",
-  (done) => {},
-);
+describe?.("some string some string some string some string some string some string some string some string", (done) => {});

```

**Prettier Similarity**: 0.00%


### jsx/comments/in-attributes.js
```diff
 <div attr=/* comment */ "foo"></div>;
 
 <div attr=/* comment */
 "foo"></div>;
 
-<div attr /* comment */="foo"></div>;
+<div attr=/* comment */
+"foo"></div>;
 
 <div attr=/* comment */ "foo"></div>;
 
 <div attr=// comment
 "foo"></div>;
 
-<div attr="foo"></div>; // comment
+<div attr=// comment
+"foo"></div>;

```

**Prettier Similarity**: 73.33%


### jsx/comments/in-end-tag.js
```diff
 /* =========== before slash =========== */
 <a></
   // line
   a
 >;
 <a></ /* block */
 a>;
 
 <></
   // line
 >;
 <></ /* block */>;
 
 /* =========== after slash =========== */
 <a></
   // line
   a
 >;
 <a></ /* block */
 a>;
 
 <></
   // line
 >;
 <></ /* block */>;
 
 /* =========== after name =========== */
-<a></a>; // line
+<a></a // line
+>;
 <a></a /* block */>;
 
 /* =========== block =========== */
 <a></a /* block */>;
 <></ /* block */>;
 
 /* =========== multiple ===========  */
 <a></
   // line 1
   // line 2
   a
 >;
 <a></ /* block1 */ /* block2 */
 a>;
 <a></
   /* block */ // line
   a
 >;
 
 <></
   // line 1
   // line 2
 >;
 <></ /* block1 */
   /* block2 */>;
 <></
   /* block */
   // line
 >;

```

**Prettier Similarity**: 96.55%


### jsx/fbt/test.js
```diff
 x = (
   <fbt>
-    <fbt:param>First</fbt:param>,
-    <fbt:param>Second</fbt:param>
+    <fbt:param>First</fbt:param>,<fbt:param>Second</fbt:param>
   </fbt>
 );
 
 x = (
   <fbt>
-    <fbt:param>First</fbt:param>
-    ,
-    <fbt:param>Second</fbt:param>
+    <fbt:param>First</fbt:param>,<fbt:param>Second</fbt:param>
   </fbt>
 );
 
 x = (
   <fbt>
     <fbt:param>First</fbt:param>,<fbt:param>Second</fbt:param>
   </fbt>
 );
 
 x = (
   <fbt>
     <fbt:param>First</fbt:param>,<fbt:param>Second</fbt:param>
   </fbt>
 );
 
 x = (
   <fbt desc="example 1">
     Prefix comes before
     <fbt:param>
       <b>suffix</b>
     </fbt:param>
   </fbt>
 );
 
 x = (
   <fbt desc="example 2">
     Prefix comes before
     <fbt:param name="bold stuff">
       <b>suffix</b>
     </fbt:param>
     <fbt:param name="a link">
       <link href="#">suffix</link>
     </fbt:param>
   </fbt>
 );
 
 x = (
   <fbt desc="example 3">
     Count Chocula knows the the number
     <fbt:enum enum-range={["one", "two", "three"]} value={getValue()} />
     is awesome
   </fbt>
 );
 
 x = (
   <fbt>
     {hour}:{minute}:{second}
   </fbt>
 );
 
 x = (
   <fbt>
-    {hour}
-    :
-    {minute}
-    :
-    {second}
+    {hour}:{minute}:{second}
   </fbt>
 );
 
 x = (
   <fbt>
-    {hour}:
-    {minute}:
-    {second}
+    {hour}:{minute}:{second}
   </fbt>
 );
 
 first = (
   <fbt>
-    Text<br />
-    More text<br />
-    And more<br />
+    Text
+    <br />
+    More text
+    <br />
+    And more
+    <br />
   </fbt>
 );
 
 second = (
   <fbt>
-    Text<br />More text<br />And more<br />
+    Text
+    <br />
+    More text
+    <br />
+    And more
+    <br />
   </fbt>
 );
 
 third = (
   <fbt>
     Text
     <br />
     More text
     <br />
     And more
     <br />
   </fbt>
 );

```

**Prettier Similarity**: 83.65%


### jsx/jsx/await.js
```diff
 async function testFunction() {
   const short = (
     <>
       {await Promise.all(hierarchyCriticism)}
       {await hierarchyCriticism.ic.me.oa.p}
       {await hierarchyCriticism}
 
       {Promise.all(hierarchyCriticism)}
       {hierarchyCriticism.ic.me.oa.p}
       {hierarchyCriticism}
     </>
   );
 
   const long = (
     <>
-      {await Promise.all(
-        hierarchyCriticismIncongruousCooperateMaterialEducationOriginalArticulateParameter,
-      )}
       {
+        await Promise.all(
+          hierarchyCriticismIncongruousCooperateMaterialEducationOriginalArticulateParameter,
+        )
+      }
+      {
         await hierarchyCriticism.IncongruousCooperate.MaterialEducation
           .OriginalArticulate.Parameter
       }
       {
         await hierarchyCriticismIncongruousCooperateMaterialEducationOriginalArticulateParameter
       }
 
       {Promise.all(
         hierarchyCriticismIncongruousCooperateMaterialEducationOriginalArticulateParameter,
       )}
       {
         hierarchyCriticism.IncongruousCooperate.MaterialEducation
           .OriginalArticulate.Parameter
       }
       {
         hierarchyCriticismIncongruousCooperateMaterialEducationOriginalArticulateParameter
       }
     </>
   );
 
   const jsx = (
     <>
-      {await (
-        <IncongruousCooperate>
-          material education original articulate parameter
-        </IncongruousCooperate>
-      )}
+      {
+        await (
+          <IncongruousCooperate>
+            material education original articulate parameter
+          </IncongruousCooperate>
+        )
+      }
     </>
   );
 }

```

**Prettier Similarity**: 77.36%


### jsx/jsx/quotes.js
```diff
 <div id="&quot;'<>&amp;quot;" />;
 <div id='"&#39;<>&amp;quot;' />;
 <div id={"'\"&quot;<>&amp;quot;"} />;
 <div id="123" />;
-<div id='&#39;"' />;
+<div id="&#39;&quot;" />;
 <div id={"'\"\\'"} />;
 <div
   single="foo"
   single2={"foo"}
   double="bar"
   double2={"bar"}
   singleDouble='"'
   singleDouble2={'"'}
   doubleSingle="'"
   doubleSingle2={"'"}
   singleEscaped={"'"}
-  singleEscaped2="'"
+  singleEscaped2="&apos;"
   doubleEscaped={'"'}
-  doubleEscaped2='"'
+  doubleEscaped2="&quot;"
   singleBothEscaped={"'\""}
-  singleBothEscaped2="'&quot;"
-  singleBoth="' &quot;"
+  singleBothEscaped2='&apos;"'
+  singleBoth='&apos; "'
   singleBoth2={"' \""}
-  singleBoth3="' ' &quot;"
+  singleBoth3='&apos; &apos; "'
   doubleBoth="&quot; '"
   doubleBoth2={"\" '"}
-  doubleBoth3="&quot; ' '"
+  doubleBoth3="&quot; &apos; '"
 />;
 
 <p>
   GitHub Desktop has encountered an unrecoverable error and will need to 1231231
   restart. This has been reported to the team, but if youencounter this121312331
   repeatedly please report this issue to the GitHub 12312312312312313{"  "}{" "}
 </p>;

```

**Prettier Similarity**: 79.41%


### jsx/jsx/regex.js
```diff
-((x = <div>one</div>), (<div>two</div>));
+(x = <div>one</div>), (<div>two</div>);
 x = <a>{}</a>;
 x = <a>{1 / 2}</a>;
 x = <a>{/w/.test(s)}</a>;

```

**Prettier Similarity**: 75.00%


### jsx/text-wrap/test.js
```diff
 // Wrapping text
 x = (
   <div>
     Some text that would need to wrap on to a new line in order to display
     correctly and nicely
   </div>
 );
 
 // Wrapping tags
 x = (
   <div>
     <first>f</first> <first>f</first> <first>f</first> <first>f</first>{" "}
     <first>f</first> <first>f</first>
   </div>
 );
 
 // Wrapping tags
 x = (
   <div>
     <first>f</first>
     <first>f</first>
     <first>f</first>
     <first>f</first>
     <first>f</first>
     <first>f</first>
   </div>
 );
 
 // Wrapping tags
 x = (
   <div>
     <a />
     <b />
     <c />
     <first>aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa</first>{" "}
     <first>f</first>
   </div>
 );
 
 // Wrapping tags
 x = (
   <div>
     <sadashdkjahsdkjhaskjdhaksjdhkashdkashdkasjhdkajshdkashdkashd />{" "}
     <first>f</first>
   </div>
 );
 
 x = (
   <div>
     before
     <div>
       Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur at
       mollis lorem.
     </div>
     after
   </div>
 );
 
 x = (
   <div>
     before{stuff}after{stuff}after{stuff}after{stuff}after{stuff}after{stuff}
     {stuff}
     {stuff}after{stuff}after
   </div>
 );
 
 x = (
   <div>
     before {stuff} after {stuff} after {stuff} after {stuff} after {stuff} after{" "}
     {stuff} {stuff} {stuff} after {stuff} after
   </div>
 );
 
 x = (
   <div>
     Please state your <b>name</b> and <b>occupation</b> for the board of{" "}
     <b>school</b> directors.
   </div>
 );
 
 function DiffOverview(props) {
   const { source, target, since } = props;
   return (
     <div>
       <div className="alert alert-info">
         <p>
           This diff overview is computed against the current list of records in
           this collection and the list it contained on <b>{humanDate(since)}</b>
           .
         </p>
         <p>
           <b>Note:</b> <code>last_modified</code> and <code>schema</code> record
           metadata are omitted for easier review.
         </p>
       </div>
       <Diff source={source} target={target} />
     </div>
   );
 }
 
 x = (
   <font size={-3}>
     <i>
       Starting at minute {graphActivity.startTime}, running for{" "}
       {graphActivity.length} to minute{" "}
       {graphActivity.startTime + graphActivity.length}
     </i>
   </font>
 );
 
 x = (
   <div>
     First second third
     <div attr="a very long string attribute that will overflow because it is very long">
       Something
     </div>
   </div>
 );
 
 x = (
   <div>
     <div>First</div>
     Second
     <div>Third</div>
   </div>
 );
 
 x = (
   <div>
     First <div>Second</div> Third
   </div>
 );
 
 leading_whitespace = (
   <div>
     {" "}
     First Second Third Fourth Fifth Sixth Seventh Eighth Ninth Tenth Eleventh
     Twelfth Thirteenth Fourteenth
   </div>
 );
 
 trailing_whitespace = (
   <div>
     First Second Third Fourth Fifth Sixth Seventh Eighth Ninth Tenth Eleventh
     Twelfth Thirteenth Fourteenth{" "}
   </div>
 );
 
 no_leading_or_trailing_whitespace = (
   <div>
     First Second Third Fourth Fifth Sixth Seventh Eighth Ninth Tenth Eleventh
     Twelfth Thirteenth Fourteenth
   </div>
 );
 
 facebook_translation_leave_text_around_tag = (
   <div>
     <span>First</span>, (<span>Second</span>)
   </div>
 );
 
 x = (
   <div>
     <span>First second third fourth fifth sixth seventh</span>, (
     <span>Second</span>)
   </div>
 );
 
 this_really_should_split_across_lines = (
   <div>
     before{stuff}after{stuff}after{stuff}after{stuff}after{stuff}after{stuff}
     after{stuff}after{stuff}after{stuff}after{stuff}after{stuff}after{stuff}
     after{stuff}after{stuff}after
   </div>
 );
 
 unstable_before = (
   <div className="yourScore">
     Your score:{" "}
     <span className="score">{`${mini.crosstable.users[sessionUserId]} - ${mini.crosstable.users[user.id]}`}</span>
   </div>
 );
 
 unstable_after_first_run = (
   <div className="yourScore">
     Your score:{" "}
     <span className="score">{`${mini.crosstable.users[sessionUserId]} - ${
       mini.crosstable.users[user.id]
     }`}</span>
   </div>
 );
 
 solitary_whitespace = (
   <div
     first="first"
     second="second"
     third="third"
     fourth="fourth"
     fifth="fifth"
     sixth="sixth"
   >
     {" "}
   </div>
 );
 
 jsx_whitespace_on_newline = (
   <div>
     <div>First</div> <div>Second</div> <div>Third</div>
   </div>
 );
 
 jsx_around_multiline_element = (
   <div>
     Before{" "}
     <div>
       {
         "Enough text to make this element wrap on to multiple lines when formatting"
       }
     </div>{" "}
     After
   </div>
 );
 
 jsx_around_multiline_element_second_pass = (
   <div>
     Before{" "}
     <div>
       {
         "Enough text to make this element wrap on to multiple lines when formatting"
       }
     </div>{" "}
     After
   </div>
 );
 
 convert_space_expressions = <div> </div>;
 
 x = (
   <div>
     <first />
     <second />
     <third />
     <fourth />
     <fifth />
     <sixth />
   </div>
 );
 
 const Abc = () => {
   return (
     <div>
       Please state your <b>name</b> and <b>occupation</b> for the board of
       directors.
     </div>
   );
 };
 
 x = <div id="moo">Some stuff here</div>;
 
 headers_and_paragraphs = (
   <div>
     <h2>First</h2>
     <p>The first paragraph.</p>
 
     <h2>Second</h2>
     <p>The second paragraph.</p>
   </div>
 );
 
 no_text_one_tag_per_line = (
   <div>
     <first />
     <second />
   </div>
 );
 
 with_text_fill_line = (
   <div>
     Text <first />
     <second />
   </div>
 );
 
 line_after_br = (
   <div>
     Text
     <br />
     More text
     <br />
     And more
     <br />
   </div>
 );
 
 line_after_br = (
   <div>
     Text
     <br />
     More text
     <br />
     And more
     <br />
   </div>
 );
 
 line_after_br = (
   <div>
     Text
     <br />
     More text
     <br />
     And more
     <br />
   </div>
 );
 
 line_after_br_2 = (
   <div>
     A<br />B<br />C
   </div>
 );
 
 br_followed_by_whitespace = (
   <div>
     <br /> text
   </div>
 );
 
 dont_preserve_blank_lines_when_jsx_contains_text = (
   <div>
     <div>Zeroth</div>
     <div>First</div>
     Second
   </div>
 );
 
 multiple_expressions = (
   <div>
     {header}
     {body}
     {footer}
   </div>
 );
 
 single_expression_child_tags = (
   <div>
     You currently have <strong>{dashboardStr}</strong> and{" "}
     <strong>{userStr}</strong>
   </div>
 );
 
 expression_does_not_break = (
   <div>
     texty text text text text text text text text text text text{" "}
     {this.props.type}{" "}
   </div>
 );
 
 br_triggers_expression_break = (
   <div>
     <br />
-    text text text text text text text text text text text{" "}
-    {this.props.type}{" "}
+    text text text text text text text text text text text {
+      this.props.type
+    }{" "}
   </div>
 );
 
 jsx_whitespace_after_tag = (
   <div>
     <span a="a" b="b">
       {variable}
     </span>{" "}
     ({variable})
   </div>
 );
 
 x = (
   <div>
     ENDS IN <div>text text text text text text text text text text text</div>{" "}
     HRS
   </div>
 );
 
 x = (
   <div>
     <h2>Message</h2>
     Hello, I'm a simple message.
   </div>
 );
 
 x = (
   <div>
     Hello, I'm a simple message.
     <h2>Message</h2>
   </div>
 );
 
 x = (
   <div>
     <div>
       <div>
         <div>
           <div>
             Line {startRange.row + 1}:{startRange.column + 1} -{" "}
             {endRange.row + 1}:{endRange.column + 1}
             {caller}
           </div>
         </div>
       </div>
     </div>
   </div>
 );
 
 x = (
   <div>
     {" "}
     <div>text</div>
   </div>
 );
 
 // NOTE: Multiple JSX whitespaces are collapsed into a single space.
 x = <div> </div>;
 
 // Don't break a self-closing element without attributes
 // ----------
 x = (
   <p>
     text text text text text text text text text text text text text text text
     <br />
     text text text text text text
   </p>
 );
 
 x = (
   <div>
     <div>First</div>-<div>Second</div>
   </div>
 );
 
 x = (
   <div>
     <div>First</div>-<div>Second</div>
   </div>
 );
 
 x = (
   <div>
     <div>First</div>-<div>Second</div>
   </div>
 );
 
 x = (
   <div>
     <div className="first" tabIndex="1">
       First
     </div>
     -
     <div className="second" tabIndex="2">
       Second
     </div>
   </div>
 );
 
 x = (
   <div>
     <div className="first" tabIndex="1">
       First
     </div>
     -
     <div className="second" tabIndex="2">
       Second
     </div>
   </div>
 );
 
 x = (
   <div>
     <div className="first" tabIndex="1">
       First
     </div>
     -
     <div className="second" tabIndex="2">
       Second
     </div>
   </div>
 );
 
 x = (
   <div>
     {hour}:{minute}:{second}
   </div>
 );
 
 x = (
   <div>
     {hour}:{minute}:{second}
   </div>
 );
 
 x = (
   <div>
     {hour}:{minute}:{second}
   </div>
 );
 
 x = (
   <div>
     <strong>text here</strong>.<br />
   </div>
 );
 
 x = <div>Sales tax estimated using a rate of {salesTax * 100}%.</div>;
 
 x = <div>{title}&nbsp;</div>;
 
 x = (
   <div>
     <span />
     bar
   </div>
 );
 
 x = (
   <div>
     <span>
       <strong>{name}</strong>’s{" "}
     </span>
     Hello <strong>world</strong>.<br />
     <Text>You {type}ed this shipment to</Text>
   </div>
 );
 
 x = (
   <HelpBlock>
     {parameter.Description}: {errorMsg}
   </HelpBlock>
 );
 
 x = (
   <label>
     {value} solution{plural}
   </label>
 );
 
 x = <span>Copy &quot;{name}&quot;</span>;
 
 x = <BasicText light>(avg. {value}/5)</BasicText>;
 
 x = (
   <p>
     Use the <code>Button</code>'s
   </p>
 );
 
 this_really_should_split_across_lines = (
   <div>
     before{stuff}after{stuff}after{stuff}after{stuff}after{stuff}after{stuff}
     after{stuff}after
   </div>
 );
 
 let myDiv = ReactTestUtils.renderIntoDocument(
   <div>
     <div key="theDog" className="dog" />,
     <div key="theBird" className="bird" />
   </div>,
 );

```

**Prettier Similarity**: 99.47%


### typescript/arrow/16067.ts
```diff
 const foo1 =
   // comment
-  <T,>() =>
+    <T>() =>
     () =>
       1;
 
 const foo2 =
   // comment
   () => () => 1;
 
 const foo3 =
   // comment
-  <T,>() => 1;
+  <T>() => 1;
 
 foo(
   // comment
-  <T,>() =>
+  <T>() =>
     () =>
       1,
 );
 
 a ||
   // comment
-  (<T,>() =>
+  (<T>() =>
     () =>
       1);
 
 void (
   // comment
-  (<T,>() =>
+  (<T>() =>
     () =>
       1)
 );
 
 cond
   ? // comment
-    <T,>() =>
+    <T>() =>
       () =>
         1
   : // comment
-    <T,>() =>
+    <T>() =>
       () =>
         1;
 
 foo4 =
   // comment
-  <T,>() =>
+    <T>() =>
     () =>
       1;

```

**Prettier Similarity**: 83.67%


### typescript/arrow/comments.ts
```diff
 const fn1 = () => {
   return;
-}; /* foo */
+} /* foo */;
 
 const fn2 = () => {
   return;
 };
 
 // foo

```

**Prettier Similarity**: 88.89%


### typescript/as/as-const/as-const.ts
```diff
 1 /* comment */ as const;
-1 /* comment */ as const;
+1 as /* comment */ const;
 1 as const /* comment */;
 1 as const; /* comment */
 
 1 as const; // comment
 1 as const; // comment
 1 as const; // comment
 1 as const; // comment
 
 1 as /* comment */ not_const;

```

**Prettier Similarity**: 90.91%


### typescript/as/break-after-keyword/18148.ts
```diff
 // 79 width until `as` or `satisfies` keyword
 const firstItem1 = ______________.items.find(
   (item) => item.type === "item",
 ) as const;
 // 80 width until `as` or `satisfies` keyword
 const firstItem_1 = ______________.items.find(
   (item) => item.type === "item",
 ) as const;
 
 // 79 width until `as` or `satisfies` keyword
 const firstItem2 = ______________.items.find((item) => item.type === "item") as
   | string
   | number;
 // 80 width until `as` or `satisfies` keyword
-const firstItem_2 = ______________.items.find(
-  (item) => item.type === "item",
-) as string | number;
+const firstItem_2 = ______________.items.find((item) => item.type === "item") as
+  | string
+  | number;
 
 // 79 width until `as` or `satisfies` keyword
 const firstItem3 = _____________.find((item) => item.type === "item") satisfies
   | string
   | number;
 // 80 width until `as` or `satisfies` keyword
-const firstItem_3 = _____________.find(
-  (item) => item.type === "item",
-) satisfies string | number;
+const firstItem_3 = _____________.find((item) => item.type === "item") satisfies
+  | string
+  | number;
 
 // 79 width until `as` or `satisfies` keyword
 const firstItem4 = ______________.items.find(
   (item) => item.type === "item",
 ) as not_union;
 // 80 width until `as` or `satisfies` keyword
 const firstItem_4 = ______________.items.find(
   (item) => item.type === "item",
 ) as not_union;
 
 // 79 width until `as` or `satisfies` keyword
 const firstItem5 = ______________.items.find((item) => item.type === "item") as
   | a_union_will_break // comments
   | a_union_will_break2;
 // 80 width until `as` or `satisfies` keyword
-const firstItem_5 = ______________.items.find(
-  (item) => item.type === "item",
-) as
+const firstItem_5 = ______________.items.find((item) => item.type === "item") as
   | a_union_will_break // comments
   | a_union_will_break2;

```

**Prettier Similarity**: 80.43%


### typescript/as/comments/18160.ts
```diff
-1 as const /*
-comment
-*/;
-1 as /*
-comment
-*/ Foo;
-1 satisfies /*
-comment
-*/ Foo;
+// FIXME
+// TODO: reformat issue
+// 1 as /*
+// comment
+// */const;
+// 1 as /*
+// comment
+// */Foo;
+// 1 satisfies /*
+// comment
+// */Foo;
 
-1 as const;
-/*
-comment
-*/
-1 as /*
-comment
-*/ Foo;
-1 satisfies /*
-comment
-*/ Foo;
+// 1 as
+// /*
+// comment
+// */const;
+// 1 as
+// /*
+// comment
+// */Foo;
+// 1 satisfies
+// /*
+// comment
+// */Foo;
 
-1 as const /*
-comment
-*/;
-1 /*
-comment
-*/ as Foo;
-1 /*
-comment
-*/ satisfies Foo;
+// 1 as /*
+// comment
+// */
+// const;
+// 1 as /*
+// comment
+// */
+// Foo;
+// 1 satisfies /*
+// comment
+// */
+// Foo;
 
-1 as const;
-/*
-comment
-*/
-1 /*
-comment
-*/ as Foo;
-1 /*
-comment
-*/ satisfies Foo;
+// 1 as
+// /*
+// comment
+// */
+// const;
+// 1 as
+// /*
+// comment
+// */
+// Foo;
+// 1 satisfies
+// /*
+// comment
+// */
+// Foo;
 
-1 as const; // comment
-1 as Foo; // comment
-1 satisfies Foo; // comment
+// 1 as // comment
+// const;
+// 1 as // comment
+// Foo;
+// 1 satisfies // comment
+// Foo;

```

**Prettier Similarity**: 6.67%


### typescript/assignment/issue-5370.ts
```diff
-const durabilityMetricsSelectable: Immutable.OrderedSet<SomeReportingMetric> =
-  myExperienceSelectable.concat(otherDurabilityMetricsSelectable);
+const durabilityMetricsSelectable: Immutable.OrderedSet<
+  SomeReportingMetric,
+> = myExperienceSelectable.concat(otherDurabilityMetricsSelectable);

```

**Prettier Similarity**: 0.00%


### typescript/chain-expression/call-expression.ts
```diff
 // Member expressions
-(a?.b)!();
-(a?.b)!();
+a?.b!();
+a?.b!();
 (a!?.b)();
-(a.b?.c)!();
-(a.b?.c)!();
+a.b?.c!();
+a.b?.c!();
 (a.b!?.c)();
 (a!.b?.c)();
-(a?.b.c)!();
-(a?.b.c)!();
-(a?.b!.c)();
+a?.b.c!();
+a?.b.c!();
+a?.b!.c();
 (a!?.b.c)();
 a[b?.c]!();
 a[b?.c]!();
 a[b?.c!]();
 a[b!?.c]();
 (a?.b).c!();
 (a?.b).c!();
 // FIXME: ((a?.b!).c)   ();
 (a!?.b).c();
 a[b?.()]!();
 a[b?.()]!();
 a[b?.()!]();
 a[b!?.()]();
 a![b?.()]();
 (a?.b).c!();
 (a?.b).c!();
 // FIXME: ((a?.b)!.c)   ();
 // FIXME: ((a?.b!).c)   ();
 (a!?.b).c();
 (a?.()).b!();
 (a?.()).b!();
-(a?.())!.b();
-(a?.())!.b();
+a?.()!.b();
+a?.()!.b();
 (a!?.()).b();
 
 // Call expressions
-(a?.())!();
-(a?.())!();
+a?.()!();
+a?.()!();
 (a!?.())();
-(a.b.c?.())!();
-(a.b.c?.())!();
+a.b.c?.()!();
+a.b.c?.()!();
 (a.b.c!?.())();
-(a.b?.c())!();
-(a.b?.c())!();
+a.b?.c()!();
+a.b?.c()!();
 (a.b!?.c())();
-(a?.b.c())!();
-(a?.b.c())!();
-(a?.b!.c())();
+a?.b.c()!();
+a?.b.c()!();
+a?.b!.c()();
 a(b?.c)!();
 a(b?.c)!();
 a(b?.c!)();
 (a?.b)()!();
 (a?.b)()!();
-(a?.b)!()();
-(a?.b)!()();
+a?.b!()();
+a?.b!()();
 (a?.())()!();
 (a?.())()!();
-(a?.())!()();
-(a?.())!()();
+a?.()!()();
+a?.()!()();
 (a!?.())()();
 
 // Not `.callee`
 foo(a?.b!);

```

**Prettier Similarity**: 65.62%


### typescript/chain-expression/member-expression.ts
```diff
 // Member expressions
-(a?.b)!.foo;
-(a?.b)!.foo;
+a?.b!.foo;
+a?.b!.foo;
 (a!?.b).foo;
-(a.b?.c)!.foo;
-(a.b?.c)!.foo;
+a.b?.c!.foo;
+a.b?.c!.foo;
 (a.b!?.c).foo;
 (a!.b?.c).foo;
-(a?.b.c)!.foo;
-(a?.b.c)!.foo;
-(a?.b!.c).foo;
+a?.b.c!.foo;
+a?.b.c!.foo;
+a?.b!.c.foo;
 (a!?.b.c).foo;
 a[b?.c]!.foo;
 a[b?.c]!.foo;
 a[b?.c!].foo;
 a[b!?.c].foo;
 (a?.b).c!.foo;
 (a?.b).c!.foo;
-(a?.b)!.c.foo;
+a?.b!.c.foo;
 (a!?.b).c.foo;
 a[b?.()]!.foo;
 a[b?.()]!.foo;
 a[b?.()!].foo;
 a[b!?.()].foo;
 a![b?.()].foo;
 (a?.b).c!.foo;
 (a?.b).c!.foo;
-(a?.b)!.c.foo;
-(a?.b)!.c.foo;
+a?.b!.c.foo;
+a?.b!.c.foo;
 (a!?.b).c.foo;
 (a?.()).b!.foo;
 (a?.()).b!.foo;
-(a?.())!.b.foo;
-(a?.())!.b.foo;
+a?.()!.b.foo;
+a?.()!.b.foo;
 (a!?.()).b.foo;
 
 // Call expressions
-(a?.())!.foo;
-(a?.())!.foo;
+a?.()!.foo;
+a?.()!.foo;
 (a!?.()).foo;
-(a.b.c?.())!.foo;
-(a.b.c?.())!.foo;
+a.b.c?.()!.foo;
+a.b.c?.()!.foo;
 (a.b.c!?.()).foo;
-(a.b?.c())!.foo;
-(a.b?.c())!.foo;
+a.b?.c()!.foo;
+a.b?.c()!.foo;
 (a.b!?.c()).foo;
-(a?.b.c())!.foo;
-(a?.b.c())!.foo;
-(a?.b!.c()).foo;
+a?.b.c()!.foo;
+a?.b.c()!.foo;
+a?.b!.c().foo;
 a(b?.c)!.foo;
 a(b?.c)!.foo;
 a(b?.c!).foo;
 (a?.b)()!.foo;
 (a?.b)()!.foo;
-(a?.b)!().foo;
-(a?.b)!().foo;
+a?.b!().foo;
+a?.b!().foo;
 (a?.())()!.foo;
 (a?.())()!.foo;
-(a?.())!().foo;
-(a?.())!().foo;
+a?.()!().foo;
+a?.()!().foo;
 (a!?.())().foo;
 
 // Not `.object`
 _[a?.b!](
   // Computed
   a?.b!,
 )[foo];

```

**Prettier Similarity**: 62.69%


### typescript/chain-expression/test.ts
```diff
-(a?.b)!.c;
-(a?.())!.b;
-(a?.b)!.c;
-(a?.())!.b;
+a?.b!.c;
+a?.()!.b;
+a?.b!.c;
+a?.()!.b;

```

**Prettier Similarity**: 0.00%


### typescript/chain-expression/test2.ts
```diff
 {
   {
-    const rotation1 = getTransformHandles(
-      arrow,
-      h.state.zoom,
-      "mouse",
-    ).rotation!;
+    const rotation1 = getTransformHandles(arrow, h.state.zoom, "mouse")
+      .rotation!;
     const rotation2 = getTransformHandles(
       arrow,
       h.state.zoom,
       "mouse",
     ).rotation;
     const rotation3 = getTransformHandles(
       arrow,
       h.state.zoom,
       "mouse",
     )?.rotation;
   }
 }

```

**Prettier Similarity**: 73.68%


### typescript/class-and-interface/long-type-parameters/long-type-parameters.ts
```diff
 export interface MarkDef<
   M extends string | Mark = Mark,
   ES extends ExprRef | SignalRef = ExprRef | SignalRef,
 > extends A,
     B,
     C {}
 
 declare class MarkDef<
-    M extends string | Mark = Mark,
-    ES extends ExprRef | SignalRef = ExprRef | SignalRef,
-  >
-  implements A, B, C {}
+  M extends string | Mark = Mark,
+  ES extends ExprRef | SignalRef = ExprRef | SignalRef,
+> implements A, B, C {}

```

**Prettier Similarity**: 66.67%


### typescript/class/declare-field.ts
```diff
 class A {
-  declare private readonly name: string;
-  declare private readonly name2: string;
+  private declare readonly name: string;
+  private declare readonly name2: string;
 }

```

**Prettier Similarity**: 50.00%


### typescript/class/empty-method-body.ts
```diff
 // #9324
 
 class foo1 {
-  bar /* bat */();
+  bar() /* bat */;
 }
 
 // #9367
 class Test {
-  foo /* 3 */(/* 2 */);
+  foo(/* 2 */) /* 3 */;
 }

```

**Prettier Similarity**: 80.00%


### typescript/class/quoted-property.ts
```diff
 class User {
-  "username": string;
+  username: string;
 }

```

**Prettier Similarity**: 66.67%


### typescript/comments/11662.ts
```diff
-foo<a>
-// TEST
+foo<a>`// TEST
 // 1
 // 2
 // 3
 // 4
 // 5
-`x`;
+x`;

```

**Prettier Similarity**: 62.50%


### typescript/comments/16121.ts
```diff
-export type IntersectionType = FirstPart &
-  // Presence of comment makes this non-idempotent
-  {
-    an: string;
-    object: string;
-    type: string;
-  };
+// FIXME
+// TODO: reformat issue
+// export type IntersectionType =
+//   & FirstPart
+//   // Presence of comment makes this non-idempotent
+//   & {
+//     an: string,
+//     object: string,
+//     type: string,
+//   }
 
-export type IntersectionType = FirstPart &
-  // Comment Line 1
-  // Comment Line 2
-  {
-    an: string;
-    object: string;
-    type: string;
-  };
+// export type IntersectionType =
+//   & FirstPart
+//   // Comment Line 1
+//   // Comment Line 2
+//   & {
+//     an: string,
+//     object: string,
+//     type: string,
+//   }
 
-export type IntersectionType =
-  | FirstPart
-  // Presence of comment is not a problem
-  | {
-      an: string;
-      object: string;
-      type: string;
-    };
+// export type IntersectionType =
+//   | FirstPart
+//   // Presence of comment is not a problem
+//   | {
+//       an: string;
+//       object: string;
+//       type: string;
+//     };
 
-export type IntersectionType =
-  | FirstPart
-  // Comment Line 1
-  // Comment Line 2
-  | {
-      an: string;
-      object: string;
-      type: string;
-    };
+// export type IntersectionType =
+//   | FirstPart
+//   // Comment Line 1
+//   // Comment Line 2
+//   | {
+//       an: string;
+//       object: string;
+//       type: string;
+//     };

```

**Prettier Similarity**: 7.69%


### typescript/comments/16889.ts
```diff
 class A {
   @decorator
   /**
    * The method description
    *
    */
   async method() {}
 
   @decorator /**
    * The method description
    *
    */
   async method() {}
 
   @decorator /**
    * The method description
    *
    */
   async method() {}
 
   @decorator
   async /* comment */ method() {}
 
   @decorator /* comment */ async method() {}
 
   @decorator
   // line comment
   async method() {}
 
   @decorator // line comment
   async method() {}
 
   @decorator
   /* comment */
   public async method() {}
 
   @decorator
   /* comment */
   static async method() {}
 
   @decorator
   /* comment */
   protected async method() {}
 
   @decorator
   /* comment */
   protected async method() {}
 
   @decorator
   /* comment */
   *method() {}
 
   @decorator
   */* comment */ method() {}
 
   /* comment */
-  abstract method(): void;
+  abstract method():void;
 }

```

**Prettier Similarity**: 98.28%


### typescript/comments/declare_function.ts
```diff
 declare function fn(
   currentRequest: { a: number },
   // TODO this is a very very very very long comment that makes it go > 80 columns
 ): number;
 
-declare function /* foo */ f(/* baz */ a /* taz */); /* bar */
+declare function /* foo */ f(/* baz */ a /* taz */) /* bar */;

```

**Prettier Similarity**: 83.33%


### typescript/comments/mapped_types.ts
```diff
 type A = {
   // commentA
   [a in A]: string;
 };
 
 type B = {
-  /* commentB */
-  [b in B]: string;
+  /* commentB */ [b in B]: string;
 };
 
 type C = {
   [/* commentC */ c in C]: string;
 };
 
 type D = {
   [d /* commentD */ in D]: string;
 };
 
 type E = {
   [e in /* commentE */ E]: string;
 };
 
 type F = {
   [f in F /* commentF */]: string;
 };
 
 type G = {
-  [g in G /* commentG */]: string;
+  [g in G] /* commentG */ : string;
 };
 
 type H = { [/* commentH */ h in H]: string };
 
 type I = { [/* commentI */ i in I]: string };
 
 type J = { [j /* commentJ */ in J]: string };
 
 type K = { [k in /* commentK */ K]: string };
 
 type L = { [l in L /* commentL */]: string };
 
-type M = { [m in M /* commentG */]: string };
+type M = { [m in M] /* commentG */ : string };
 
 // https://github.com/excalidraw/excalidraw/blob/712f2675195ace8d507f563ec4306efe319b3c84/packages/common/src/utility-types.ts#L61-L64
 type MakeBrand<T extends string> = {
   /** @private using ~ to sort last in intellisense */
   [K in `~brand~${T}`]: T;
 };

```

**Prettier Similarity**: 91.49%


### typescript/comments/method_types.ts
```diff
 interface foo1 {
   bar3 /* foo */(/* baz */); // bat
-  bar /* foo */ /* bar */?(/* baz */) /* bat */;
+  bar /* foo */?/* bar */ (/* baz */) /* bat */;
   bar2 /* foo */(/* baz */) /* bat */;
 }
 
 interface foo2 {
-  bar /* foo */?(/* bar */ bar: /* baz */ string): /* bat */ string;
+  bar /* foo */?/* bar */ (bar: /* baz */ string): /* bat */ string;
 }
 
 interface foo3 {
   /* foo */ (/* bar */): /* baz */ string;
 }
 
 interface foo4 {
   /* foo */ (bar: /* bar */ string): /* baz */ string;
 }
 
 interface foo5 {
-  /* foo */ new (/* bar */ a: /* baz */ string): /* bat */ string;
+  /* foo */ new /* bar */ (a: /* baz */ string): /* bat */ string;
 }
 
 interface foo6 {
-  /* foo */ new (/* baz */) /* bar */ : /* bat */ string;
+  /* foo */ new /* bar */ (/* baz */): /* bat */ string;
 }
 
 type foo7 = /* foo */ (/* bar */) /* baz */ => void;
 
 type foo8 = /* foo */ (a: /* bar */ string) /* baz */ => void;
 
-let foo9: new (/* bar */) /* foo */ /* baz */ => string;
+let foo9: new /* foo */ (/* bar */) /* baz */ => string;
 
-let foo10: new (/* foo */ a: /* bar */ string) /* baz */ => string;
+let foo10: new /* foo */ (a: /* bar */ string) /* baz */ => string;
 
 abstract class Test {
   abstract foo12 /* foo */(a: /* bar */ string): /* baz */ void;
 
   abstract foo13 /* foo */(/* bar */); /* baz */
 }

```

**Prettier Similarity**: 84.62%


### typescript/compiler/indexSignatureWithInitializer.ts
```diff
 // These used to be indexers, now they are computed properties
 interface I {
   [x = ""]: string;
 }
 
 class C {
-  [(x = 0)]: string;
+  [x = 0]: string;
 }

```

**Prettier Similarity**: 87.50%


### typescript/conditional-types/parentheses.ts
```diff
 // #13275
 type Foo<T> = T extends ((...a: any[]) => infer R extends string) ? R : never;
-type Foo<T> = T extends (new (...a: any[]) => infer R extends string)
+type Foo<T> = T extends (new (
+  ...a: any[]
+) => infer R extends string)
   ? R
   : never;
 
 // #14275
-type Test<T> = T extends ((
+type Test<T> = T extends (
   token: TSESTree.Token,
-) => token is infer U extends TSESTree.Token)
+) => token is infer U extends TSESTree.Token
   ? U
   : TSESTree.Token;
 type Test<T> = T extends ((
   token: TSESTree.Token,
 ) => asserts token is infer U extends TSESTree.Token)
   ? U
   : TSESTree.Token;
 type Test<T> = T extends (new (
-  token: TSESTree.Token,
-) => token is infer U extends TSESTree.Token)
+  token: TSESTree.Token
+) => token is infer U extends TSESTree.Token
+)
   ? U
-  : TSESTree.Token;
+  : TSESTree.Token

```

**Prettier Similarity**: 64.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractInstantiations2.ts
```diff
 class A {}
 
 abstract class B {
   foo(): number {
     return this.bar();
   }
   abstract bar(): number;
 }
 
 new B();
 
 var BB: typeof B = B;
 var AA: typeof A = BB;
 new AA();
 
 function constructB(Factory: typeof B) {
   new Factory();
 }
 
 var BB = B;
 new BB();
 
 var x: any = C;
 new x();
 
 class C extends B {}
 
 abstract class D extends B {}
 
 class E extends B {
   bar() {
     return 1;
   }
 }
 
 abstract class F extends B {
   abstract foo(): number;
   bar() {
     return 2;
   }
 }
 
 abstract class G {
   abstract qux(x: number): string;
   abstract qux(): number;
   y: number;
   abstract quz(x: number, y: string): boolean;
 
   abstract nom(): boolean;
   nom(x: number): boolean;
 }
 
 class H {
-  abstract baz(): number;
+  abstract baz() : number;
 }

```

**Prettier Similarity**: 98.18%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractMixedWithModifiers.ts
```diff
 abstract class A {
   abstract foo_a();
 
   public abstract foo_b();
   protected abstract foo_c();
   private abstract foo_d();
 
-  public abstract foo_bb();
-  protected abstract foo_cc();
-  private abstract foo_dd();
+  abstract public foo_bb();
+  abstract protected foo_cc();
+  abstract private foo_dd();
 
-  static abstract foo_d();
+  abstract static foo_d();
 
   static abstract foo_e();
 }

```

**Prettier Similarity**: 73.33%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractProperties.ts
```diff
 abstract class A {
   abstract x: number;
   public abstract y: number;
   protected abstract z: number;
-  private abstract w: number;
+  private abstract w : number;
 
   abstract m: () => void;
 
   abstract foo_x(): number;
   public abstract foo_y(): number;
   protected abstract foo_z(): number;
-  private abstract foo_w(): number;
+  private abstract foo_w() : number;
 }

```

**Prettier Similarity**: 84.62%


### typescript/conformance/classes/constructorDeclarations/constructorParameters/readonlyInConstructorParameters.ts
```diff
 class C {
   constructor(readonly x: number) {}
 }
 new C(1).x = 2;
 
 class E {
-  constructor(public readonly x: number) {}
+  constructor(readonly public x: number) {}
 }
 
 class F {
   constructor(private readonly x: number) {}
 }
 new F(1).x;

```

**Prettier Similarity**: 92.31%


### typescript/custom/abstract/abstractProperties.ts
```diff
 abstract class Foo {
-  private abstract a: 1;
+  abstract private a: 1;
   private abstract b: 2;
   static abstract c: 3;
-  private abstract ["g"];
-  private abstract ["h"];
-  static abstract ["i"];
+  abstract private ['g'];
+  private abstract ['h'];
+  static abstract ['i'];
 }

```

**Prettier Similarity**: 50.00%


### typescript/declare/object-type-in-declare-function.ts
```diff
-declare function foo(this: {
-  a: boolean;
-  b: string;
-  c: number;
-}): Promise<Array<foo>>;
+declare function foo(this: { a: boolean; b: string; c: number }): Promise<
+  Array<foo>
+>;
 
 declare function bazFlip({
   a: boolean,
   b: string,
   c: number,
 }): Promise<Array<foo>>;
 
 declare function bar(
   ...{ a: boolean, b: string, c: number }
 ): Promise<Array<foo>>;
 
 declare function bar(
   ...x: { a: boolean; b: string; c: number }
 ): Promise<Array<foo>>;

```

**Prettier Similarity**: 73.68%


### typescript/declare/trailing-comma/function-rest-trailing-comma.ts
```diff
-declare function foo(...args: any[]);
-declare function foo(
-  ...long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_args: any[]
-);
+declare function foo(...args: any[], )
+declare function foo(...long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_args: any[], )

```

**Prettier Similarity**: 0.00%


### typescript/decorators-ts/angular.ts
```diff
 @Component({
   selector: "toh-hero-button",
-  template: `<button>{{ label }}</button>`,
+  template: `<button>{{label}}</button>`,
 })
 export class HeroButtonComponent {
   @Output() change = new EventEmitter<any>();
   @Input() label: string;
 }

```

**Prettier Similarity**: 87.50%


### typescript/decorators-ts/typeorm.ts
```diff
 @Entity()
 export class Board {
   @PrimaryGeneratedColumn()
   id: number;
 
   @Column()
   slug: string;
 
   @Column()
   name: string;
 
   @Column()
   theme: string;
 
   @Column()
   description: string;
 
-  @OneToMany((type) => Topic, (topic) => topic.board)
+  @OneToMany(
+    (type) => Topic,
+    (topic) => topic.board,
+  )
   topics: Topic[];
 }

```

**Prettier Similarity**: 82.61%


### typescript/decorators/decorators-comments.ts
```diff
 class Foo1 {
   @foo
   // comment
   async method() {}
 }
 
 class Foo2 {
   @foo
   // comment
   private method() {}
 }
 
 class Foo3 {
   @foo
   // comment
   *method() {}
 }
 
 class Foo4 {
   @foo
   // comment
   async *method() {}
 }
 
 class Something {
   @foo()
   // comment
   readonly property: Array<string>;
 }
 
 class Something2 {
   @foo()
-  // comment
-  abstract property: Array<string>;
+    // comment
+    abstract property: Array<string>
 }

```

**Prettier Similarity**: 94.29%


### typescript/definite/without-annotation.ts
```diff
 class Foo {
-  a!;
-  #b!;
-  static c!;
-  [d]! = 1;
-  "e"!;
+  a!
+  #b!
+  static c!
+  [d]! = 1
+  'e'!
 }
 
-let a! = x;
-const b! = x;
-var c! /* */ = x;
-export const d! = x;
+let a! = x
+const b! = x
+var c/* */! = x
+export const d! = x

```

**Prettier Similarity**: 25.00%


### typescript/import-export/empty-import.ts
```diff
 import type {} from "@types/googlemaps";
 import "a";
 import /* comment */ "a";
 import // comment
 "a";
+import type {/* comment */} from "a";
+import /* comment */ type {} from "a";
+import type {} /* comment */ from "a";
 import type {} from /* comment */ "a";
-import type {} from /* comment */ "a";
-import type {} from /* comment */ "a";
-import type {} from /* comment */ "a";
-import type {} from /* comment */ /* comment */ /* comment */ /* comment */ "a";
+import /* comment */ type {/* comment */} /* comment */ from /* comment */ "a";
+import type {
+  // comment
+} from "a";
+import // comment
+type {} from "a";
 import type {} from // comment
 "a";
 import type {} from // comment
 "a";
-import type {} from // comment
-"a";
-import type {} from // comment
-"a";
-import type {} from // comment
-// comment
-// comment
+import type // comment
+{
+  // comment
+} from // comment
 // comment
 "a";
 
 import // {} from
 "a";
 import type {} from // comment ends with from
 "a";
 import type {} from /* comment ends with from */ "a";
 import type {} from // comment not ends with from ___
 "a";
 import type {} from /* comment not ends with from ___ */ "a";
 
 import // comment ends with from
 "a";
 import /* comment ends with from */ "a";
 import // comment not ends with from ___
 "a";
 import /* comment not ends with from ___ */ "a";

```

**Prettier Similarity**: 68.29%


### typescript/import-export/type-modifier.ts
```diff
 export type { SomeThing };
 export type { A as B };
 export type { B as C } from "./a";
 export type { foo } from "bar";
 export type { foo };
 
 // this should be treated as a normal import statement
 import type from "./foo";
 
 import type { SomeThing } from "./some-module.js";
 import type { foo, bar } from "baz";
 import type { foo as bar } from "baz";
 import type * as foo from "./bar";
 import type foo from "bar";
-import type foo, { bar } from "bar";
+import type foo, { bar } from 'bar';

```

**Prettier Similarity**: 93.33%


### typescript/import-type/attributes/import-type-attributes.ts
```diff
-type A = import("foo", { with: { type: "json" } });
+type A = import("foo", { with: { type: "json" }});

```

**Prettier Similarity**: 0.00%


### typescript/import-type/comma/comma.ts
```diff
-type A = import("foo", {
-  with: {
-    type: "json",
-  },
-});
+type A = import("foo", { with: { type: "json" }});
 type A = import("foo", {
   with: {
-    type: "json",
-  },
-});
+  type: "json"}
+,})
 // Not supported, https://github.com/microsoft/TypeScript/issues/61489
 // type A = import("foo", {
 //   with: {
 //   type: "json"}},)

```

**Prettier Similarity**: 42.86%


### typescript/import-type/long-module-name/long-module-name2.ts
```diff
 const plugin = {};
-export default plugin as typeof import(
-  // Comment
-  "@babel/plugin-transform-react-jsx"
-).default;
+export default plugin as typeof import(// Comment
+"@babel/plugin-transform-react-jsx").default;

```

**Prettier Similarity**: 20.00%


### typescript/import-type/long-module-name/long-module-name4.ts
```diff
 interface RuleMap {
   "arrow-parens": typeof import("./long/long/long/long/long/path/to/rules/arrow-parens");
   "consistent-return": typeof import("./long/long/long/long/long/path/to/rules/consistent-return");
   "dot-notation": typeof import("./long/long/long/long/long/path/to/rules/dot-notation");
   "init-declarations": typeof import("./long/long/long/long/long/path/to/rules/init-declarations");
   "max-params": typeof import("./long/long/long/long/long/path/to/rules/max-params");
   "no-dupe-args": typeof import("./long/long/long/long/long/path/to/rules/no-dupe-args");
   "no-dupe-class-members": typeof import("./long/long/long/long/long/path/to/rules/no-dupe-class-members");
   "no-empty-function": typeof import("./long/long/long/long/long/path/to/rules/no-empty-function");
   "no-implicit-globals": typeof import("./long/long/long/long/long/path/to/rules/no-implicit-globals");
   "no-invalid-this": typeof import("./long/long/long/long/long/path/to/rules/no-invalid-this");
   "no-loop-func": typeof import("./long/long/long/long/long/path/to/rules/no-loop-func");
   "no-loss-of-precision": typeof import("./long/long/long/long/long/path/to/rules/no-loss-of-precision");
   "no-magic-numbers": typeof import("./long/long/long/long/long/path/to/rules/no-magic-numbers");
   "no-restricted-globals": typeof import("./long/long/long/long/long/path/to/rules/no-restricted-globals");
   "no-restricted-imports": typeof import("./long/long/long/long/long/path/to/rules/no-restricted-imports");
   "no-undef": typeof import("./long/long/long/long/long/path/to/rules/no-undef");
   "no-unused-expressions": typeof import("./long/long/long/long/long/path/to/rules/no-unused-expressions");
   "no-useless-constructor": typeof import("./long/long/long/long/long/path/to/rules/no-useless-constructor");
   "prefer-const": typeof import("./long/long/long/long/long/path/to/rules/prefer-const");
   "prefer-destructuring": typeof import("./long/long/long/long/long/path/to/rules/prefer-destructuring");
-  "prefer-destructuring2": typeof import(
-    // comment
-    "./long/long/long/long/long/path/to/rules/prefer-destructuring"
-  );
+  "prefer-destructuring2": typeof import(// comment
+  "./long/long/long/long/long/path/to/rules/prefer-destructuring");
   "prefer-destructuring3": // comment
   typeof import("./long/long/long/long/long/path/to/rules/prefer-destructuring");
   strict: typeof import("./long/long/long/long/long/path/to/rules/strict");
 }

```

**Prettier Similarity**: 86.21%


### typescript/import-type/long-module-name/long-module-name5.ts
```diff
 type A =
   import("./long/long/long/long/long/long/long/long/long/long/path/to/module");
-type B = import(
-  "./long/long/long/long/long/long/long/long/long/long/path/to/module",
-  { with: { type: "json" } }
-);
-type C = import(
-  "./long/long/long/long/long/long/long/long/long/long/path/to/module",
-  {
-    with: {
-      type: "json",
-    },
-  }
-);
-type D = import(
-  "./long/long/long/long/long/long/long/long/long/long/path/to/module",
-  {
-    with: { type: "json" },
-  }
-);
+type B =
+  import("./long/long/long/long/long/long/long/long/long/long/path/to/module", { with: {
+    type: "json",
+  }});
+type C =
+  import("./long/long/long/long/long/long/long/long/long/long/path/to/module", { with: {
+    type: "json",
+  }});
+type D =
+  import("./long/long/long/long/long/long/long/long/long/long/path/to/module", { with: {
+    type: "json",
+  }});

```

**Prettier Similarity**: 10.00%


### typescript/instantiation-expression/17714.ts
```diff
-void <_T extends never>() => {}<never>;
+// FIXME
+// TODO: reformat issue
+// void (<_T extends never>() => {})<never>;

```

**Prettier Similarity**: 0.00%


### typescript/interface2/break/break.ts
```diff
-export interface Environment1
-  extends GenericEnvironment<SomeType, AnotherType, YetAnotherType> {
+export interface Environment1 extends GenericEnvironment<
+  SomeType,
+  AnotherType,
+  YetAnotherType,
+> {
   m(): void;
 }
 export class Environment1____ extends GenericEnvironment<
   SomeType,
   AnotherType,
-  YetAnotherType
+  YetAnotherType,
 > {
   m(): void;
 }
 export class Environment2 extends GenericEnvironment<
   SomeType,
   AnotherType,
   YetAnotherType,
   DifferentType1,
   DifferentType2,
   DifferentType3,
-  DifferentType4
+  DifferentType4,
 > {
-  m() {}
+  m() {};
 }
 
 // Declare Interface Break
 declare interface ExtendsOne extends ASingleInterface {
   x: string;
 }
 
 declare interface ExtendsLarge
   extends ASingleInterfaceWithAReallyReallyReallyReallyLongName {
   x: string;
 }
 declare class ExtendsLarge____ extends ASingleInterfaceWithAReallyReallyReallyReallyLongName {
   x: string;
 }
 
 declare interface ExtendsMany
   extends Interface1,
     Interface2,
     Interface3,
     Interface4,
     Interface5,
     Interface6,
     Interface7 {
   x: string;
 }
 
 // Interface declaration break
 interface ExtendsOne extends ASingleInterface {
   x: string;
 }
 
 interface ExtendsLarge
   extends ASingleInterfaceWithAReallyReallyReallyReallyLongName {
   x: string;
 }
 class ExtendsLarge____ extends ASingleInterfaceWithAReallyReallyReallyReallyLongName {
   x: string;
 }
 
 interface ExtendsMany
   extends Interface1,
     Interface2,
     Interface3,
     Interface4,
     Interface5,
     Interface6,
     Interface7 {
   s: string;
 }
 
 // Generic Types
 interface ExtendsOne extends ASingleInterface<string> {
   x: string;
 }
 
 interface ExtendsLarge
   extends ASingleInterfaceWithAReallyReallyReallyReallyLongName<string> {
   x: string;
 }
 class ExtendsLarge____ extends ASingleInterfaceWithAReallyReallyReallyReallyLongName<string> {
   x: string;
 }
 
 interface ExtendsMany
   extends ASingleGenericInterface<
     Interface1,
     Interface2,
     Interface3,
     Interface4,
     Interface5,
     Interface6,
     Interface7
   > {
   x: string;
 }
 class ExtendsMany____ extends ASingleGenericInterface<
   Interface1,
   Interface2,
   Interface3,
   Interface4,
   Interface5,
   Interface6,
   Interface7
 > {
   x: string;
 }
 
 interface ExtendsManyWithGenerics
   extends InterfaceOne,
     InterfaceTwo,
     ASingleGenericInterface<
       Interface1,
       Interface2,
       Interface3,
       Interface4,
       Interface5,
       Interface6,
       Interface7
     >,
     InterfaceThree {
   x: string;
 }
 
 export interface ExtendsLongOneWithGenerics
   extends Bar<
     SomeLongTypeSomeLongTypeSomeLongTypeSomeLongType,
     ToBreakLineToBreakLineToBreakLine
   > {}
 export class ExtendsLongOneWithGenerics____ extends Bar<
   SomeLongTypeSomeLongTypeSomeLongTypeSomeLongType,
   ToBreakLineToBreakLineToBreakLine
 > {}

```

**Prettier Similarity**: 94.12%


### typescript/intersection/consistent-with-flow/intersection-parens.ts
```diff
 type A = (number | string) & boolean;
 type B = (number | string) & boolean;
 type C = (number | string) & boolean;
 type D = (number | string) & boolean;
 
 let b1: C;
 let b2: C;
 let b3: C;
 let b4: C;
 let b5: C;
 let b6: /*1*/ C;
 let b7: /*1*/ C;
 let b8: /*1*/ C;
 let b9: /*1*/ C;
 let b10: /*1*/ /*2*/ C;
 let b11: /*1*/ /*2*/ C;
 
 let bb1: /*1*/ /*2*/ C & D;
 let bb2: /*1*/ /*2*/ C & /*3*/ D;
 let bb3: /*1*/ /*2*/ C & /*3*/ D /*5*/;
 
 type B2 = C;
 type B3 = C;
 type B4 = C;
 type B5 = C;
-type B6 /*1*/ = C;
-type B7 /*1*/ = C;
-type B8 /*1*/ = C;
-type B9 /*1*/ = C;
-type B10 /*1*/ = /*2*/ C;
-type B11 /*1*/ = /*2*/ C;
-type B12 /*1*/ = C;
+type B6 = /*1*/ C;
+type B7 = /*1*/ C;
+type B8 = /*1*/ C;
+type B9 = /*1*/ C;
+type B10 = /*1*/ /*2*/ C;
+type B11 = /*1*/ /*2*/ C;
+type B12 = /*1*/ C;
 
 type Bb1 = /*1*/ /*2*/ C & D;
 type Bb2 = /*1*/ /*2*/ C & /*3*/ D;
 type Bb3 = /*1*/ /*2*/ C & /*3*/ D /*4*/;
 
-type D1 /*1*/ = a & b;
-type D2 /*1*/ = a & b;
-type D3 /*1*/ = a & b;
-type D4 /*1*/ = a & b;
-type D5 /*1*/ = a & b;
-type D6 /*0*/ /*1*/ = a & b;
+type D1 = /*1*/ a & b;
+type D2 = /*1*/ a & b;
+type D3 = /*1*/ a & b;
+type D4 = /*1*/ a & b;
+type D5 = /*1*/ a & b;
+type D6 /*0*/ = /*1*/ a & b;

```

**Prettier Similarity**: 69.77%


### typescript/intersection/intersection-parens.ts
```diff
 export type A = aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa &
   bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type B = aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa &
   bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type C = aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa &
   bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type D = aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa &
   bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type Multi = (string & number)[];
 
 function f(): string & number {}
 
 var x: string & number;
 var y: string & number;
 
 class Foo<T extends string & number> {}
 
 interface Interface {
   i: (X & Y) | Z;
   j: Partial<X & Y>;
 }
 
 type State = {
   sharedProperty: any;
 } & ({ discriminant: "FOO"; foo: any } & { discriminant: "BAR"; bar: any } & {
   discriminant: "BAZ";
   baz: any;
 });
 
 const foo1 = [abc, def, ghi, jkl, mno, pqr, stu, vwx, yz] as (string &
   undefined)[];
 
 const foo2: (AAAAAAAAAAAAAAAAAAAAAA &
   BBBBBBBBBBBBBBBBBBBBBB &
   CCCCCCCCCCCCCCCCCCCCCC &
   DDDDDDDDDDDDDDDDDDDDDD)[] = [];
 
 const foo3: keyof (AAAAAAAAAAAAAAAAAAAAAA &
   BBBBBBBBBBBBBBBBBBBBBB &
   CCCCCCCCCCCCCCCCCCCCCC &
   DDDDDDDDDDDDDDDDDDDDDD) = bar;
 
 const foo4: foo &
   (AAAAAAAAAAAAAAAAAAAAAA &
     BBBBBBBBBBBBBBBBBBBBBB &
     CCCCCCCCCCCCCCCCCCCCCC &
     DDDDDDDDDDDDDDDDDDDDDD) = bar;
 
 let a1: C;
 let a2: C;
 let a3: C;
 let a4: C;
 let a5: C;
 let a6: /*1*/ C;
 let a7: /*1*/ C;
 let a8: /*1*/ C;
 let a9: /*1*/ C;
 let a10: /*1*/ /*2*/ C;
 let a11: /*1*/ /*2*/ C;
 
 let aa1: /*1*/ /*2*/ C & D;
 let aa2: /*1*/ /*2*/ C & /*3*/ D;
 let aa3: /*1*/ /*2*/ C & /*3*/ D /*4*/;
 
 type A1 = C;
 type A2 = C;
 type A3 = C;
 type A4 = C;
 type A5 = C;
-type A6 /*1*/ = C;
-type A7 /*1*/ = C;
-type A8 /*1*/ = C;
-type A9 /*1*/ = C;
-type A10 /*1*/ = /*2*/ C;
-type A11 /*1*/ = /*2*/ C;
-type A12 /*1*/ = C;
+type A6 = /*1*/ C;
+type A7 = /*1*/ C;
+type A8 = /*1*/ C;
+type A9 = /*1*/ C;
+type A10 = /*1*/ /*2*/ C;
+type A11 = /*1*/ /*2*/ C;
+type A12 = /*1*/ C;
 type A13 = /*1*/ C;
 
 type Aa1 = /*1*/ /*2*/ C & D;
 type Aa2 = /*1*/ /*2*/ C & /*3*/ D;
 type Aa3 = /*1*/ /*2*/ C & /*3*/ D /*4*/;
 
-type C1 /*1*/ = a & b;
-type C2 /*1*/ = a & b;
-type C3 /*1*/ = a & b;
-type C4 /*1*/ = a & b;
-type C5 /*1*/ = a & b;
-type C6 /*0*/ /*1*/ = a & b;
+type C1 = /*1*/ a & b;
+type C2 = /*1*/ a & b;
+type C3 = /*1*/ a & b;
+type C4 = /*1*/ a & b;
+type C5 = /*1*/ a & b;
+type C6 /*0*/ = /*1*/ a & b;
 
 type Ctor = (new () => X) & Y;

```

**Prettier Similarity**: 86.17%


### typescript/intersection/mutiple-comments/17192.ts
```diff
-export type ErrorLike = SerializedProps<Error> &
-  // cause is a new addition to Error that is not yet available in all runtimes. We have added
-  // it to try and pinpoint additional reasoning for failures such as Node's fetch.
-  // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause
-  { cause?: unknown };
+// FIXME
+// TODO: reformat issue
+// export type ErrorLike =
+//   SerializedProps<Error> &
+//   // cause is a new addition to Error that is not yet available in all runtimes. We have added
+//   // it to try and pinpoint additional reasoning for failures such as Node's fetch.
+//   // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause
+//   { cause?: unknown };
 
-export type ErrorLike2 = SerializedProps<Error> & // cause is a new addition to Error that is not yet available in all runtimes. We have added
-  // it to try and pinpoint additional reasoning for failures such as Node's fetch.
-  // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause
-  { cause?: unknown };
+// export type ErrorLike2 =
+//   SerializedProps<Error> & // cause is a new addition to Error that is not yet available in all runtimes. We have added
+//   // it to try and pinpoint additional reasoning for failures such as Node's fetch.
+//   // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause
+//   { cause?: unknown };

```

**Prettier Similarity**: 7.14%


### typescript/last-argument-expansion/decorated-function.tsx
```diff
-const Counter = decorator("my-counter")((props: {
-  initialCount?: number;
-  label?: string;
-}) => {
-  const p = useDefault(props, {
-    initialCount: 0,
-    label: "Counter",
-  });
+const Counter = decorator("my-counter")(
+  (props: { initialCount?: number; label?: string }) => {
+    const p = useDefault(props, {
+      initialCount: 0,
+      label: "Counter",
+    });
 
-  const [s, set] = useState({ count: p.initialCount });
-  const onClick = () => set("count", (it) => it + 1);
+    const [s, set] = useState({ count: p.initialCount });
+    const onClick = () => set("count", (it) => it + 1);
 
-  return () => (
-    <button onclick={onClick}>
-      {p.label}: {s.count}
-    </button>
-  );
-});
+    return () => (
+      <button onclick={onClick}>
+        {p.label}: {s.count}
+      </button>
+    );
+  },
+);
 
-const Counter2 = decorators.decorator("my-counter")((props: {
-  initialCount?: number;
-  label?: string;
-}) => {
-  return () => (
-    <button onclick={onClick}>
-      {p.label}: {s.count}
-    </button>
-  );
-});
+const Counter2 = decorators.decorator("my-counter")(
+  (props: { initialCount?: number; label?: string }) => {
+    return () => (
+      <button onclick={onClick}>
+        {p.label}: {s.count}
+      </button>
+    );
+  },
+);
 
-export default decorators.decorator("my-counter")((props: {
-  initialCount?: number;
-  label?: string;
-}) => {
-  return foo;
-});
+export default decorators.decorator("my-counter")(
+  (props: { initialCount?: number; label?: string }) => {
+    return foo;
+  },
+);
 
-export = decorators.decorator("my-counter")((props: {
-  initialCount?: number;
-  label?: string;
-}) => {
-  return foo;
-});
+export = decorators.decorator("my-counter")(
+  (props: { initialCount?: number; label?: string }) => {
+    return foo;
+  },
+);
 
-module.exports = decorators.decorator("my-counter")((props: {
-  initialCount?: number;
-  label?: string;
-}) => {
-  return foo;
-});
+module.exports = decorators.decorator("my-counter")(
+  (props: { initialCount?: number; label?: string }) => {
+    return foo;
+  },
+);
 
 const Counter = decorator("foo")(
   decorator("bar")(
     (props: {
       loremFoo1: Array<Promise<any>>;
       ipsumBarr2: Promise<number>;
     }) => {
       return <div />;
     },
   ),
 );

```

**Prettier Similarity**: 27.87%


### typescript/mapped-type/break-mode/break-mode.ts
```diff
 type A1 = { readonly [A in B]: T };
 type A2 = {
   readonly [A in B]: T;
 };
-type A3 = { readonly [A in B]: T };
-type A4 = { readonly [A in B]: T };
+type A3 = {
+  readonly [A in B]: T;
+};
+type A4 = {
+  readonly [A in B]: T;
+};
 type A5 = { readonly [A in B]: T };
 type A6 = { readonly [A in B]: T };
 type A7 = { readonly [A in B]: T };
 
 type B1 = { [A in B]: T };
 type B2 = {
   [A in B]: T;
 };
-type B4 = { [A in B]: T };
+type B4 = {
+  [A in B]: T;
+};
 
 type C1 = { +readonly [A in B]: T };
-type C2 = { +readonly [A in B]: T };
+type C2 = {
+  +readonly [A in B]: T;
+};
 type C3 = {
   +readonly [A in B]: T;
 };
 
 type D1 = { -readonly [A in B]: T };
-type D2 = { -readonly [A in B]: T };
+type D2 = {
+  -readonly [A in B]: T;
+};
 type D3 = {
   -readonly [A in B]: T;
 };

```

**Prettier Similarity**: 59.46%


### typescript/mapped-type/issue-11098.ts
```diff
 type Type = {
   // comment
   readonly [T in number];
 };
 
 type Type = {
   // comment1
   // comment2
   readonly [T in number];
 };
 
 type Type = {
   // comment
   +readonly [T in number];
 };
 
 type Type = {
   // comment
   -readonly [T in number];
 };
 
 type Type = {
   // comment
   +readonly [T in number];
 };
 
 type Type = {
   // comment
   +readonly [T in number];
 };
 
 type Type = {
   // comment
   readonly [T in number];
 };
 
 type Type = {
   // comment
   [T in number];
 };
 
 type Type = {
   // comment
   readonly [T in number];
 };
 
 type Type = {
   // foo
-  /* bar */
-  readonly [T in number];
+  /* bar */ readonly [T in number];
 };

```

**Prettier Similarity**: 96.08%


### typescript/multiparser-css/issue-6259.ts
```diff
 const yesFrame = (
   ...args: Interpolation<ThemedStyledProps<{}, Theme>>[]
 ) => css`
-  ${ChatRoot}[data-frame="yes"] & {
-    ${css({}, ...args)}
-  }
+    ${ChatRoot}[data-frame="yes"] & {
+        ${css({}, ...args)}
+    }
 `;

```

**Prettier Similarity**: 57.14%


### typescript/non-null/optional-chain.ts
```diff
 a?.b!.c;
 a?.b!.c.d;
 a?.b.c!.d;
 a!.b?.c;
 a?.b!?.c;
 a?.b!.c?.c;
-(a?.b)!.c;
-(a?.b)!.c;
+a?.b!.c;
+a?.b!.c;
 
 a?.().b!.c;
 a?.().b!.c.d;
 a?.().b.c!.d;
 a?.().b!?.c;
 a?.().b!.c?.c;
-(a?.().b)!.c;
-(a?.().b)!.c;
+a?.().b!.c;
+a?.().b!.c;
 
-(a?.b)![c?.d!];
+a?.b![c?.d!];

```

**Prettier Similarity**: 72.22%


### typescript/prettier-ignore/issue-14238.ts
```diff
-export const foo = // prettier-ignore
-  (bar as Baz).qux;
+export const foo =
+  (
+  // prettier-ignore
+  bar as Baz
+).qux;

```

**Prettier Similarity**: 0.00%


### typescript/prettier-ignore/mapped-types.ts
```diff
 type a = {
     // prettier-ignore
     [A in B]: C  |  D
   };
 
 type b = {
     [
       // prettier-ignore
       A in B
     ]: C  |  D
   };
 
 type c = {
-  [A in B]: C | D; // prettier-ignore
+  [A in B]: C | D;
 };
 
 type d = {
-  [A in B]: C | D; // prettier-ignore
+  [A in B]: // prettier-ignore
+  C | D;
 };
 
 type e = {
     [
       /* prettier-ignore */
       A in B
     ]: C  |  D
   };
 
 type f = {
-  [A /* prettier-ignore */ in B]: C | D;
+  [A in B]: C | D;
 };
 
 type g = {
-  [A in B /* prettier-ignore */]: C | D;
+  [A in B]: /* prettier-ignore */
+  C | D;
 };
 
 type h = {
     /* prettier-ignore */ [A in B]: C  |  D
   };
 
 type i = {
-    [/* prettier-ignore */ A in B ]: C  |  D
-  };
+  [/* prettier-ignore */ A in B]: C | D;
+};
 
 type j = {
   [A in /* prettier-ignore */ B]: C | D;
 };
 
 type k = {
-  [A in B]: /* prettier-ignore */ C  |  D;
+  [A in B]: /* prettier-ignore */ C | D;
 };
 
 type l = {
     /* prettier-ignore */
     [A in B]: C  |  D
   };

```

**Prettier Similarity**: 84.21%


### typescript/prettier-ignore/prettier-ignore-nested-unions.ts
```diff
 export type a =
   // foo
   | (foo1 & foo2)
   // bar
   | (bar1 & bar2)
   // prettier-ignore
-  | (| aaaaaaaaaaaaa&1
-    // b
-    | bbbbbbbbbbbbb&2)
+  | (
+      | (aaaaaaaaaaaaa & 1)
+      // b
+      | (bbbbbbbbbbbbb & 2)
+    )
   // baz
   | (baz1 & baz2);
 
 export type b =
   // foo
   | (foo1 & foo2)
   // bar
   | (bar1 & bar2)
   // prettier-ignore
-  | (| aaaaaaaaaaaaa&1
-    // b
-    | bbbbbbbbbbbbb&2)
+  | (
+      | (aaaaaaaaaaaaa & 1)
+      // b
+      | (bbbbbbbbbbbbb & 2)
+    )
   // baz
   | (baz1 & baz2);

```

**Prettier Similarity**: 62.96%


### typescript/prettier-ignore/prettier-ignore-parenthesized-type.ts
```diff
 type Foo =
   // prettier-ignore
-  aa;
+  (
+    aa
+  );

```

**Prettier Similarity**: 40.00%


### typescript/property-signature/consistent-with-flow/comments.ts
```diff
 interface A {
-  property: B; // Comment
+  property: // Comment
+  B;
 }
 
 interface A {
   property: /* Comment */ B;
 }

```

**Prettier Similarity**: 75.00%


### typescript/property-signature/consistent-with-flow/union.ts
```diff
 export interface DirectiveArgumentNode extends ArrayExpression {
   elements: // dir, exp, arg, modifiers
-  | [string]
+    | [string]
     | [string, ExpressionNode]
     | [string, ExpressionNode, ExpressionNode]
     | [string, ExpressionNode, ExpressionNode, ObjectExpression];
 }

```

**Prettier Similarity**: 85.71%


### typescript/satisfies-operators/comments-unstable.ts
```diff
 const t1 = {
   prop1: 1,
   prop2: 2,
   prop3: 3,
-} satisfies Record<string, number>; // Comment
+} satisfies
+  // Comment
+  Record<string, number>;

```

**Prettier Similarity**: 57.14%


### typescript/test-declarations/test_declarations.ts
```diff
-test("does something really long and complicated so I have to write a very long name for the test", <T>(done) => {
+test("does something really long and complicated so I have to write a very long name for the test", <
+  T,
+>(done) => {
   console.log("hello!");
 });

```

**Prettier Similarity**: 40.00%


### typescript/trailing-comma/trailing.ts
```diff
 export class BaseSingleLevelProfileTargeting<
   T extends ValidSingleLevelProfileNode,
 > {}
 
 enum Enum {
   x = 1,
   y = 2,
 }
 
 const {
   longKeySoThisWillGoOnMultipleLines,
   longKeySoThisWillGoOnMultipleLines2,
   longKeySoThisWillGoOnMultipleLines3,
-  ...rest
+  ...rest,
 } = something;

```

**Prettier Similarity**: 93.33%


### typescript/trailing-comma/type-parameters-vs-arguments.ts
```diff
 class FooClass<A, B, C> {
   a: A;
   b: B;
   c: C;
 }
 
 const instance = new FooClass<
-  boolean,
-  number,
-  string // [ts] Trailing comma not allowed.
->();
+	boolean,
+	number,
+	string, // [ts] Trailing comma not allowed.
+	>();

```

**Prettier Similarity**: 63.64%


### typescript/type-alias/conditional.ts
```diff
-type FallbackFlags<F extends Flags | undefined> =
-  Equals<NonNullableFlag<F>["flags"], {}> extends true
-    ? Dict<any>
-    : NonNullableFlag<F>["flags"];
+type FallbackFlags<F extends Flags | undefined> = Equals<
+  NonNullableFlag<F>["flags"],
+  {}
+> extends true
+  ? Dict<any>
+  : NonNullableFlag<F>["flags"];
 
-export type UnPromise<Type extends Promise<unknown>> =
-  Type extends Promise<infer Generic> ? Generic : never;
+export type UnPromise<Type extends Promise<unknown>> = Type extends Promise<
+  infer Generic
+>
+  ? Generic
+  : never;
 
-export type Equals<X, Y> =
-  (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2
-    ? true
-    : false;
+export type Equals<X, Y> = (<T>() => T extends X ? 1 : 2) extends <
+  T,
+>() => T extends Y ? 1 : 2
+  ? true
+  : false;
 
-export type _Repeat<A extends any, N extends number, L extends List = []> =
-  __Repeat<N, A, L> extends infer X ? Cast<X, List> : never;
+export type _Repeat<
+  A extends any,
+  N extends number,
+  L extends List = [],
+> = __Repeat<N, A, L> extends infer X ? Cast<X, List> : never;
 
 export type Repeat<
   A extends any,
   N extends number,
   L extends List = [],
 > = N extends unknown ? (L extends unknown ? _Repeat<A, N, L> : never) : never;
 
 export type Intersect<U1 extends any, U2 extends any> = U1 extends unknown
   ? U2 extends unknown
     ? { 1: U1; 0: never }[Equals<U1, U2>]
     : never
   : never;

```

**Prettier Similarity**: 41.67%


### typescript/type-params/18041.ts
```diff
 type BufferStreamOrVoid<
   C extends undefined | Buffer | Stream | boolean,
   P extends undefined | Buffer,
-  R extends Buffer | Stream | void = // Above line comment about exclue. // Inline comment about void.
+  R extends Buffer | Stream | void = // Inline comment about void.
+  // Above line comment about exclue.
   Exclude<C, false> extends never
     ? void
     : C & P extends Buffer
       ? Buffer
       : Stream,
 > = R;
 
 type A<
-  B = // Above line comment about exclue. // Inline comment about void.
+  B = // Inline comment about void.
+  // Above line comment about exclue.
   C,
 > = R;

```

**Prettier Similarity**: 76.47%


### typescript/type-params/const.ts
```diff
 function a<const T>() {}
 function b<const T extends U>() {}
 function c<T, const U>() {}
 declare function d<const T>();
-<const T,>() => {};
+<const T>() => {};
 <const T extends U>() => {};
 (function <const T>() {});
 (function <const T extends U>() {});
 (function <T, const U>() {});
 
 class A<const T> {}
 class B<const T extends U> {}
 class C<T, const U> {}
 class D<const in T> {}
 class E<const in T> {}
 (class<const T> {});
 (class<const T extends U> {});
 (class<T, const U> {});
 (class<const in T> {});
 (class<const in T> {});
 
 interface I<const T> {}
 interface J<const T extends U> {}
 interface K<T, const U> {}
-interface L<const in T> {}
+interface L<in const T> {}
 interface M<const in T> {}
 
 class _ {
   method<const T>() {}
   method<const T extends U>() {}
   method<T, const U>() {}
 }

```

**Prettier Similarity**: 93.75%


### typescript/type-params/constraints-and-default-2.ts
```diff
 a = {
   parseFunctionBodyAndFinish<
     T extends
       | N.Function
       | N.TSDeclareMethod
       | N.TSDeclareFunction
       | N.ClassPrivateMethod,
   >() {},
 };
 a = {
   parseFunctionBodyAndFinish<
     T =
       | N.Function
       | N.TSDeclareMethod
       | N.TSDeclareFunction
       | N.ClassPrivateMethod,
   >() {},
 };
 
 function parseFunctionBodyAndFinish<
   T extends
     | N.Function
     | N.TSDeclareMethod
     | N.TSDeclareFunction
     | N.ClassPrivateMethod
     | Foo
     | Bar
     | Baz,
 >();
 function parseFunctionBodyAndFinish<
   T =
     | N.Function
     | N.TSDeclareMethod
     | N.TSDeclareFunction
     | N.ClassPrivateMethod
     | Foo
     | Bar
     | Baz,
 >();
 
 function parseFunctionBodyAndFinish<
   T extends // comment
-    N.Function | N.TSDeclareMethod | Baz,
+  N.Function | N.TSDeclareMethod | Baz,
 >();
 function parseFunctionBodyAndFinish<
-  T = N.Function | N.TSDeclareMethod | Baz, // comment
+  T = // comment
+  N.Function | N.TSDeclareMethod | Baz,
 >();
 
 function makeChainWalker<
   ArgT extends {
     options: ValidatedOptions;
     dirname: string;
     filepath?: string;
   },
 >() {}
 function makeChainWalker2<
   ArgT = {
     options: ValidatedOptions;
     dirname: string;
     filepath?: string;
   },
 >() {}

```

**Prettier Similarity**: 95.24%


### typescript/union/comments/18106.ts
```diff
 export interface DirectiveArgumentNode1 extends ArrayExpression {
   elements: // dir, exp, arg, modifiers
-  | [string]
+    | [string]
     | [string, ExpressionNode]
     | [string, ExpressionNode, ExpressionNode]
     | [string, ExpressionNode, ExpressionNode, ObjectExpression];
 }
 
 export class DirectiveArgumentNode12 extends ArrayExpression {
   elements: // dir, exp, arg, modifiers
-  | [string]
+    | [string]
     | [string, ExpressionNode]
     | [string, ExpressionNode, ExpressionNode]
     | [string, ExpressionNode, ExpressionNode, ObjectExpression] = 1;
 }
 
-type A = // dir, exp, arg, modifiers
-
-    | [string]
-    | [string, ExpressionNode]
-    | [string, ExpressionNode, ExpressionNode]
-    | [string, ExpressionNode, ExpressionNode, ObjectExpression];
+type A =
+  // dir, exp, arg, modifiers
+  | [string]
+  | [string, ExpressionNode]
+  | [string, ExpressionNode, ExpressionNode]
+  | [string, ExpressionNode, ExpressionNode, ObjectExpression];
 
 const elements: // dir, exp, arg, modifiers
-| [string]
+  | [string]
   | [string, ExpressionNode]
   | [string, ExpressionNode, ExpressionNode]
   | [string, ExpressionNode, ExpressionNode, ObjectExpression] = 1;
 
 type A2 =
   /* block comment */
   | [string]
   | [string, ExpressionNode]
   | [string, ExpressionNode, ExpressionNode]
   | [string, ExpressionNode, ExpressionNode, ObjectExpression];
 
 type A3 =
   /* block comment
    */
   | [string]
   | [string, ExpressionNode]
   | [string, ExpressionNode, ExpressionNode]
   | [string, ExpressionNode, ExpressionNode, ObjectExpression];

```

**Prettier Similarity**: 79.07%


### typescript/union/consistent-with-flow/comment.ts
```diff
 type A1 = /* 4 */ (A | B)[];
 
-type A2 = // dir, exp, arg, modifiers
-
-    | [string]
-    | [string, ExpressionNode]
-    | [string, ExpressionNode, ExpressionNode]
-    | [string, ExpressionNode, ExpressionNode, ObjectExpression];
+type A2 =
+  // dir, exp, arg, modifiers
+  | [string]
+  | [string, ExpressionNode]
+  | [string, ExpressionNode, ExpressionNode]
+  | [string, ExpressionNode, ExpressionNode, ObjectExpression];
 
 type A3 = // dir, exp, arg, modifiers
   [string] &
     [string, ExpressionNode] &
     [string, ExpressionNode, ExpressionNode] &
     [string, ExpressionNode, ExpressionNode, ObjectExpression];
 
 type SuperLongTypeNameLoremIpsumLoremIpsumBlaBlaBlaBlaBlaBlaBlaBlaBlaBlaBlaBla =
-
-    | Fooo1000
-    | Baz2000
-    | BarLoooooooooooooooooooooooooooooooooooooooooooooooooLong;
+  | Fooo1000
+  | Baz2000
+  | BarLoooooooooooooooooooooooooooooooooooooooooooooooooLong;
 
 type SuperLongTypeNameLoremIpsumLoremIpsumBlaBlaBlaBlaBlaBlaBlaBlaBlaBlaBlaBl2 =
   Fooo1000 &
     Baz2000 &
     BarLoooooooooooooooooooooooooooooooooooooooooooooooooLong;

```

**Prettier Similarity**: 60.00%


### typescript/union/consistent-with-flow/prettier-ignore.ts
```diff
 export type a =
   // foo
   | (foo1 & foo2)
   // bar
   | (bar1 & bar2)
   // prettier-ignore
-  | (qux1&qux2);
+  | qux1&qux2;
 
 export type b =
   // foo
   | (foo1 & foo2)
   // bar
   | (bar1 & bar2)
   // prettier-ignore
-  | (qux1&qux2)
+  | qux1&qux2
   // baz
   | (baz1 & baz2);
 
 export type c =
   // prettier-ignore
-  | (foo1&foo2)
+  | foo1&foo2
   // bar
   | (bar1 & bar2)
   // qux
   | (qux1 & qux2);

```

**Prettier Similarity**: 88.00%


### typescript/union/inlining.ts
```diff
 interface RelayProps {
   articles: a | null;
 }
 interface RelayProps {
   articles: Array<{
     __id: string;
   } | null> | null | void;
 }
 
 interface RelayProps {
   articles:
     | Array<{
         __id: string;
       } | null>
     | null // articles type may be null
     | void; // articles type may be void
 }
 
 type FooBar =
   | null // null
   | {
-      /** x **/
-      y: number;
+      /** x **/ y: number;
       z: string;
     } // this documents the first option
   | void; // this documents the second option
 
 type FooBarWithoutComment = null | {
   y: number;
   z: string;
 } | void;
 
 type FooBar2 =
   | Number // this documents the first option
   | void; // this documents the second option
 
 type UploadState<E, EM, D> =
   // The upload hasnt begun yet
   | { type: "Not_begun" }
   // The upload timed out
   | { type: "Timed_out" }
   // Failed somewhere on the line
   | { type: "Failed"; error: E; errorMsg: EM }
   // Uploading to aws3 and CreatePostMutation succeeded
   | { type: "Success"; data: D };
 
 type UploadState2<E, EM, D> =
   // The upload hasnt begun yet
   | A
   // The upload timed out
   | B
   // Failed somewhere on the line
   | C
   // Uploading to aws3 and CreatePostMutation succeeded
   | D;
 
 type window = Window & {
   __REDUX_DEVTOOLS_EXTENSION_COMPOSE__: Function;
 };
 
 type T1 = (number | string)["toString"];
 type T2 = (number | string)["toString"];
 type T3 = (number | string)["toString"];
 type T4 = (number | string)["toString"];
 type T5 = number | ((arg: any) => void);
 type T6 = number | ((arg: any) => void);
 type T7 = number | ((arg: any) => void);
 type T8 = number | ((arg: any) => void);

```

**Prettier Similarity**: 97.06%


### typescript/union/single-type/single-type.ts
```diff
-type A1 /* 2 */ = /* 1 */ /* 3 */ /* 4 */ {
-  key: string;
-};
+type A1 =
+  /* 1 */ /* 2 */
+  /* 3 */ /* 4 */ {
+    key: string;
+  };

```

**Prettier Similarity**: 0.00%


### typescript/union/union-parens.ts
```diff
 export type A =
   | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
   | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type B =
   | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
   | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type C =
   | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
   | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type D =
   | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
   | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type Multi = (string | number)[];
 
 function f(): string | number {}
 
 var x: string | number;
 var y: string | number;
 
 class Foo<T extends string | number> {}
 
 interface Interface {
   i: (X | Y) & Z;
   j: Partial<X | Y>;
 }
 
 type State = {
   sharedProperty: any;
 } & (
   | { discriminant: "FOO"; foo: any }
   | { discriminant: "BAR"; bar: any }
   | { discriminant: "BAZ"; baz: any }
 );
 
 const foo1 = [abc, def, ghi, jkl, mno, pqr, stu, vwx, yz] as (
   | string
   | undefined
 )[];
 
 const foo2: (
   | AAAAAAAAAAAAAAAAAAAAAA
   | BBBBBBBBBBBBBBBBBBBBBB
   | CCCCCCCCCCCCCCCCCCCCCC
   | DDDDDDDDDDDDDDDDDDDDDD
 )[] = [];
 
 const foo3: keyof (
   | AAAAAAAAAAAAAAAAAAAAAA
   | BBBBBBBBBBBBBBBBBBBBBB
   | CCCCCCCCCCCCCCCCCCCCCC
   | DDDDDDDDDDDDDDDDDDDDDD
 ) = bar;
 
 const foo4:
   | foo
   | (
       | AAAAAAAAAAAAAAAAAAAAAA
       | BBBBBBBBBBBBBBBBBBBBBB
       | CCCCCCCCCCCCCCCCCCCCCC
       | DDDDDDDDDDDDDDDDDDDDDD
     ) = bar;
 
 let a1: C;
 let a2: C;
 let a3: C;
 let a4: C;
 let a5: C;
 let a6: /*1*/ C;
 let a7: /*1*/ C;
 let a8: /*1*/ C;
 let a9: /*1*/ C;
 let a10: /*1*/ /*2*/ C;
 let a11: /*1*/ /*2*/ C;
 
 let aa1: /*1*/ /*2*/ C | D;
 let aa2: /*1*/ /*2*/ C | /*3*/ D;
 let aa3: /*1*/ /*2*/ C | /*3*/ D /*4*/;
 
 type A1 = C;
 type A2 = C;
 type A3 = C;
 type A4 = C;
 type A5 = C;
-type A6 /*1*/ = C;
-type A7 /*1*/ = C;
-type A8 /*1*/ = C;
-type A9 /*1*/ = C;
-type A10 /*1*/ = /*2*/ C;
-type A11 /*1*/ = /*2*/ C;
-type A12 /*1*/ = C;
+type A6 = /*1*/ C;
+type A7 = /*1*/ C;
+type A8 = /*1*/ C;
+type A9 = /*1*/ C;
+type A10 = /*1*/ /*2*/ C;
+type A11 = /*1*/ /*2*/ C;
+type A12 = /*1*/ C;
 type A13 = /*1*/ C;
 
 type Aa1 = /*1*/ /*2*/ C | D;
 type Aa2 = /*1*/ /*2*/ C | /*3*/ D;
 type Aa3 = /*1*/ /*2*/ C | /*3*/ D /*4*/;
 
 type C1 = /*1*/ a | b;
 type C2 = /*1*/ a | b;
 type C3 = /*1*/ a | b;
-type C4 /*1*/ = a | b;
+type C4 = /*1*/ a | b;
 type C5 = /*1*/ a | b;
 type C6 /*0*/ = /*1*/ a | b;
 
 type Ctor = (new () => X) | Y;

```

**Prettier Similarity**: 92.59%


