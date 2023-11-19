# Overall Metrics

**Average compatibility**: 95.62

<details>
    <summary>Definition</summary>

    $$average = \frac\{\sum_{file}^\{files}compatibility_\{file}}\{files}$$
</details>

    **Compatible lines**: 96.05
<details>
    <summary>Definition</summary>

    $$average = \frac{\sum_{file}^{files}matching\_lines_{file}}{max(lines_{rome}, lines_{prettier})}$$
</details>

[Metric definition discussion](https://github.com/rome/tools/issues/2555#issuecomment-1124787893)
                

# js/arrays/issue-10159.js
```diff
 {
   for (const srcPath of [src, `${src}.js`, `${src}/index`, `${src}/index.js`]) {
   }
 }
 {
   for (const srcPath of [123, 123_123_123, 123_123_123_1, 13_123_3123_31_43]) {
   }
 }
 {
-  for (const srcPath of [123, 123_123_123, 123_123_123_1, 13_123_3123_31_432]) {
+  for (const srcPath of [
+    123, 123_123_123, 123_123_123_1, 13_123_3123_31_432,
+  ]) {
   }
 }
 {
   for (const srcPath of [
     123, 123_123_123, 123_123_123_1, 13_123_3123_31_4321,
   ]) {
   }
 }

```

**Prettier Similarity**: 85.00%


# js/arrays/numbers-negative.js
```diff
 const numbers1 = [
   -2017, -506252, -744011292, -7224, -70.4, -83353.6, -708.4, -174023963.52,
   -40385,
   // comment1
-  -380014, -253951682, -728, -15.84, -2058467564.56, -43, -33, -85134845,
-  -67092, -1, -78820379, -2371.6, -16, 7,
+  -380014,
+  -253951682, -728, -15.84, -2058467564.56, -43, -33, -85134845, -67092, -1,
+  -78820379, -2371.6, -16, 7,
   // comment2
-  -62454, -4282239912, -10816495.36, 0.88, -100622682, 8.8, -67087.68000000001,
+  -62454,
+  -4282239912, -10816495.36, 0.88, -100622682, 8.8, -67087.68000000001,
   -3758276, -25.5211, -54, -1184265243, -46073628, -280423.44, -41833463,
   -27961.12, -305.36, -199875.28,
 ];
 
 const numbers2 = [
   -234,
   -342, // comment3
   -223,
   -333333.33,
   12345,
 ];

```

**Prettier Similarity**: 76.19%


# js/arrays/numbers-with-holes.js
```diff
 const numberWithHoles1 = [
   7234932941,
   7234932722,
   7234932312,
+  // comment before a hole 1
   ,
-  // comment before a hole 1
   7234932841,
   ,
   7234932843,
-  ,
   // comment after a hole 1
+  ,
   7234932436,
 ];
 
 const numberWithHoles2 = [
   0x234932941,
   0x234932722,
   0x234932312,
-
+  // comment before a hole 2
   ,
-  // comment before a hole 2
   0x234932841,
   ,
   0x234932843,
-  ,
   // comment after a hole 2
+  ,
+
   0x234932436,
 ];

```

**Prettier Similarity**: 82.14%


# js/arrays/numbers-with-tricky-comments.js
```diff
 const lazyCatererNumbers = [
   1, 2, 4, 7, 11, 16, 22, 29, 37, 46, 56, 67, 79, 92, 106, 121, 137, 154, 172,
   191, 211, 232, 254, 277, 301, 326, 352, 379, 407, 436, 466 /*block*/,
   // line
-  497, 529, 562, 596, 631, 667, 704, 742, 781, 821, 862, 904, 947, 991, 1036,
-  1082, 1129, 1177, 1226,
+  497,
+  529, 562, 596, 631, 667, 704, 742, 781, 821, 862, 904, 947, 991, 1036, 1082,
+  1129, 1177, 1226,
   // line 2
-  1276, 1327, 1379,
+  1276,
+  1327, 1379,
 ];

```

**Prettier Similarity**: 54.55%


# js/arrows/chain-as-arg.js
```diff
 const w = a.b(
   (
-    c = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
-    d = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
-  ) =>
+      c = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
+      d = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
+    ) =>
     (e) =>
       0,
 );
 
 const x = a.b(
   (
-    c = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
-    d = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
-  ) =>
+      c = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
+      d = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
+    ) =>
     (e) =>
       0,
 )(x);
 
 const y = a.b(
   1,
   (
-    c = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
-    d = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
-  ) =>
+      c = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
+      d = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
+    ) =>
     (e) =>
       0,
 )(x);
 
 const z = a.b(
   (
-    c = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
-    d = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
-  ) =>
+      c = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
+      d = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
+    ) =>
     (e) =>
       0,
   2,
 )(x);

```

**Prettier Similarity**: 67.57%


# js/arrows/chain-in-logical-expression.js
```diff
 const x =
   a.b ??
   ((
-    c = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
-    d = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
-  ) =>
+      c = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
+      d = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef",
+    ) =>
     (e) =>
       0);

```

**Prettier Similarity**: 62.50%


# js/arrows/comment.js
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
-  () =>
-    // comment
-    a,
+  () => a, // comment
 );
 func(
-  () => () =>
-    // comment
-    a,
+  () =>
+    () => // comment
+      a,
 );
 func(
-  () => () => () =>
-    // comment
-    a,
+  () =>
+    () =>
+    () => // comment
+      a,
 );
 
-func(() =>
-  // comment
-  a ? b : c,
+func(
+  () => (a ? b : c), // comment
 );
 func(
-  () => () =>
-    // comment
-    a ? b : c,
+  () =>
+    () => // comment
+      a ? b : c,
 );
 func(
-  () => () => () =>
-    // comment
-    a ? b : c,
+  () =>
+    () =>
+    () => // comment
+      a ? b : c,
 );
 
 func(
-  () =>
-    (
-      // comment
-      a, b, c
-    ),
+  () => ( // comment
+    a, b, c
+  ),
 );
 func(
-  () => () =>
-    (
-      // comment
+  () =>
+    () => ( // comment
       a, b, c
     ),
 );
 func(
-  () => () => () =>
-    (
-      // comment
+  () =>
+    () =>
+    () => ( // comment
       a, b, c
     ),
 );

```

**Prettier Similarity**: 65.48%


# js/arrows/curried.js
```diff
 const fn1 = (a) => 3;
 const fn2 = (a) => (b) => 3;
 const fn3 = (a) => (b) => (c) => 3;
 const fn4 = (a) => (b) => (c) => (d) => 3;
 const fn5 = (a) => (b) => (c) => (d) => (e) => 3;
 const fn6 = (a) => (b) => (c) => (d) => (e) => (g) => 3;
 const fn7 = (a) => (b) => (c) => (d) => (e) => (g) => (f) => 3;
 
 const fn8 = (a) => ({ foo: bar, bar: baz, baz: foo });
 const fn9 = (a) => (b) => ({ foo: bar, bar: baz, baz: foo });
 const fn10 = (a) => (b) => (c) => ({ foo: bar, bar: baz, baz: foo });
 const fn11 = (a) => (b) => (c) => (d) => ({ foo: bar, bar: baz, baz: foo });
 const fn12 = (a) => (b) => (c) => (d) => (e) => ({
   foo: bar,
   bar: baz,
   baz: foo,
 });
 const fn13 = (a) => (b) => (c) => (d) => (e) => (g) => ({
   foo: bar,
   bar: baz,
   baz: foo,
 });
 const fn14 = (a) => (b) => (c) => (d) => (e) => (g) => (f) => ({
   foo: bar,
   bar: baz,
   baz: foo,
 });
 
 const curryTest =
   (argument1) =>
   (argument2) =>
   (argument3) =>
   (argument4) =>
   (argument5) =>
   (argument6) =>
   (argument7) =>
   (argument8) =>
   (argument9) =>
   (argument10) =>
   (argument11) =>
   (argument12) => ({
     foo: argument1,
     bar: argument2,
   });
 
 let curryTest2 =
   (argument1) =>
   (argument2) =>
   (argument3) =>
   (argument4) =>
   (argument5) =>
   (argument6) =>
   (argument7) =>
   (argument8) =>
   (argument9) =>
   (argument10) =>
   (argument11) =>
   (argument12) => {
     const foo = "foo";
     return foo + "bar";
   };
 
 curryTest2 =
   (argument1) =>
   (argument2) =>
   (argument3) =>
   (argument4) =>
   (argument5) =>
   (argument6) =>
   (argument7) =>
   (argument8) =>
   (argument9) =>
   (argument10) =>
   (argument11) =>
   (argument12) => {
     const foo = "foo";
     return foo + "bar";
   };
 
 throw (argument1) =>
   (argument2) =>
   (argument3) =>
   (argument4) =>
   (argument5) =>
   (argument6) =>
   (argument7) =>
   (argument8) =>
   (argument9) =>
   (argument10) =>
   (argument11) =>
   (argument12) => {
     const foo = "foo";
     return foo + "bar";
   };
 
 foo(
   (argument1) =>
     (argument2) =>
     (argument3) =>
     (argument4) =>
     (argument5) =>
     (argument6) =>
     (argument7) =>
     (argument8) =>
     (argument9) =>
     (argument10) =>
     (argument11) =>
     (argument12) =>
       3,
 );
 
-foo(
-  (argument1) =>
-    (argument2) =>
-    (argument3) =>
-    (argument4) =>
-    (argument5) =>
-    (argument6) =>
-    (argument7) =>
-    (argument8) =>
-    (argument9) =>
-    (argument10) =>
-    (argument11) =>
-    (argument12) => ({
-      foo: bar,
-      bar: baz,
-      baz: foo,
-    }),
-);
+foo((argument1) =>
+  (argument2) =>
+  (argument3) =>
+  (argument4) =>
+  (argument5) =>
+  (argument6) =>
+  (argument7) =>
+  (argument8) =>
+  (argument9) =>
+  (argument10) =>
+  (argument11) =>
+  (argument12) => ({
+    foo: bar,
+    bar: baz,
+    baz: foo,
+  }));
 
-foo(
-  (argument1) =>
-    (argument2) =>
-    (argument3) =>
-    (argument4) =>
-    (argument5) =>
-    (argument6) =>
-    (argument7) =>
-    (argument8) =>
-    (argument9) =>
-    (argument10) =>
-    (argument11) =>
-    (argument12) => {
-      const foo = "foo";
-      return foo + "bar";
-    },
-);
+foo((argument1) =>
+  (argument2) =>
+  (argument3) =>
+  (argument4) =>
+  (argument5) =>
+  (argument6) =>
+  (argument7) =>
+  (argument8) =>
+  (argument9) =>
+  (argument10) =>
+  (argument11) =>
+  (argument12) => {
+    const foo = "foo";
+    return foo + "bar";
+  });
 
 (
   (argument1) =>
   (argument2) =>
   (argument3) =>
   (argument4) =>
   (argument5) =>
   (argument6) =>
   (argument7) =>
   (argument8) =>
   (argument9) =>
   (argument10) =>
   (argument11) =>
   (argument12) =>
     3
 )(3);
 
 bar(
-  foo(
-    (argument1) =>
-      (argument2) =>
-      (argument3) =>
-      (argument4) =>
-      (argument5) =>
-      (argument6) =>
-      (argument7) =>
-      (argument8) =>
-      (argument9) =>
-      (argument10) =>
-      (argument11) =>
-      (argument12) => ({
-        foo: bar,
-        bar: baz,
-      }),
-  ),
+  foo((argument1) =>
+    (argument2) =>
+    (argument3) =>
+    (argument4) =>
+    (argument5) =>
+    (argument6) =>
+    (argument7) =>
+    (argument8) =>
+    (argument9) =>
+    (argument10) =>
+    (argument11) =>
+    (argument12) => ({
+      foo: bar,
+      bar: baz,
+    })),
 );
 
 const baaaz =
   (aaaaa1, bbbbb1) =>
   (aaaaa2, bbbbb2) =>
   (aaaaa3, bbbbb3) =>
   (aaaaa4, bbbbb4) => ({
     foo: bar,
   });
 
 new Fooooooooooooooooooooooooooooooooooooooooooooooooooo(
   (action) => (next) => (next) => (next) => (next) => (next) => (next) =>
     dispatch(action),
 );
 
 foo?.Fooooooooooooooooooooooooooooooooooooooooooooooooooo(
   (action) => (next) => (next) => (next) => (next) => (next) => (next) =>
     dispatch(action),
 );
 
 foo((action) => (action) => action);
 
-import(
-  (argument1) =>
-    (argument2) =>
-    (argument3) =>
-    (argument4) =>
-    (argument5) =>
-    (argument6) =>
-    (argument7) =>
-    (argument8) =>
-    (argument9) =>
-    (argument10) =>
-    (argument11) =>
-    (argument12) => {
-      const foo = "foo";
-      return foo + "bar";
-    }
-);
+import((argument1) =>
+  (argument2) =>
+  (argument3) =>
+  (argument4) =>
+  (argument5) =>
+  (argument6) =>
+  (argument7) =>
+  (argument8) =>
+  (argument9) =>
+  (argument10) =>
+  (argument11) =>
+  (argument12) => {
+    const foo = "foo";
+    return foo + "bar";
+  });

```

**Prettier Similarity**: 68.78%


# js/arrows/currying-2.js
```diff
 const a = (x) => (y) => (z) =>
   x / 0.123456789 + (y * calculateSomething(z)) / Math.PI;
 
 request.get("https://preview-9992--prettier.netlify.app", (head) => (body) => {
   console.log(head, body);
 });
 
-request.get(
-  "https://preview-9992--prettier.netlify.app",
-  (head) => (body) => (mody) => {
+request.get("https://preview-9992--prettier.netlify.app", (head) =>
+  (body) =>
+  (mody) => {
     console.log(head, body);
-  },
-);
+  });
 
-request.get(
-  "https://preview-9992--prettier.netlify.app",
-  (head) =>
-    (body) =>
-    (modyLoremIpsumDolorAbstractProviderFactoryServiceModule) => {
-      console.log(head, body);
-    },
-);
+request.get("https://preview-9992--prettier.netlify.app", (head) =>
+  (body) =>
+  (modyLoremIpsumDolorAbstractProviderFactoryServiceModule) => {
+    console.log(head, body);
+  });

```

**Prettier Similarity**: 40.91%


# js/arrows/currying-4.js
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
 
-const x2 = () => () => [
-  "The",
-  "green",
-  "dragon",
-  "liked",
-  "to",
-  "knit",
-  "sweaters",
-  "for",
-  "the",
-  "fluffy",
-  "clouds",
-  "in",
-  "the",
-  "sky.",
-];
+const x2 = () => () =>
+  [
+    "The",
+    "green",
+    "dragon",
+    "liked",
+    "to",
+    "knit",
+    "sweaters",
+    "for",
+    "the",
+    "fluffy",
+    "clouds",
+    "in",
+    "the",
+    "sky.",
+  ];
 
-const x3 = () => () => () => [
-  "The",
-  "green",
-  "dragon",
-  "liked",
-  "to",
-  "knit",
-  "sweaters",
-  "for",
-  "the",
-  "fluffy",
-  "clouds",
-  "in",
-  "the",
-  "sky.",
-];
+const x3 = () => () => () =>
+  [
+    "The",
+    "green",
+    "dragon",
+    "liked",
+    "to",
+    "knit",
+    "sweaters",
+    "for",
+    "the",
+    "fluffy",
+    "clouds",
+    "in",
+    "the",
+    "sky.",
+  ];
 
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

**Prettier Similarity**: 67.57%


# js/arrows/issue-1389-curry.js
```diff
 const foobar =
   (argumentOne, argumentTwo, argumentThree) =>
   (...restOfTheArguments) => {
     return "baz";
   };
 
 const foobaz =
   (argumentOne, argumentTwo, argumentThree) => (restOfTheArguments123, j) => {
     return "baz";
   };
 
-const makeSomeFunction =
-  (services = { logger: null }) =>
-  (a, b, c) =>
-    services.logger(a, b, c);
+const makeSomeFunction = (services = { logger: null }) => (a, b, c) =>
+  services.logger(a, b, c);
 
 const makeSomeFunction2 =
   (
     services = {
       logger: null,
     },
   ) =>
   (a, b, c) =>
     services.logger(a, b, c);

```

**Prettier Similarity**: 83.33%


# js/arrows/newline-before-arrow/newline-before-arrow.js
```diff
-async (x) => x;
+async;
+x;
+=> x

```

**Prettier Similarity**: 0.00%


# js/chain-expression/test.js
```diff
-(a?.b).c;
-(a?.()).b;
+a?.b.c;
+a?.().b;
 
-(a?.b)();
-(a?.())();
+a?.b();
+a?.()();
 
 new (a?.b)();
 new (a?.())();

```

**Prettier Similarity**: 50.00%


# js/comments-closure-typecast/satisfies.js
```diff
-module.exports = /** @satisfies {Record<string, string>} */ ({
+module.exports = /** @satisfies {Record<string, string>} */ {
   hello: 1337,
-});
+};

```

**Prettier Similarity**: 33.33%


# js/comments-closure-typecast/styled-components.js
```diff
 const OverlapWrapper =
   /** @type {import('styled-components').ThemedStyledFunction<'div',null,{overlap: boolean}>} */
   (styled.div)`
-    position: relative;
+position:relative;
     > {
-      position: absolute;
-      bottom: ${(p) => p.overlap === "previous" && 0};
-      top: ${(p) => p.overlap === "next" && 0};
-    }
-  `;
+  position: absolute;
+  bottom: ${(p) => p.overlap === "previous" && 0};
+top: ${(p) => p.overlap === "next" && 0};
+}
+`;

```

**Prettier Similarity**: 40.00%


# js/comments/empty-statements.js
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


# js/comments/export.js
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


# js/comments/function/between-parentheses-and-function-body.js
```diff
 function function_declaration() {
   // this is a function
   return 42;
 }
 
-(function named() {
-  // this is a function
-  return 42;
-})();
+// FIXME
+// TODO: reformat issue
+// (function named()
+// // this is a function
+// {
+//   return 42
+// })();
 
-(function () {
-  // this is a function
-  return 42;
-})();
+// FIXME
+// TODO: reformat issue
+// (function ()
+// // this is a function
+// {
+//   return 42
+// })();
 
 /* anonymous declaration */
 export default function () {
   // this is a function
   return 42;
 }
 
+// FIXME
+// TODO: reformat issue
 a = {
   foo() {
     // this is a function
   },
 
-  bar: function () {
-    // this is a function
-  },
+  // bar: function()
+  // // this is a function
+  // {},
 };

```

**Prettier Similarity**: 50.00%


# js/comments/html-like/comment.js
```diff
 <!--
-alert(1); 
+alert(1)
 -->

```

**Prettier Similarity**: 66.67%


# js/comments/jsdoc-nestled-dangling.js
```diff
 {
   {
     {
       {
         {
           {
             {
               o = {
                 /**
                  * A
-                 *//**
+                 */
+                /**
                  * B
                  */
               };
             }
           }
         }
       }
     }
   }
 }

```

**Prettier Similarity**: 90.91%


# js/comments/jsdoc-nestled.js
```diff
 const issues = {
   see: "#7724 and #12653",
-  /** Trailing comment 1 (not nestled as both comments should be multiline for that) */ /**
+  /** Trailing comment 1 (not nestled as both comments should be multiline for that) */
+  /**
    * Trailing comment 2
    */
 };
 
 /**
  * @template T
  * @param {Type} type
  * @param {T} value
  * @return {Value}
- *//**
+ */ /**
  * @param {Type} type
  * @return {Value}
  */
 function value(type, value) {
   if (arguments.length === 2) {
     return new ConcreteValue(type, value);
   } else {
     return new Value(type);
   }
 }
 
 /** Trailing nestled comment 1
- *//** Trailing nestled comment 2
- *//** Trailing nestled comment 3
  */
+/** Trailing nestled comment 2
+ */
+/** Trailing nestled comment 3
+ */

```

**Prettier Similarity**: 77.42%


# js/comments/multi-comments-on-same-line.js
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


# js/comments/return-statement.js
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
     a, b
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
 
-function multilineBlockSameLine() {
-  return (
-    /**
-     * @type {string}
-     */ "result"
-  );
-}
+// TODO: fix idempotency issue
+// function multilineBlockSameLine() {
+//   return (
+//     /**
+//     * @type {string}
+//     */ 'result'
+//   )
+// }
 
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

**Prettier Similarity**: 91.67%


# js/comments/tagged-template-literal.js
```diff
 foo``; // comment
 
 foo // comment
 ``;
 
 foo // comment
 `
 `;
 
-foo/* comment */ `
+foo /* comment */`
 `;
 
-foo/* comment */ `
+foo /* comment */`
 `;

```

**Prettier Similarity**: 85.71%


# js/comments/trailing-jsdocs.js
```diff
 const CONNECTION_STATUS = (exports.CONNECTION_STATUS = {
   CLOSED: Object.freeze({ kind: "CLOSED" }),
   CONNECTED: Object.freeze({ kind: "CONNECTED" }),
   CONNECTING: Object.freeze({ kind: "CONNECTING" }),
   NOT_CONNECTED: Object.freeze({ kind: "NOT_CONNECTED" }),
 });
 
-/* A comment */ /**
+/* A comment */
+/**
  * A type that can be written to a buffer.
- */ /**
+ */
+/**
  * Describes the connection status of a ReactiveSocket/DuplexConnection.
  * - NOT_CONNECTED: no connection established or pending.
  * - CONNECTING: when `connect()` has been called but a connection is not yet
  *   established.
  * - CONNECTED: when a connection is established.
  * - CLOSED: when the connection has been explicitly closed via `close()`.
  * - ERROR: when the connection has been closed for any other reason.
- */ /**
+ */
+/**
  * A contract providing different interaction models per the [ReactiveSocket protocol]
  * (https://github.com/ReactiveSocket/reactivesocket/blob/master/Protocol.md).
- */ /**
+ */
+/**
  * A single unit of data exchanged between the peers of a `ReactiveSocket`.
  */

```

**Prettier Similarity**: 70.37%


# js/conditional/comments.js
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
+  test
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


# js/conditional/postfix-ternary-regressions.js
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


# js/destructuring/destructuring.js
```diff
 const [one, two = null, three = null] = arr;
 a = ([s = 1]) => 1;
 const { children, ...props } = this.props;
 
 const {
   user: { firstName, lastName },
 } = this.props;
 
 const {
   name: { first, last },
   organisation: {
     address: { street: orgStreetAddress, postcode: orgPostcode },
   },
 } = user;
 
 function f({ data: { name } }) {}
 
 const UserComponent = function ({
   name: { first, last },
   organisation: {
     address: { street: orgStreetAddress, postcode: orgPostcode },
   },
 }) {
   return;
 };
 
 const {
   a,
   b,
   c,
   d: { e },
 } = someObject;
 
 try {
   // code
 } catch ({ data: { message } }) {
   // code
 }
 
 try {
   // code
-} catch ({
-  data: {
-    message: { errors },
-  },
-}) {
+} catch ({ data: { message: { errors } } }) {
   // code
 }
 
 const obj = {
   func(id, { blog: { title } }) {
     return id + title;
   },
 };
 
 class A {
   func(id, { blog: { title } }) {
     return id + title;
   }
 }

```

**Prettier Similarity**: 91.67%


# js/explicit-resource-management/valid-await-using-binding-escaped.js
```diff
 async function f() {
-  await using ab = c;
+  await using \u0061b = c;
 }

```

**Prettier Similarity**: 66.67%


# js/explicit-resource-management/valid-await-using-comments.js
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


# js/explicit-resource-management/valid-using-binding-escaped.js
```diff
 {
-  using ab = c;
+  using \u0061b = c;
 }

```

**Prettier Similarity**: 66.67%


# js/export-default/escaped/default-escaped.js
```diff
 // export asyn\u{63} from "async";
-export nc from "async";
+export
+n\u{63};
+from;
+("async");

```

**Prettier Similarity**: 20.00%


# js/export/blank-line-between-specifiers.js
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


# js/for/continue-and-break-comment-without-blocks.js
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
 
 label1: for (;;) continue label1 /* comment */;
 
-label1: for (;;) continue label1;
-/* comment */
+// FIXME: TODO: reformat issue
+// label1: for (;;) continue label1
+// /* comment */
+// ;
 
-label1: for (;;) continue label1; // comment
+// label1: for (;;) continue label1 // comment
+// ;
 
-label1: for (;;) continue label1;
-// comment
+// label1: for (;;) continue label1
+// // comment
+// ;

```

**Prettier Similarity**: 87.67%


# js/identifier/for-of/let.js
```diff
 for ((let) of foo);
 for (foo of let);
 for (foo of let.a);
 for (foo of let[a]);
-for ((let).a of foo);
-for ((let)[a] of foo);
+for (let.a of foo);
+for (let[a] of foo);
 for ((let)().a of foo);
 for (letFoo of foo);
 
 for (let.a in foo);
-for ((let)[a] in foo);
+for (let[a] in foo);
 
 for (let of of let);

```

**Prettier Similarity**: 76.92%


# js/identifier/parentheses/const.js
```diff
-const [a = (let[0] = 1)] = 2;
+const [a = ((let)[0] = 1)] = 2;

```

**Prettier Similarity**: 0.00%


# js/identifier/parentheses/let.js
```diff
 let.a = 1;
 
 let.a[0] = 1;
 
-(let)[a] = 1;
+let[a] = 1;
 
-(let)[a].b.c.e = 1;
+let[a].b.c.e = 1;
 
 foo[let[a]] = 1;
 
 (let)[let[a]] = 1;
 
-(let)[a] ??= 1;
+let[a] ??= 1;
 
 foo = let[a];
 
 let()[a] = 1;
 
 foo(let)[a] = 1;
 
 foo(let[a])[a] = 1;
 
-(let)[0] = 1;
+let[0] = 1;
 
-(let)["a"] = 1;
+let["a"] = 1;
 
 let = 1;
 
 var let = 1;
 
 [let[a]] = 1;
 
 ({ a: let[a] } = 1);
 
 alert((let[0] = 1));
 
-((let)[0] = 1) || 2;
+(let[0] = 1) || 2;
 
-((let)[0] = 1), 2;
+(let[0] = 1), 2;
 
-((let)[0] = 1) ? a : b;
+(let[0] = 1) ? a : b;
 
 if ((let[0] = 1));
 
 while ((let[0] = 1));
 
 do {} while ((let[0] = 1));
 
 var a = (let[0] = 1);
 
-((let)[0] = 1) instanceof a;
+(let[0] = 1) instanceof a;
 
 void (let[0] = 1);
 
-((let)[0] = 1)();
+(let[0] = 1)();
 
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
 
-while (true) (let)[0] = 1;
+while (true) let[0] = 1;
 
 throw (let[0] = 1);
 
 ({ foo: (let[0] = 1) });
 
 [(let[0] = 1)];
 
-for ((let)[0] = 1; ; );
+for (let[0] = 1; ; );
 for ((let)[0] in {});
 for ((let)[0] of []);
 
 switch ((let[0] = 1)) {
 }
 
 switch (foo) {
   case (let[0] = 1):
 }
 
-with ((let[0] = 1));
+with (let[0] = 1);
 
-(let)[x].foo();
+let[x].foo();
 
 let.let[x].foo();
 
 a = let[x].foo();
 
 (let)[2];
 
 a[1] + (let[2] = 2);

```

**Prettier Similarity**: 87.27%


# js/if/expr_and_same_line_comments.js
```diff
 if (a === 0) doSomething(); // comment A1
 else if (a === 1) doSomethingElse(); // comment B1
 else if (a === 2) doSomethingElse(); // comment C1
 
 if (a === 0) doSomething(); /* comment A2 */
 else if (a === 1) doSomethingElse(); /* comment B2 */
 else if (a === 2) doSomethingElse(); /* comment C2 */
 
 if (a === 0) expr; // comment A3
 else if (a === 1) expr; // comment B3
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
   if (a) return /* comment 6a */;
   else return 2;
 
   if (a) return 1 /* comment 6b */;
   else return 2;
 
   if (a) throw e /* comment 6d */;
   else return 2;
 
   // TODO[@fisker]: fix this
   // if (a) var a = 1; /* comment 6e */
   // else return 2;
 
   if (a)
-    if (b /* comment 6f */);
+    if (b);/* comment 6f */
     else return 2;
 }

```

**Prettier Similarity**: 97.56%


# js/ignore/class-expression-decorator.js
```diff
-// prettier-ignore
 (
+  // prettier-ignore
   @decorator
   class {}
 );

```

**Prettier Similarity**: 80.00%


# js/ignore/issue-11077.js
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


# js/ignore/issue-13737.js
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


# js/ignore/issue-14404.js
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


# js/ignore/issue-9877.js
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


# js/last-argument-expansion/dangling-comment-in-arrow-function.js
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


# js/last-argument-expansion/function-body-in-mode-break.js
```diff
 fs.readdirSync(suiteLoc).forEach(function (testName) {
-  (skip ? it.skip : it)(
-    testName,
-    buildTest(binName, testName, opts),
-    2_000_000,
-  );
+  (skip
+    ? it.skip
+    : it)(testName, buildTest(binName, testName, opts), 2_000_000);
 });
 
 {
   (skip ? it.skip : it)(
     testName,
     buildTest(binName, testName, opts),
     2_000_000,
   );
 }

```

**Prettier Similarity**: 66.67%


# js/objects/assignment-expression/object-property.js
```diff
 a = {
-  [(this.resource = resource)]: 1,
+  [this.resource = resource]: 1,
 };

```

**Prettier Similarity**: 66.67%


# js/optional-chaining-assignment/valid-parenthesized.js
```diff
-a?.b = c;
+(a?.b) = c;

```

**Prettier Similarity**: 0.00%


# js/optional-chaining/chaining.js
```diff
 var street = user.address?.street;
 var fooValue = myForm.querySelector("input[name=foo]")?.value;
 
 obj?.prop;
 obj?.[expr];
 func?.(...args);
 
 a?.();
 a?.[++x];
 a?.b.c(++x).d;
 a?.b[3].c?.(x).d;
 a?.b.c;
-(a?.b).c;
+a?.b.c;
 a?.b?.c;
 delete a?.b;
 
 a?.b[3].c?.(x).d.e?.f[3].g?.(y).h;
 
-(a?.b).c();
-(a?.b[c]).c();
+a?.b.c();
+a?.b[c].c();
 
 a?.b?.c.d?.e;
 (a ? b : c)?.d;
 
 (list || list2)?.length;
 (list || list2)?.[list || list2];
 
 async function HelloWorld() {
   var x = (await foo.bar.blah)?.hi;
   a?.[await b];
   (await x)?.();
 }
 
 a[b?.c].d();
 a?.[b?.c].d();
 a[b?.c]?.d();
 a?.[b?.c]?.d();
 
 one?.fn();
-(one?.two).fn();
-(one?.two)();
-(one?.two())();
+one?.two.fn();
+one?.two();
+one?.two()();
 one.two?.fn();
-(one.two?.three).fn();
+one.two?.three.fn();
 one.two?.three?.fn();
 
 one?.();
-(one?.())();
+one?.()();
 one?.()?.();
 
-(one?.()).two;
+one?.().two;
 
 a?.[b ? c : d];
 
 (-1)?.toFixed();
 (void fn)?.();
 (a && b)?.();
 (a ? b : c)?.();
 (function () {})?.();
 (() => f)?.();
 (() => f)?.x;
-(a?.(x)).x;
+a?.(x).x;
 (
   aaaaaaaaaaaaaaaaaaaaaaaa &&
   aaaaaaaaaaaaaaaaaaaaaaaa &&
   aaaaaaaaaaaaaaaaaaaaaaaa
 )?.();
 
 let f = () => ({})?.();
 let g = () => ({})?.b;
 a = () => ({})?.() && a;
 a = () => ({})?.()() && a;
 a = () => ({})?.().b && a;
 a = () => ({})?.b && a;
 a = () => ({})?.b() && a;
 (a) => ({})?.()?.b && 0;
 (a) => ({})?.b?.b && 0;
 (x) => ({})?.()();
 (x) => ({})?.().b;
 (x) => ({})?.b();
 (x) => ({})?.b.b;
 ({})?.a().b();
 ({ a: 1 })?.entries();
 
 new (foo?.bar)();
 new (foo?.bar())();
 new (foo?.())();

```

**Prettier Similarity**: 88.51%


# js/quotes/objects.js
```diff
 const obj = {
   a: true,
   b: true,
-  "𐊧": true,
+  𐊧: true,
 };

```

**Prettier Similarity**: 80.00%


# js/range/boundary-2.js
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


# js/range/boundary-3.js
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


# js/range/boundary.js
```diff
-foo = 1.0000;bar = 1.0;
-baz = 1.0;
+foo = 1.0000;bar = 1.0;baz=1.0000;
 // The range will be 13~26
 // `foo` ends at 13, should not format
 // `bar` ends at 26, should format

```

**Prettier Similarity**: 60.00%


# js/range/class-declaration.js
```diff
 
 
 class a {
   b() {}
 }
 
-let x;
+let    x

```

**Prettier Similarity**: 85.71%


# js/range/multiple-statements2.js
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


# js/range/nested3.js
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


# js/return-outside-function/return-outside-function.js
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


# js/sequence-expression/ignore.js
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


# js/sequence-expression/parenthesized.js
```diff
-console.log(
-  /* 1 */
-  /* 2 */
-  /* 3 */
-  (first,
-  /* 4 */
-  /* 5 */
-  /* 6 */
-  /* 7 */
-  last),
-  /* 8 */
-  /* 9 */
-  /* 10 */
-);
+// FIXME
+// TODO: parse issue
+// console.log(
+//   /* 1 */
+//   (
+//     /* 2 */
+//     (
+//       /* 3 */
+//       first
+//       /* 4 */
+//     )
+//     /* 5 */
+//     ,
+//     /* 6 */
+//     (
+//       /* 7 */
+//       last
+//       /* 8 */
+//     )
+//     /* 9 */
+//   )
+//   /* 10 */
+// );

```

**Prettier Similarity**: 0.00%


# js/sloppy-mode/function-declaration-in-if.js
```diff
-if (false) function foo() {}
+if (false) function foo(){}

```

**Prettier Similarity**: 0.00%


# js/sloppy-mode/function-declaration-in-while.js
```diff
-while (false) function foo() {}
+while (false) function foo(){}

```

**Prettier Similarity**: 0.00%


# js/strings/escaped.js
```diff
+// FIXME
+// TODO: reformat issue
 export const MSG_GENERIC_OPERATION_FAILURE_BODY_1 =
   goog.getMsg("That's all we know");
 
-export const MSG_GENERIC_OPERATION_FAILURE_BODY_2 = goog.getMsg(
-  "That's all we know",
-);
+// FIXME
+// TODO: reformat issue
+// export const MSG_GENERIC_OPERATION_FAILURE_BODY_2 =
+//   goog.getMsg("That\'s all we know");

```

**Prettier Similarity**: 33.33%


# js/switch/comments2.js
```diff
 switch (1) {
   default: // comment1
 }
 
 switch (2) {
   default: // comment2
   //comment2a
 }
 
 switch (3) {
   default: // comment3
     break; // comment3a
 }
 
 switch (4) {
   default: // comment4
     // comment4a
     break; // comment4b
 }
 
-switch (5) {
-  default: // comment5
-    // comment5a
-    foo();
-    bar(); //comment5b
-    break; // comment5c
-}
+// FIXME
+// TODO: reformat issue
+// switch(5){default: // comment5
+// // comment5a
+// foo();bar();//comment5b
+// break;// comment5c
+// }

```

**Prettier Similarity**: 74.07%


# js/template-literals/indention.js
```diff
 [
   `
       1. Go to "-{chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer"
       )}" ${chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer",
       )}
   `,
   `
       2. Go to "${chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer",
       )}" ${chalk.green.underline(
-        "https://www.example.com/drupedalKangarooTransformer",
-      )}
+    "https://www.example.com/drupedalKangarooTransformer",
+  )}
   `,
   `
       1. Go to "-{chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer"
       )}" ${chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer",
       )}
       2. Go to "${chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer",
       )}" ${chalk.green.underline(
-        "https://www.example.com/drupedalKangarooTransformer",
-      )}
+    "https://www.example.com/drupedalKangarooTransformer",
+  )}
   `,
   `
       2. Go to "${chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer",
       )}" ${chalk.green.underline(
-        "https://www.example.com/drupedalKangarooTransformer",
-      )}
+    "https://www.example.com/drupedalKangarooTransformer",
+  )}
       1. Go to "-{chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer"
       )}" ${chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer",
       )}
   `,
   `
           1. Go to "-{chalk.green.underline(
             "https://www.example.com/drupedalKangarooTransformer"
           )}" ${chalk.green.underline(
             "https://www.example.com/drupedalKangarooTransformer",
           )}
       2. Go to "${chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer",
       )}" ${chalk.green.underline(
-        "https://www.example.com/drupedalKangarooTransformer",
-      )}
+    "https://www.example.com/drupedalKangarooTransformer",
+  )}
   `,
   `
       1. Go to "-{chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer"
       )}" ${chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer",
       )}
           2. Go to "${chalk.green.underline(
             "https://www.example.com/drupedalKangarooTransformer",
           )}" ${chalk.green.underline(
-            "https://www.example.com/drupedalKangarooTransformer",
-          )}
+    "https://www.example.com/drupedalKangarooTransformer",
+  )}
   `,
   `
 # blabla ${a} ${chalk.green.underline(
     "https://www.example.com/drupedalKangarooTransformer",
   )}
 
     2. Go to "${chalk.green.underline(
       "https://www.example.com/drupedalKangarooTransformer",
     )}"
 
 # blabla ${a} ${chalk.green.underline(
     "https://www.example.com/drupedalKangarooTransformer",
   )}
 `,
   `
   # blabla ${a} ${chalk.green.underline(
     "https://www.example.com/drupedalKangarooTransformer",
   )}
 
       2. Go to "${chalk.green.underline(
         "https://www.example.com/drupedalKangarooTransformer",
       )}"
 
   # blabla ${a} ${chalk.green.underline(
     "https://www.example.com/drupedalKangarooTransformer",
   )}
   `,
 ];

```

**Prettier Similarity**: 88.89%


# js/ternaries/func-call.js
```diff
 fn(
   bifornCringerMoshedPerplexSawder,
   askTrovenaBeenaDependsRowans,
   glimseGlyphsHazardNoopsTieTie === averredBathersBoxroomBuggyNurl &&
-    anodyneCondosMalateOverateRetinol
+  anodyneCondosMalateOverateRetinol
     ? annularCooeedSplicesWalksWayWay
     : kochabCooieGameOnOboleUnweave,
 );
 // TODO(rattrayalex): try to indent consequent/alternate here.

```

**Prettier Similarity**: 88.89%


# js/test-declarations/angularjs_inject.js
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


# js/with/indent.js
```diff
-with (0) {
-}
+with (0) {}
 
 with (0) 1;

```

**Prettier Similarity**: 50.00%


# jsx/comments/in-attributes.js
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


# jsx/comments/in-end-tag.js
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


# jsx/cursor/in-jsx-text.js
```diff
 <>
-  a<div>hi</div>
+  a
+  <div>hi</div>
 </>;

```

**Prettier Similarity**: 50.00%


# jsx/fbt/test.js
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


# jsx/jsx/arrow.js
```diff
 () => (
   <Component>
     <Children />
   </Component>
 );
-() => () => (
-  <Component>
-    <Children />
-  </Component>
-);
-() => () => () => (
-  <Component>
-    <Children />
-  </Component>
-);
+() => () =>
+  (
+    <Component>
+      <Children />
+    </Component>
+  );
+() => () => () =>
+  (
+    <Component>
+      <Children />
+    </Component>
+  );
 
 () => <div>Some text here</div>;
 () => () => <div>Some text here</div>;
 () => () => () => <div>Some text here</div>;
 
 () => (
   <div>
     Long long long long long, very very long text. And more text. Another text.
   </div>
 );
-() => () => (
-  <div>
-    Long long long long long, very very long text. And more text. Another text.
-  </div>
-);
-() => () => () => (
-  <div>
-    Long long long long long, very very long text. And more text. Another text.
-  </div>
-);
+() => () =>
+  (
+    <div>
+      Long long long long long, very very long text. And more text. Another
+      text.
+    </div>
+  );
+() => () => () =>
+  (
+    <div>
+      Long long long long long, very very long text. And more text. Another
+      text.
+    </div>
+  );
 
 <Prettier>
   {(We) =>
     "The purple monkey danced with a tambourine made of cheese." +
     "The robot chef cooked a cake that tasted like rainbows." +
     "The talking pineapple sang a lullaby to the sleepy giraffe."
   }
 </Prettier>;
 <Prettier>
   {(We) => (love) =>
     "The purple monkey danced with a tambourine made of cheese." +
     "The robot chef cooked a cake that tasted like rainbows." +
-    "The talking pineapple sang a lullaby to the sleepy giraffe."
-  }
+    "The talking pineapple sang a lullaby to the sleepy giraffe."}
 </Prettier>;
 <Prettier>
   {(We) => (love) => (currying) =>
     "The purple monkey danced with a tambourine made of cheese." +
     "The robot chef cooked a cake that tasted like rainbows." +
-    "The talking pineapple sang a lullaby to the sleepy giraffe."
-  }
+    "The talking pineapple sang a lullaby to the sleepy giraffe."}
 </Prettier>;

```

**Prettier Similarity**: 54.10%


# jsx/jsx/await.js
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


# jsx/jsx/quotes.js
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


# jsx/newlines/test.js
```diff
 keep = (
   <p>
     Welcome to the <strong>Universal React Starter-kyt</strong>. This starter
     kyt should serve as the base for an advanced, server-rendered React app.
   </p>
 );
 
 newlines_text = <div>hi there how are you are you fine today?</div>;
 
 newlines_text_spaced = <div>space above space below</div>;
 
 newlines_elems_spaced = (
   <div>
     <span>space above</span>
 
     <span>space below</span>
   </div>
 );
 
 newlines_mixed = (
   <div>
     hi
     <span>there</span>
     how are <strong>you</strong>
     are you fine today?
   </div>
 );
 
 newlines_elems = (
   <div>
     <div>
       <div></div>
     </div>
     hi
     <div></div>
+
     <span />
+
     <Big />
   </div>
 );
 
 regression_extra_newline = (
   <div>
     <span className="nuclide-console-new-messages-notification-icon icon icon-nuclicon-arrow-down" />
     New Messages
   </div>
 );
 
 regression_extra_newline_2 = (
   <div>
     (
     <FormattedMessage
       id="some-id"
       defaultMessage="some loooooooooooooooooooooooooooong default"
     />
     )
   </div>
 );

```

**Prettier Similarity**: 96.61%


# jsx/spread/attribute.js
```diff
 <div {...a} />;
 
 <div {...(a || {})} />;
 
 <div {...(cond ? foo : bar)} />;
 
 <div {...a /* comment */} />;
 
-<div {/* comment */ ...a} />;
+<div {.../* comment */ a} />;
 
 <div
   {
     ...a //comment
   }
 />;
 
 <div
   {
     ...a
     //comment
   }
 />;
 
 <div
   {
-    //comment
-    ...a
+    ...//comment
+    a
   }
 />;
 
 <div
   {
-    //comment
-    ...a // comment
+    ...//comment
+    a // comment
   }
 />;

```

**Prettier Similarity**: 86.11%


# jsx/spread/child.js
```diff
 <div>{...a}</div>;
 
 <div>{...a /* comment */}</div>;
 
-<div>{/* comment */ ...a}</div>;
+<div>{.../* comment */ a}</div>;
 
 <div>
   {
     ...a //comment
   }
 </div>;
 
 <div>
   {
     ...a
     //comment
   }
 </div>;
 
 <div>
   {
-    //comment
-    ...a
+    ...//comment
+    a
   }
 </div>;
 
 <div>
   {
-    //comment
-    ...a // comment
+    ...//comment
+    a // comment
   }
 </div>;

```

**Prettier Similarity**: 84.38%


# jsx/template/styled-components.js
```diff
 <style jsx>{`
   p {
     color: red;
   }
 `}</style>;
 
 <style jsx>{tpl`
   p {
     color: red;
   }
 `}</style>;
 
 <style jsx>
-  {`
-    p {
-      color: red;
-    }
+  {`p {
+     color: red;
+     }
   `}
 </style>;

```

**Prettier Similarity**: 78.95%


# jsx/text-wrap/test.js
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
     <span className="score">{`${mini.crosstable.users[sessionUserId]} - ${
       mini.crosstable.users[user.id]
     }`}</span>
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
-    A<br />B<br />C
+    A
+    <br />
+    B
+    <br />
+    C
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
+
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
 
 // FIXME
 br_triggers_expression_break = (
   <div>
     <br />
     text text text text text text text text text text text {
       this.props.type
     }{" "}
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

**Prettier Similarity**: 98.96%


# jsx/tuple/tuple.js
```diff
 a = [<div />, <div />];
 
-a = #[<div />, <div />];
+a = #;
+[<div />, <div />];

```

**Prettier Similarity**: 50.00%


# typescript/arrow/comments.ts
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


# typescript/arrows/type_params.ts
```diff
-<T,>(a) => {};
+<T>(a) => {};

```

**Prettier Similarity**: 0.00%


# typescript/as/expression-statement.ts
```diff
 // expression statemnt of "as" expression hardly ever makes sense, but it's still valid.
 const [type, x] = [0, 0];
-type as unknown;
+// FIXME
+// TODO: parse issue
+// (type) as unknown;
 x as unknown;

```

**Prettier Similarity**: 50.00%


# typescript/assignment/issue-5370.ts
```diff
-const durabilityMetricsSelectable: Immutable.OrderedSet<SomeReportingMetric> =
-  myExperienceSelectable.concat(otherDurabilityMetricsSelectable);
+const durabilityMetricsSelectable: Immutable.OrderedSet<
+  SomeReportingMetric,
+> = myExperienceSelectable.concat(otherDurabilityMetricsSelectable);

```

**Prettier Similarity**: 0.00%


# typescript/chain-expression/test.ts
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


# typescript/class/constructor.ts
```diff
 class C {
   constructor(override a: number) {}
 }
 class D {
   constructor(private a: number) {}
 }
 class E {
   constructor(protected a: number) {}
 }
 class F {
   constructor(public a: number) {}
 }
 class G {
   constructor(readonly a: number) {}
 }
 
 class A {
-  "constructor": typeof A;
+  'constructor': typeof A
   static Foo() {
     return new A();
   }
 }
 
 class B {
   constructor<>() {}
 }

```

**Prettier Similarity**: 96.15%


# typescript/class/duplicates-access-modifier.ts
```diff
 class Foo {
-  public a;
-  private b;
-  protected c;
-  public d;
-  public e;
+  public public a;
+  private public b;
+  protected private c;
+  public protected d;
+  public protected private e;
 }

```

**Prettier Similarity**: 28.57%


# typescript/class/empty-method-body.ts
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


# typescript/class/parameter-properties.ts
```diff
 class MyClass {
-  constructor(
-    protected x: number,
-    private y: string,
-  ) {}
+  constructor(protected x: number, private y: string) {}
 }
 
 [
   class {
-    constructor(
-      protected x: number,
-      private y: string,
-    ) {}
+    constructor(protected x: number, private y: string) {}
   },
 ];
 
 class Mixed {
-  constructor(
-    public a: number,
-    b: unknown,
-  ) {}
+  constructor(public a: number, b: unknown) {}
 }
 
 class OneParameterProperty {
   constructor(public foobar: boolean) {}
 }

```

**Prettier Similarity**: 53.85%


# typescript/class/quoted-property.ts
```diff
 class User {
-  "username": string;
+  username: string;
 }

```

**Prettier Similarity**: 66.67%


# typescript/comments/declare_function.ts
```diff
 declare function fn(
   currentRequest: { a: number },
   // TODO this is a very very very very long comment that makes it go > 80 columns
 ): number;
 
-declare function /* foo */ f(/* baz */ a /* taz */); /* bar */
+declare function /* foo */ f(/* baz */ a /* taz */) /* bar */;

```

**Prettier Similarity**: 83.33%


# typescript/comments/mapped_types.ts
```diff
 type A = {
   // commentA
   [a in A]: string;
 };
 
 type B = {
   /* commentB */ [b in B]: string;
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

```

**Prettier Similarity**: 95.00%


# typescript/comments/method_types.ts
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


# typescript/comments/ts-parameter-proerty.ts
```diff
 class A {
-  constructor(private readonly paramProp: Type) // comment
-  {}
+  constructor(
+    private readonly paramProp: Type,
+    // comment
+  ) {}
 }

```

**Prettier Similarity**: 33.33%


# typescript/comments/type-parameters.ts
```diff
 functionName<A /* A comment */>();
 const a: T</* comment */> = 1;
 functionName</* comment */>();
 function foo</* comment */>() {}
 interface Foo {
-  </* comment */>(arg): any;
+ </* comment */>(arg): any;
 }
 type T = </* comment */>(arg) => any;
 
 functionName<A>(); // comment
 const a: T<
   // comment
 > = 1;
 functionName<
   // comment
 >();
 function foo<
   // comment
 >() {}
 interface Foo {
   <
     A, // comment
   >(
     arg,
   ): any;
 }
 type T = <
   // comment
->(
-  arg,
-) => any;
+>(arg) => any;

```

**Prettier Similarity**: 87.10%


# typescript/compiler/contextualSignatureInstantiation2.ts
```diff
 // dot f g x = f(g(x))
 var dot: <T, S>(f: (_: T) => S) => <U>(g: (_: U) => T) => (_: U) => S;
 dot =
   <T, S>(f: (_: T) => S) =>
-  <U,>(g: (_: U) => T): ((r: U) => S) =>
+  <U>(g: (_: U) => T): ((r: U) => S) =>
   (x) =>
     f(g(x));
 var id: <T>(x: T) => T;
 var r23 = dot(id)(id);

```

**Prettier Similarity**: 88.89%


# typescript/compiler/decrementAndIncrementOperators.ts
```diff
 var x = 0;
 
 // errors
 1++;
 
-1++;
-1--;
+(1)++;
+(1)--;
 
-++1;
---1;
+++(1);
+--(1);
 
 (1 + 2)++;
 (1 + 2)--;
 
 ++(1 + 2);
 --(1 + 2);
 
 (x + x)++;
 (x + x)--;
 
 ++(x + x);
 --(x + x);
 
 //OK
 x++;
 x--;
 
 ++x;
 --x;
 
 x++;
 --x;
 
 x++;
 x--;
 
 x[x++]++;

```

**Prettier Similarity**: 89.19%


# typescript/conditional-types/parentheses.ts
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
-type Test<T> = T extends ((
+type Test<T> = T extends (
   token: TSESTree.Token,
-) => asserts token is infer U extends TSESTree.Token)
+) => asserts token is infer U extends TSESTree.Token
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

**Prettier Similarity**: 56.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractInstantiations2.ts
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


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractMixedWithModifiers.ts
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


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractProperties.ts
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


# typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorImplementationWithDefaultValues2.ts
```diff
 class C {
   constructor(x);
   constructor(public x: string = 1) {
     var y = x;
   }
 }
 
 class D<T, U> {
   constructor(x: T, y: U);
-  constructor(
-    x: T = 1,
-    public y: U = x,
-  ) {
+  constructor(x: T = 1, public y: U = x) {
     var z = x;
   }
 }
 
 class E<T extends Date> {
   constructor(x);
   constructor(x: T = new Date()) {
     var y = x;
   }
 }

```

**Prettier Similarity**: 82.61%


# typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorParameterProperties.ts
```diff
 class C {
   y: string;
-  constructor(
-    private x: string,
-    protected z: string,
-  ) {}
+  constructor(private x: string, protected z: string) {}
 }
 
 var c: C;
 var r = c.y;
 var r2 = c.x; // error
 var r3 = c.z; // error
 
 class D<T> {
   y: T;
-  constructor(
-    a: T,
-    private x: T,
-    protected z: T,
-  ) {}
+  constructor(a: T, private x: T, protected z: T) {}
 }
 
 var d: D<string>;
 var r = d.y;
 var r2 = d.x; // error
 var r3 = d.a; // error
 var r4 = d.z; // error

```

**Prettier Similarity**: 66.67%


# typescript/conformance/classes/constructorDeclarations/constructorParameters/readonlyInConstructorParameters.ts
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


# typescript/conformance/classes/constructorDeclarations/constructorParameters/readonlyReadonly.ts
```diff
 class C {
-  readonly x: number;
-  constructor(readonly y: number) {}
+  readonly readonly x: number;
+  constructor(readonly readonly y: number) {}
 }

```

**Prettier Similarity**: 50.00%


# typescript/conformance/classes/mixinClassesAnnotated.ts
```diff
 // @declaration: true
 
 type Constructor<T> = new (...args: any[]) => T;
 
 class Base {
-  constructor(
-    public x: number,
-    public y: number,
-  ) {}
+  constructor(public x: number, public y: number) {}
 }
 
 class Derived extends Base {
-  constructor(
-    x: number,
-    y: number,
-    public z: number,
-  ) {
+  constructor(x: number, y: number, public z: number) {
     super(x, y);
   }
 }
 
 const Printable = <T extends Constructor<Base>>(
   superClass: T,
 ): Constructor<Printable> & { message: string } & T =>
   class extends superClass {
     static message = "hello";
     print() {
       const output = this.x + "," + this.y;
     }
   };
 
 function Tagged<T extends Constructor<{}>>(
   superClass: T,
 ): Constructor<Tagged> & T {
   class C extends superClass {
     _tag: string;
     constructor(...args: any[]) {
       super(...args);
       this._tag = "hello";
     }
   }
   return C;
 }
 
 const Thing1 = Tagged(Derived);
 const Thing2 = Tagged(Printable(Derived));
 Thing2.message;
 
 function f1() {
   const thing = new Thing1(1, 2, 3);
   thing.x;
   thing._tag;
 }
 
 function f2() {
   const thing = new Thing2(1, 2, 3);
   thing.x;
   thing._tag;
   thing.print();
 }
 
 class Thing3 extends Thing2 {
   constructor(tag: string) {
     super(10, 20, 30);
     this._tag = tag;
   }
   test() {
     this.print();
   }
 }

```

**Prettier Similarity**: 87.14%


# typescript/conformance/classes/mixinClassesAnonymous.ts
```diff
 type Constructor<T> = new (...args: any[]) => T;
 
 class Base {
-  constructor(
-    public x: number,
-    public y: number,
-  ) {}
+  constructor(public x: number, public y: number) {}
 }
 
 class Derived extends Base {
-  constructor(
-    x: number,
-    y: number,
-    public z: number,
-  ) {
+  constructor(x: number, y: number, public z: number) {
     super(x, y);
   }
 }
 
 const Printable = <T extends Constructor<Base>>(superClass: T) =>
   class extends superClass {
     static message = "hello";
     print() {
       const output = this.x + "," + this.y;
     }
   };
 
 function Tagged<T extends Constructor<{}>>(superClass: T) {
   class C extends superClass {
     _tag: string;
     constructor(...args: any[]) {
       super(...args);
       this._tag = "hello";
     }
   }
   return C;
 }
 
 const Thing1 = Tagged(Derived);
 const Thing2 = Tagged(Printable(Derived));
 Thing2.message;
 
 function f1() {
   const thing = new Thing1(1, 2, 3);
   thing.x;
   thing._tag;
 }
 
 function f2() {
   const thing = new Thing2(1, 2, 3);
   thing.x;
   thing._tag;
   thing.print();
 }
 
 class Thing3 extends Thing2 {
   constructor(tag: string) {
     super(10, 20, 30);
     this._tag = tag;
   }
   test() {
     this.print();
   }
 }
 
 // Repro from #13805
 
 const Timestamped = <CT extends Constructor<object>>(Base: CT) => {
   return class extends Base {
     timestamp = new Date();
   };
 };

```

**Prettier Similarity**: 87.50%


# typescript/conformance/internalModules/importDeclarations/exportImportAlias.ts
```diff
 // expect no errors here
 
 module A {
   export var x = "hello world";
   export class Point {
-    constructor(
-      public x: number,
-      public y: number,
-    ) {}
+    constructor(public x: number, public y: number) {}
   }
   export module B {
     export interface Id {
       name: string;
     }
   }
 }
 
 module C {
   export import a = A;
 }
 
 var a: string = C.a.x;
 var b: { x: number; y: number } = new C.a.Point(0, 0);
 var c: { name: string };
 var c: C.a.B.Id;
 
 module X {
   export function Y() {
     return 42;
   }
 
   export module Y {
     export class Point {
-      constructor(
-        public x: number,
-        public y: number,
-      ) {}
+      constructor(public x: number, public y: number) {}
     }
   }
 }
 
 module Z {
   // 'y' should be a fundule here
   export import y = X.Y;
 }
 
 var m: number = Z.y();
 var n: { x: number; y: number } = new Z.y.Point(0, 0);
 
 module K {
   export class L {
     constructor(public name: string) {}
   }
 
   export module L {
     export var y = 12;
     export interface Point {
       x: number;
       y: number;
     }
   }
 }
 
 module M {
   export import D = K.L;
 }
 
 var o: { name: string };
 var o = new M.D("Hello");
 
 var p: { x: number; y: number };
 var p: M.D.Point;

```

**Prettier Similarity**: 88.89%


# typescript/conformance/internalModules/importDeclarations/importAliasIdentifiers.ts
```diff
 module moduleA {
   export class Point {
-    constructor(
-      public x: number,
-      public y: number,
-    ) {}
+    constructor(public x: number, public y: number) {}
   }
 }
 
 import alias = moduleA;
 
 var p: alias.Point;
 var p: moduleA.Point;
 var p: { x: number; y: number };
 
 class clodule {
   name: string;
 }
 
 module clodule {
   export interface Point {
     x: number;
     y: number;
   }
   var Point: Point = { x: 0, y: 0 };
 }
 
 import clolias = clodule;
 
 var p: clolias.Point;
 var p: clodule.Point;
 var p: { x: number; y: number };
 
 function fundule() {
   return { x: 0, y: 0 };
 }
 
 module fundule {
   export interface Point {
     x: number;
     y: number;
   }
   var Point: Point = { x: 0, y: 0 };
 }
 
 import funlias = fundule;
 
 var p: funlias.Point;
 var p: fundule.Point;
 var p: { x: number; y: number };

```

**Prettier Similarity**: 92.00%


# typescript/conformance/parser/ecmascript5/Statements/parserES5ForOfStatement21.ts
```diff
 //@target: ES5
-for (var of of) {
-}
+for (var of of) { }

```

**Prettier Similarity**: 33.33%


# typescript/conformance/types/moduleDeclaration/kind-detection.ts
```diff
-declare namespace /* module */ A {}
+declare /* module */ namespace A {}

```

**Prettier Similarity**: 0.00%


# typescript/custom/abstract/abstractProperties.ts
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


# typescript/declare/declare_function_with_body.ts
```diff
 // Invalid, but recoverable
-declare function foo() {};
+declare function foo() {}
 declare function bar() {
   // comment
-};
+}

```

**Prettier Similarity**: 60.00%


# typescript/declare/object-type-in-declare-function.ts
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


# typescript/declare/trailing-comma/function-rest-trailing-comma.ts
```diff
-declare function foo(...args: any[]);
-declare function foo(
-  ...long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_args: any[]
-);
+declare function foo(...args: any[], )
+declare function foo(...long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_args: any[], )

```

**Prettier Similarity**: 0.00%


# typescript/decorators-ts/angular.ts
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


# typescript/decorators-ts/typeorm.ts
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


# typescript/decorators/decorators-comments.ts
```diff
 class Foo1 {
   @foo
-  // comment
-  async method() {}
+  async // comment
+  method() {}
 }
 
 class Foo2 {
   @foo
   // comment
   private method() {}
 }
 
 class Foo3 {
   @foo
-  // comment
-  *method() {}
+  *// comment
+  method() {}
 }
 
 class Foo4 {
   @foo
-  // comment
-  async *method() {}
+  async *// comment
+  method() {}
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

**Prettier Similarity**: 77.14%


# typescript/definite/definite.ts
```diff
 class MyComponent {
   ngModel!: ng.INgModelController;
 }
 
-const x!: string = "";
+const x!: string = '';
 
 var y!: MyComponent;

```

**Prettier Similarity**: 85.71%


# typescript/definite/without-annotation.ts
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


# typescript/error-recovery/generic.ts
```diff
 f1<>();
 
 new f2<>();
 
 function f3<>() {}
 
 class f4 {
   constructor<>() {}
 }
 
-const f5 = function <>() {};
+const f5 = function<>() {}
 
 interface f6<> {
-  test<>();
+    test<>();
 }
 
 class f7<> {
-  test<>() {}
+    test<>() {}
 }

```

**Prettier Similarity**: 84.21%


# typescript/error-recovery/index-signature.ts
```diff
 type A = { [key: string] };
 
 type TwoParams = {
   [a: string, b: string]: string;
-};
+}
 type ThreeParams = {
   [a: string, b: string, c: string]: string;
-};
+}
 
 type TooLong = {
-  [
-    loooooooooooooooooooooooooong: string,
-    looooooooooooooooooooooooooooooooooooooong: string,
-  ]: string;
-};
-type TooLong81 = {
-  [
-    loooooooooooooooooooooooooong: string,
-    loooooooooooooooooong: string,
-  ]: string;
-};
-type TooLong80 = {
-  [loooooooooooooooooooooooooong: string, looooooooooooooooong: string]: string;
-};
+  [loooooooooooooooooooooooooong: string, looooooooooooooooooooooooooooooooooooooong: string]: string;
+}
+type TooLong81 =
+  { [loooooooooooooooooooooooooong: string, loooooooooooooooooong: string]: string;
+}
+type TooLong80 =
+  { [loooooooooooooooooooooooooong: string, looooooooooooooooong: string]: string;
+}
 
 // note lack of trailing comma in the index signature
 type TooLongSingleParam = {
-  [
-    looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong: string
-  ]: string;
+  [looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong: string]: string;
 };

```

**Prettier Similarity**: 38.71%


# typescript/error-recovery/jsdoc_only_types.ts
```diff
-let a: *;
+let a:
+*
 function b(x: ?) {}
-let c: ?string;
-let d: string?;
-let e: ?(string | number);
-let f: !string;
-let g: string!;
-let h: !(string | number);
+let c:
+?string
+let d: string;
+?
+let e:
+?(string | number)
+let f:
+!string;
+let g: string;
+!;
+let h:
+!(string | number);

```

**Prettier Similarity**: 6.67%


# typescript/function-type/consistent.ts
```diff
 // TSFunctionType
 type A = (
   tpl: TemplateStringsArray,
   ...args: Array<unknown>
 ) => (replacements?: PublicReplacements) => T;
 
 // TSConstructorType
 type B = new (
   tpl: TemplateStringsArray,
   ...args: Array<unknown>
 ) => (replacements?: PublicReplacements) => T;
 
 type X = {
   // TSCallSignatureDeclaration
-  (
-    tpl: TemplateStringsArray,
-    ...args: Array<unknown>
-  ): (replacements?: PublicReplacements) => T;
+  (tpl: TemplateStringsArray, ...args: Array<unknown>): (
+    replacements?: PublicReplacements,
+  ) => T;
 
   // TSConstructSignatureDeclaration
-  new (
-    tpl: TemplateStringsArray,
-    ...args: Array<unknown>
-  ): (replacements?: PublicReplacements) => T;
+  new (tpl: TemplateStringsArray, ...args: Array<unknown>): (
+    replacements?: PublicReplacements,
+  ) => T;
 };

```

**Prettier Similarity**: 68.00%


# typescript/generic/ungrouped-parameters.ts
```diff
-function filterTooltipWithFoo<F extends Field>(
-  oldEncoding: Encoding<F>,
-): {
+function filterTooltipWithFoo<F extends Field>(oldEncoding: Encoding<F>): {
   customTooltipWithoutAggregatedField?:
     | StringFieldDefWithCondition<F>
     | StringValueDefWithCondition<F>
     | StringFieldDef<F>[];
   filteredEncoding: Encoding<F>;
 } {
   const { tooltip, ...filteredEncoding } = oldEncoding;
   if (!tooltip) {
     return { filteredEncoding };
   }
   // ...
 }

```

**Prettier Similarity**: 80.00%


# typescript/import-export/type-modifier.ts
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


# typescript/interface2/break/break.ts
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

```

**Prettier Similarity**: 93.33%


# typescript/intersection/consistent-with-flow/intersection-parens.ts
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


# typescript/last-argument-expansion/decorated-function.tsx
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


# typescript/last-argument-expansion/forward-ref.tsx
```diff
-export const Link = forwardRef<HTMLAnchorElement, LinkProps>(
-  function Link(props, ref) {
-    return <ThemeUILink ref={ref} variant="default" {...props} />;
-  },
-);
+export const Link = forwardRef<HTMLAnchorElement, LinkProps>(function Link(
+  props,
+  ref,
+) {
+  return <ThemeUILink ref={ref} variant="default" {...props} />;
+});
 
 export const LinkWithLongName = forwardRef<HTMLAnchorElement, LinkProps>(
   function Link(props, ref) {
     return <ThemeUILink ref={ref} variant="default" {...props} />;
   },
 );
 
 export const Arrow = forwardRef<HTMLAnchorElement, LinkProps>((props, ref) => {
   return <ThemeUILink ref={ref} variant="default" {...props} />;
 });
 
 export const ArrowWithLongName = forwardRef<HTMLAnchorElement, LinkProps>(
   (props, ref) => {
     return <ThemeUILink ref={ref} variant="default" {...props} />;
   },
 );
 
-const Link = React.forwardRef<HTMLAnchorElement, LinkProps>(
-  function Link(props, ref) {
-    return <ThemeUILink ref={ref} variant="default" {...props} />;
-  },
-);
+const Link = React.forwardRef<HTMLAnchorElement, LinkProps>(function Link(
+  props,
+  ref,
+) {
+  return <ThemeUILink ref={ref} variant="default" {...props} />;
+});

```

**Prettier Similarity**: 58.62%


# typescript/mapped-type/break-mode/break-mode.ts
```diff
 type A1 = { readonly [A in B]: T };
-type A2 = {
-  readonly [A in B]: T;
-};
+type A2 = { readonly [A in B]: T };
 type A3 = {
   readonly [A in B]: T;
 };
-type A4 = {
+type A4 = { readonly [A in B]: T };
+type A5 = { readonly [A in B]: T };
+type A6 = { readonly [A in B]: T };
+type A7 = {
   readonly [A in B]: T;
 };
-type A5 = { readonly [A in B]: T };
-type A6 = { readonly [A in B]: T };
-type A7 = { readonly [A in B]: T };

```

**Prettier Similarity**: 46.15%


# typescript/mapped-type/issue-11098.ts
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
-  // comment
-  readonly [T in number];
+  readonly // comment
+  [T in number];
 };
 
 type Type = {
-  // foo
-  /* bar */ readonly [T in number];
+  readonly // foo
+  /* bar */ [T in number];
 };

```

**Prettier Similarity**: 92.00%


# typescript/multiparser-css/issue-6259.ts
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


# typescript/non-null/optional-chain.ts
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


# typescript/prettier-ignore/issue-14238.ts
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


# typescript/prettier-ignore/mapped-types.ts
```diff
 type a = {
     // prettier-ignore
     [A in B]: C  |  D
   };
 
 type a = {
     [
       // prettier-ignore
       A in B
     ]: C  |  D
   };
 
-type a = {
-  [A in B]: C | D; // prettier-ignore
-};
+// TODO: fix idempotency issue
+// type a= {
+//     [
+//       A in
+//       // prettier-ignore
+//       B
+//     ]: C  |  D
+//   }
 
 type a = {
-  A in B: C | D; // prettier-ignore
+  [A in B]: // prettier-ignore
+  C | D;
 };
 
 type a = {
     [
       /* prettier-ignore */
       A in B
     ]: C  |  D
   };
 
-type a = {
-  [A /* prettier-ignore */ in B]: C | D;
-};
+// TODO: fix idempotency issue
+// type a= {
+//     [
+//       A in
+//       /* prettier-ignore */
+//       B
+//     ]: C  |  D
+//   }
 
 type a = {
-  A in B /* prettier-ignore */: C | D;
+  [A in B]: /* prettier-ignore */
+  C | D;
 };
 
 type a = {
     /* prettier-ignore */ [A in B]: C  |  D
   };
 
 type a = {
-    [/* prettier-ignore */ A in B ]: C  |  D
-  };
+  [/* prettier-ignore */ A in B]: C | D;
+};
 
 type a = {
   [A in /* prettier-ignore */ B]: C | D;
 };
 
 type a = {
-  [A in B /* prettier-ignore */]: C | D;
+  [A in B]: /* prettier-ignore */ C | D;
 };
 
 type a = {
     /* prettier-ignore */
     [A in B]: C  |  D
   };

```

**Prettier Similarity**: 65.67%


# typescript/prettier-ignore/prettier-ignore-nested-unions.ts
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


# typescript/prettier-ignore/prettier-ignore-parenthesized-type.ts
```diff
 type Foo =
   // prettier-ignore
-  aa;
+  (
+    aa
+  );

```

**Prettier Similarity**: 40.00%


# typescript/satisfies-operators/comments-unstable.ts
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


# typescript/satisfies-operators/expression-statement.ts
```diff
 let type: "foo" | "bar" = "foo";
 
 // demonstrating how "satisfies" expression can be practically used as expression statement.
 const _ = () => {
   switch (type) {
     case "foo":
       return 1;
     case "bar":
       return 2;
     default:
       // exhaustiveness check idiom
       type satisfies never;
       throw new Error("unreachable");
   }
 };
 
 function needParens() {
-  let satisfies unknown;
-  interface satisfies unknown;
+  (let) satisfies unknown;
+  (interface) satisfies unknown;
   module satisfies unknown;
   using satisfies unknown;
-  yield satisfies unknown;
-  await satisfies unknown;
+  (yield) satisfies unknown;
+  (await) satisfies unknown;
 }
 
 function noNeedParens() {
   async satisfies unknown;
   satisfies satisfies unknown;
   as satisfies unknown;
 
   abc satisfies unknown; // not a keyword
 }
 
 function satisfiesChain() {
   satisfies satisfies satisfies satisfies satisfies;
   type satisfies never satisfies unknown;
 }

```

**Prettier Similarity**: 89.19%


# typescript/test-declarations/test_declarations.ts
```diff
-test("does something really long and complicated so I have to write a very long name for the test", <T>(done) => {
+test("does something really long and complicated so I have to write a very long name for the test", <
+  T,
+>(done) => {
   console.log("hello!");
 });

```

**Prettier Similarity**: 40.00%


# typescript/trailing-comma/trailing.ts
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


# typescript/trailing-comma/type-parameters-vs-arguments.ts
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


# typescript/type-arguments-bit-shift-left-like/1.ts
```diff
-f << (<T>x);
+f << <T>x;

```

**Prettier Similarity**: 0.00%


# typescript/typeparams/const.ts
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


# typescript/typeparams/empty-parameters-with-arrow-function/issue-13817.ts
```diff
-const xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx: xxxxxxxxxxxxxxxxxxxxxx<> = (
-  arg,
-) => null;
+const xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx: xxxxxxxxxxxxxxxxxxxxxx<> =
+  arg => null;
 
 const xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx: xxxxxxxxxxxxxxxxxxxxxx</* comment */> =
-  (arg) => null;
+  arg => null;
 
 const xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx: xxxxxxxxxxxxxxxxxxxxxx<
   // comment
-> = (arg) => null;
+> =
+  arg => null;

```

**Prettier Similarity**: 50.00%


# typescript/typeparams/line-breaking-after-extends-2.ts
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
   T extends // comment
-    N.Function | N.TSDeclareMethod | Baz,
+  N.Function | N.TSDeclareMethod | Baz,
 >();
 
 function makeChainWalker<
   ArgT extends {
     options: ValidatedOptions;
     dirname: string;
     filepath?: string;
   },
 >() {}

```

**Prettier Similarity**: 96.97%


# typescript/typeparams/line-breaking-after-extends.ts
```diff
 export type OuterType1<
-  LongerLongerLongerLongerInnerType extends
-    LongerLongerLongerLongerOtherType<OneMoreType>,
+  LongerLongerLongerLongerInnerType extends LongerLongerLongerLongerOtherType<OneMoreType>,
 > = { a: 1 };
 
 export type OuterType2<
-  LongerLongerLongerLongerInnerType extends
-    LongerLongerLongerLongerLongerLongerLongerLongerOtherType,
+  LongerLongerLongerLongerInnerType extends LongerLongerLongerLongerLongerLongerLongerLongerOtherType,
 > = { a: 1 };
 
 export type OuterType3<
-  LongerLongerLongerLongerInnerType extends
-    LongerLongerLongerLongerLongerLo.ngerLongerLongerOtherType,
+  LongerLongerLongerLongerInnerType extends LongerLongerLongerLongerLongerLo.ngerLongerLongerOtherType,
 > = { a: 1 };
 
 export type OuterType4<
   LongerLongerLongerLongerInnerType extends
     | LongerLongerLongerLongerLongerLo
     | ngerLongerLongerOtherType,
 > = { a: 1 };

```

**Prettier Similarity**: 70.00%


# typescript/typeparams/print-width-120/issue-7542.tsx
```diff
 export const Foo = forwardRef(
   (props: FooProps, ref: Ref<HTMLElement>): JSX.Element => {
     return <div />;
   },
 );
 
 export const Bar = forwardRef(
   (props: BarProps, ref: Ref<HTMLElement>): JSX.Element | null => {
     return <div />;
   },
 );
 
 users.map((user: User): User => {
   return user;
 });
 
-users.map((user: User): User => {
-  // comment
-});
+// FIXME
+// TODO: reformat issue
+// users.map((user: User): User => {
+//   ; // comment
+// })
 
 users.map((user: User): User => {
   // comment
 });

```

**Prettier Similarity**: 80.00%


# typescript/union/consistent-with-flow/prettier-ignore.ts
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


# typescript/union/consistent-with-flow/single-type.ts
```diff
-type A1 =
-  | A
-  // A comment to force break
-  | B;
-type A2 =
-  | (
-      | A
-      // A comment to force break
-      | B
-    )
-  | (
-      | A
-      // A comment to force break
-      | B
-    );
-type A3 =
-  | A
-  // A comment to force break
-  | B;
-type A4 =
-  | A
-  // A comment to force break
-  | B;
-type A5 =
-  | ({ key: string } | { key: string } | { key: string } | { key: string })
-  | { key: string }
-  | { key: string };
-type A6 =
-  /*1*/
-  | A
-  // A comment to force break
-  | B;
-
-type B1 =
-  | A
-  // A comment to force break
-  | B;
-type B2 =
-  | A
-  // A comment to force break
-  | B;
+// FIXME
+// TODO: we emit invalid AST
+// type A1 =
+//   | (
+//     | (
+//       | (
+//           | A
+//           // A comment to force break
+//           | B
+//         )
+//     )
+//   );
+// type A2 =
+//   | (
+//     | (
+//           | A
+//           // A comment to force break
+//           | B
+//         )
+//     | (
+//           | A
+//           // A comment to force break
+//           | B
+//         )
+//   );
+// type A3 =
+//   | ( | (
+//           | A
+//           // A comment to force break
+//           | B
+//         ) );
+// type A4 =
+//   | ( | ( | (
+//           | A
+//           // A comment to force break
+//           | B
+//         ) ) );
+// type A5 =
+//   | (
+//     | (
+//       | { key: string }
+//       | { key: string }
+//       | { key: string }
+//       | { key: string }
+//     )
+//     | { key: string }
+//     | { key: string }
+//   );
+// type A6 = | (
+//   /*1*/ | (
+//     | (
+//           | A
+//           // A comment to force break
+//           | B
+//         )
+//   )
+//   );
+//
+// type B1 =
+//   | (
+//     & (
+//       (
+//           | A
+//           // A comment to force break
+//           | B
+//         )
+//     )
+//   );
+// type B2 =
+//   | (
+//     & (
+//       | (
+//         & (
+//           (
+//           | A
+//           // A comment to force break
+//           | B
+//         )
+//         )
+//       )
+//     )
+//   );

```

**Prettier Similarity**: 0.00%


# typescript/union/single-type/single-type.ts
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


# typescript/union/union-parens.ts
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


