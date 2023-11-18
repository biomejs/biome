# Overall Metrics

**Average compatibility**: 94.08

    <details>
    	<summary>Definition</summary>

    	$$average = \frac\{\sum_{file}^\{files}compatibility_\{file}}\{files}$$
    </details>

    **Compatible lines**: 94.97
    <details>
        <summary>Definition</summary>

        $$average = \frac{\sum_{file}^{files}matching\_lines_{file}}{max(lines_{rome}, lines_{prettier})}$$
    </details>

    [Metric definition discussion](https://github.com/rome/tools/issues/2555#issuecomment-1124787893)
                

# js/array-spread/multiple.js

**Prettier Similarity**: 100.00%


# js/arrays/empty.js

**Prettier Similarity**: 100.00%


# js/arrays/holes-in-args.js

**Prettier Similarity**: 100.00%


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


# js/arrays/last.js

**Prettier Similarity**: 100.00%


# js/arrays/nested.js

**Prettier Similarity**: 100.00%


# js/arrays/numbers-in-args.js

**Prettier Similarity**: 100.00%


# js/arrays/numbers-in-assignment.js

**Prettier Similarity**: 100.00%


# js/arrays/numbers-negative-comment-after-minus.js

**Prettier Similarity**: 100.00%


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


# js/arrays/numbers-trailing-comma.js

**Prettier Similarity**: 100.00%


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


# js/arrays/numbers-with-trailing-comments.js

**Prettier Similarity**: 100.00%


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


# js/arrays/numbers1.js

**Prettier Similarity**: 100.00%


# js/arrays/numbers2.js

**Prettier Similarity**: 100.00%


# js/arrays/numbers3.js

**Prettier Similarity**: 100.00%


# js/arrays/preserve_empty_lines.js

**Prettier Similarity**: 100.00%


# js/arrow-call/arrow_call.js

**Prettier Similarity**: 100.00%


# js/arrow-call/class-property.js

**Prettier Similarity**: 100.00%


# js/arrows/arrow-chain-with-trailing-comments.js

**Prettier Similarity**: 100.00%


# js/arrows/arrow_function_expression.js

**Prettier Similarity**: 100.00%


# js/arrows/assignment-chain-with-arrow-chain.js

**Prettier Similarity**: 100.00%


# js/arrows/block_like.js

**Prettier Similarity**: 100.00%


# js/arrows/call.js

**Prettier Similarity**: 100.00%


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


# js/arrows/currying-3.js

**Prettier Similarity**: 100.00%


# js/arrows/currying-4.js
```diff
 Y(() => (a ? b : c));
 
-Y(() => () => (a ? b : c));
+Y(() => () => a ? b : c);
 
-Y(() => () => () => (a ? b : c));
+Y(() => () => () => a ? b : c);
 
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
-f((a) => (b) => (1 ? 2 : 3) /* b */ /* a */);
-f((a) => (b) => (c) => (1 ? 2 : 3) /* c */ /* b */ /* a */);
+f((a) => (b) => 1 ? 2 : 3 /* b */ /* a */);
+f((a) => (b) => (c) => 1 ? 2 : 3 /* b */ /* c */ /* a */);
 
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

**Prettier Similarity**: 64.86%


# js/arrows/currying.js

**Prettier Similarity**: 100.00%


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


# js/arrows/issue-4166-curry.js

**Prettier Similarity**: 100.00%


# js/arrows/long-call-no-args.js

**Prettier Similarity**: 100.00%


# js/arrows/long-contents.js

**Prettier Similarity**: 100.00%


# js/arrows/newline-before-arrow/newline-before-arrow.js
```diff
-async (x) => x;
+async;
+x;
+=> x

```

**Prettier Similarity**: 0.00%


# js/arrows/parens.js

**Prettier Similarity**: 100.00%


# js/arrows/semi/semi.js

**Prettier Similarity**: 100.00%


# js/assignment-comments/call.js

**Prettier Similarity**: 100.00%


# js/assignment-comments/call2.js

**Prettier Similarity**: 100.00%


# js/assignment-comments/function.js

**Prettier Similarity**: 100.00%


# js/assignment-comments/identifier.js

**Prettier Similarity**: 100.00%


# js/assignment-comments/number.js

**Prettier Similarity**: 100.00%


# js/assignment-comments/string.js

**Prettier Similarity**: 100.00%


# js/assignment-expression/assignment_expression.js

**Prettier Similarity**: 100.00%


# js/assignment/binaryish.js

**Prettier Similarity**: 100.00%


# js/assignment/call-with-template.js

**Prettier Similarity**: 100.00%


# js/assignment/chain-two-segments.js

**Prettier Similarity**: 100.00%


# js/assignment/chain.js

**Prettier Similarity**: 100.00%


# js/assignment/destructuring-array.js

**Prettier Similarity**: 100.00%


# js/assignment/destructuring-heuristic.js

**Prettier Similarity**: 100.00%


# js/assignment/destructuring.js

**Prettier Similarity**: 100.00%


# js/assignment/discussion-15196.js
```diff
 async function f() {
   const { section, rubric, authors, tags } =
     await utils.upsertCommonData(mainData);
 
   const loooooooooooooooooooooooooong1 =
     await looooooooooooooong.looooooooooooooong.loooooong;
   const loooooooooooooooooooooooooong2 =
     await looooooooooooooong.looooooooooooooong.loooooong();
   const loooooooooooooooooooooooooong3 =
     await looooooooooooooooooooooooooooooooooooooooooooog();
-  const loooooooooooooooooooooooooong4 =
-    !(await looooooooooooooong.looooooooooooooong.loooooong);
-  const loooooooooooooooooooooooooong5 =
-    void !!(await looooooooooooooong.looooooooooooooong.loooooong);
+  const loooooooooooooooooooooooooong4 = !(await looooooooooooooong
+    .looooooooooooooong.loooooong);
+  const loooooooooooooooooooooooooong5 = void !!(await looooooooooooooong
+    .looooooooooooooong.loooooong);
 
   const longlonglonglonglonglonglong1 = await new Promise((resolve, reject) => {
     setTimeout(() => {
       resolve("foo");
     }, 300);
   });
   const longlonglonglonglonglonglong2 = await {
     then(onFulfilled, onRejected) {
       onFulfilled(1234567890);
     },
   };
 }
 
 function* g() {
   const { section, rubric, authors, tags } =
     yield utils.upsertCommonData(mainData);
 
   const loooooooooooooooooooooooooong1 =
     yield looooooooooooooong.looooooooooooooong.loooooong;
   const loooooooooooooooooooooooooong2 =
     yield looooooooooooooong.looooooooooooooong.loooooong();
   const loooooooooooooooooooooooooong3 =
     yield looooooooooooooooooooooooooooooooooooooooooooog();
-  const loooooooooooooooooooooooooong4 =
-    !(yield looooooooooooooong.looooooooooooooong.loooooong);
-  const loooooooooooooooooooooooooong5 =
-    void !!(yield looooooooooooooong.looooooooooooooong.loooooong);
+  const loooooooooooooooooooooooooong4 = !(yield looooooooooooooong
+    .looooooooooooooong.loooooong);
+  const loooooooooooooooooooooooooong5 = void !!(yield looooooooooooooong
+    .looooooooooooooong.loooooong);
   const loooooooooooooooooooooooooong6 =
     yield* looooooooooooooong.looooooooooooooong.loooooong;
 
   const longlonglonglonglonglonglong1 = yield qwertyuiop(
     asdfghjkl,
     zxcvbnm,
     qwertyuiop,
     asdfghjkl,
   );
   const longlonglonglonglonglonglong2 = yield {
     qwertyuiop: 1234567890,
     asdfghjkl: 1234567890,
     zxcvbnm: 123456789,
   };
 
   const x = yield;
 }

```

**Prettier Similarity**: 86.21%


# js/assignment/issue-10218.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-1419.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-15534.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-1966.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-2184.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-2482-1.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-2482-2.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-2540.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-3819.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-4094.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-5610.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-6922.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-7091.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-7572.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-7961.js

**Prettier Similarity**: 100.00%


# js/assignment/issue-8218.js

**Prettier Similarity**: 100.00%


# js/assignment/lone-arg.js

**Prettier Similarity**: 100.00%


# js/assignment/sequence.js

**Prettier Similarity**: 100.00%


# js/assignment/unary.js

**Prettier Similarity**: 100.00%


# js/async/async-iteration.js

**Prettier Similarity**: 100.00%


# js/async/async-shorthand-method.js

**Prettier Similarity**: 100.00%


# js/async/await-parse.js

**Prettier Similarity**: 100.00%


# js/async/conditional-expression.js

**Prettier Similarity**: 100.00%


# js/async/exponentiation.js

**Prettier Similarity**: 100.00%


# js/async/inline-await.js

**Prettier Similarity**: 100.00%


# js/async/nested.js

**Prettier Similarity**: 100.00%


# js/async/nested2.js
```diff
 async function f() {
   await Promise.all(
-    (await readdir("src")).map(async (path) => {
+    (
+      await readdir("src")
+    ).map(async (path) => {
       import(`./${path}`);
     }),
   );
 }

```

**Prettier Similarity**: 66.67%


# js/async/parens.js

**Prettier Similarity**: 100.00%


# js/async/simple-nested-await.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/async-generators.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/bigint.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/class-properties.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/class-static-block.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/decorator-auto-accessors.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/decorators.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/destructuring-private.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/dynamic-import.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/explicit-resource-management.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/export-namespace-from.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/function-sent.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/import-assertions-dynamic.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/import-assertions-static.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/import-attributes-dynamic.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/import-attributes-static.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/import-meta.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/jsx.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/logical-assignment-operators.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/module-string-names.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/nullish-coalescing-operator.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/numeric-separator.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/object-rest-spread.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/optional-catch-binding.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/optional-chaining-assignment.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/optional-chaining.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/private-fields-in-in.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/private-methods.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/regex-v-flag.js

**Prettier Similarity**: 100.00%


# js/babel-plugins/regexp-modifiers.js

**Prettier Similarity**: 100.00%


# js/big-int/literal.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/arrow.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/bitwise-flags.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/call.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/comment.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/equality.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/exp.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/if.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/in_instanceof.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/inline-jsx.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/inline-object-array.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/jsx_parent.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/like-regexp.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/math.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/return.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/short-right.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/test.js

**Prettier Similarity**: 100.00%


# js/binary-expressions/unary.js

**Prettier Similarity**: 100.00%


# js/binary_math/parens.js

**Prettier Similarity**: 100.00%


# js/bracket-spacing/array.js

**Prettier Similarity**: 100.00%


# js/bracket-spacing/object.js

**Prettier Similarity**: 100.00%


# js/break-calls/break.js

**Prettier Similarity**: 100.00%


# js/break-calls/parent.js

**Prettier Similarity**: 100.00%


# js/break-calls/react.js

**Prettier Similarity**: 100.00%


# js/break-calls/reduce.js

**Prettier Similarity**: 100.00%


# js/call/first-argument-expansion/expression-2nd-arg.js
```diff
 call(function () {
   return 1;
 }, 200_000_000_000n * askTrovenaBeenaDependsRowans);
 
-call(
-  function () {
-    return 1;
-  },
-  (200_000_000_000n * askTrovenaBeenaDependsRowans) /
-    glimseGlyphsHazardNoopsTieTie,
-);
+call(function () {
+  return 1;
+}, (200_000_000_000n * askTrovenaBeenaDependsRowans) /
+  glimseGlyphsHazardNoopsTieTie);
 
-call(
-  function () {
-    return 1;
-  },
-  (askTrovenaBeenaDependsRowans = glimseGlyphsHazardNoopsTieTie =
-    200_000_000_000n),
-);
+call(function () {
+  return 1;
+}, (askTrovenaBeenaDependsRowans = glimseGlyphsHazardNoopsTieTie =
+  200_000_000_000n));

```

**Prettier Similarity**: 26.32%


# js/call/first-argument-expansion/issue-12892.js
```diff
-setTimeout(
-  () => {
-    console.log("test");
-  },
-  someFunctionCall(
-    veryLongParameterName1,
-    veryLongParameterName2,
-    veryLongParameterName3,
-    veryLongParameterName4,
-  ),
-);
+setTimeout(() => {
+  console.log("test");
+}, someFunctionCall(
+  veryLongParameterName1,
+  veryLongParameterName2,
+  veryLongParameterName3,
+  veryLongParameterName4,
+));

```

**Prettier Similarity**: 0.00%


# js/call/first-argument-expansion/issue-13237.js

**Prettier Similarity**: 100.00%


# js/call/first-argument-expansion/issue-14454.js

**Prettier Similarity**: 100.00%


# js/call/first-argument-expansion/issue-2456.js
```diff
-f(
-  (x) => {
-    y;
-  },
-  err.message.includes("asd") &&
-    err.message.includes("id") &&
-    err.message.includes('"1"') &&
-    err.message.includes("Model") &&
-    err.message.includes("/id") &&
-    err.message.includes("identifier(number)"),
-);
+f((x) => {
+  y;
+}, err.message.includes("asd") &&
+  err.message.includes("id") &&
+  err.message.includes('"1"') &&
+  err.message.includes("Model") &&
+  err.message.includes("/id") &&
+  err.message.includes("identifier(number)"));

```

**Prettier Similarity**: 0.00%


# js/call/first-argument-expansion/issue-4401.js
```diff
 export function test() {
-  setTimeout(
-    () => {
-      console.warn({}, "Lambda approaching timeout.");
-    },
-    Math.max(context.getRemainingTimeInMillis() - WARN_TIMEOUT_MS, 0),
-  );
+  setTimeout(() => {
+    console.warn({}, "Lambda approaching timeout.");
+  }, Math.max(context.getRemainingTimeInMillis() - WARN_TIMEOUT_MS, 0));
 }

```

**Prettier Similarity**: 25.00%


# js/call/first-argument-expansion/issue-5172.js
```diff
-call(
-  function () {
-    return 1;
-  },
+call(function () {
+  return 1;
+}, $var ??
+  $var ??
+  $var ??
+  $var ??
+  $var ??
+  $var ??
+  $var ??
   $var ??
-    $var ??
-    $var ??
-    $var ??
-    $var ??
-    $var ??
-    $var ??
-    $var ??
-    $var ??
-    "test",
-);
+  $var ??
+  "test");
 
-call(
-  function () {
-    return 1;
-  },
-  $var ||
-    ($var ?? $var ?? $var ?? $var ?? $var ?? $var ?? $var ?? $var ?? "test"),
-);
+call(function () {
+  return 1;
+}, $var ||
+  ($var ?? $var ?? $var ?? $var ?? $var ?? $var ?? $var ?? $var ?? "test"));

```

**Prettier Similarity**: 8.70%


# js/call/first-argument-expansion/jsx.js

**Prettier Similarity**: 100.00%


# js/call/first-argument-expansion/test.js

**Prettier Similarity**: 100.00%


# js/call/no-argument/special-cases.js

**Prettier Similarity**: 100.00%


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


# js/class-comment/class-property.js

**Prettier Similarity**: 100.00%


# js/class-comment/misc.js

**Prettier Similarity**: 100.00%


# js/class-comment/superclass.js

**Prettier Similarity**: 100.00%


# js/class-extends/complex.js

**Prettier Similarity**: 100.00%


# js/class-extends/extends.js

**Prettier Similarity**: 100.00%


# js/class-static-block/class-static-block.js

**Prettier Similarity**: 100.00%


# js/class-static-block/with-line-breaks.js

**Prettier Similarity**: 100.00%


# js/classes-private-fields/optional-chaining.js

**Prettier Similarity**: 100.00%


# js/classes-private-fields/private_fields.js

**Prettier Similarity**: 100.00%


# js/classes-private-fields/with_comments.js

**Prettier Similarity**: 100.00%


# js/classes/asi.js

**Prettier Similarity**: 100.00%


# js/classes/assignment.js

**Prettier Similarity**: 100.00%


# js/classes/binary.js

**Prettier Similarity**: 100.00%


# js/classes/call.js

**Prettier Similarity**: 100.00%


# js/classes/class-fields-features.js

**Prettier Similarity**: 100.00%


# js/classes/empty.js

**Prettier Similarity**: 100.00%


# js/classes/keyword-property/async.js

**Prettier Similarity**: 100.00%


# js/classes/keyword-property/computed.js

**Prettier Similarity**: 100.00%


# js/classes/keyword-property/get.js

**Prettier Similarity**: 100.00%


# js/classes/keyword-property/private.js

**Prettier Similarity**: 100.00%


# js/classes/keyword-property/set.js

**Prettier Similarity**: 100.00%


# js/classes/keyword-property/static-async.js

**Prettier Similarity**: 100.00%


# js/classes/keyword-property/static-get.js

**Prettier Similarity**: 100.00%


# js/classes/keyword-property/static-set.js

**Prettier Similarity**: 100.00%


# js/classes/keyword-property/static-static.js

**Prettier Similarity**: 100.00%


# js/classes/keyword-property/static.js

**Prettier Similarity**: 100.00%


# js/classes/member.js

**Prettier Similarity**: 100.00%


# js/classes/method.js

**Prettier Similarity**: 100.00%


# js/classes/new.js

**Prettier Similarity**: 100.00%


# js/classes/property.js

**Prettier Similarity**: 100.00%


# js/classes/super.js

**Prettier Similarity**: 100.00%


# js/classes/ternary.js

**Prettier Similarity**: 100.00%


# js/classes/top-level-super/example.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/binary-expr.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/closure-compiler-type-cast.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/comment-in-the-middle.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/comment-placement.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/extra-spaces-and-asterisks.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/iife-issue-5850-isolated.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/iife.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/issue-4124.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/issue-8045.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/issue-9358.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/member.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/nested.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/non-casts.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/object-with-comment.js

**Prettier Similarity**: 100.00%


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


# js/comments-closure-typecast/superclass.js

**Prettier Similarity**: 100.00%


# js/comments-closure-typecast/ways-to-specify-type.js

**Prettier Similarity**: 100.00%


# js/comments/arrow.js

**Prettier Similarity**: 100.00%


# js/comments/assignment-pattern.js

**Prettier Similarity**: 100.00%


# js/comments/before-comma.js

**Prettier Similarity**: 100.00%


# js/comments/binary-expressions-block-comments.js

**Prettier Similarity**: 100.00%


# js/comments/binary-expressions-parens.js

**Prettier Similarity**: 100.00%


# js/comments/binary-expressions-single-comments.js

**Prettier Similarity**: 100.00%


# js/comments/binary-expressions.js

**Prettier Similarity**: 100.00%


# js/comments/blank.js

**Prettier Similarity**: 100.00%


# js/comments/break-continue-statements.js
```diff
 for (;;) {
   break; /* comment */
   continue; /* comment */
 }
 
 loop: for (;;) {
-  break /* comment */ loop;
-  break loop /* comment */;
-  continue /* comment */ loop;
-  continue loop /* comment */;
+  break loop; /* comment */
+  break loop; /* comment */
+  continue loop; /* comment */
+  continue loop; /* comment */
 }

```

**Prettier Similarity**: 63.64%


# js/comments/call_comment.js

**Prettier Similarity**: 100.00%


# js/comments/class.js

**Prettier Similarity**: 100.00%


# js/comments/dangling.js

**Prettier Similarity**: 100.00%


# js/comments/dangling_array.js

**Prettier Similarity**: 100.00%


# js/comments/dangling_for.js

**Prettier Similarity**: 100.00%


# js/comments/dynamic_imports.js

**Prettier Similarity**: 100.00%


# js/comments/emoji.js

**Prettier Similarity**: 100.00%


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


# js/comments/export-and-import.js

**Prettier Similarity**: 100.00%


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


# js/comments/first-line.js

**Prettier Similarity**: 100.00%


# js/comments/flow-types/inline.js

**Prettier Similarity**: 100.00%


# js/comments/function-declaration.js

**Prettier Similarity**: 100.00%


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


# js/comments/if.js

**Prettier Similarity**: 100.00%


# js/comments/issue-3532.js

**Prettier Similarity**: 100.00%


# js/comments/issues.js

**Prettier Similarity**: 100.00%


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


# js/comments/jsdoc.js

**Prettier Similarity**: 100.00%


# js/comments/jsx.js

**Prettier Similarity**: 100.00%


# js/comments/last-arg.js

**Prettier Similarity**: 100.00%


# js/comments/multi-comments-2.js

**Prettier Similarity**: 100.00%


# js/comments/multi-comments-on-same-line-2.js

**Prettier Similarity**: 100.00%


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


# js/comments/multi-comments.js

**Prettier Similarity**: 100.00%


# js/comments/preserve-new-line-last.js

**Prettier Similarity**: 100.00%


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


# js/comments/single-star-jsdoc.js

**Prettier Similarity**: 100.00%


# js/comments/switch.js

**Prettier Similarity**: 100.00%


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


# js/comments/template-literal.js

**Prettier Similarity**: 100.00%


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


# js/comments/trailing_space.js

**Prettier Similarity**: 100.00%


# js/comments/try.js

**Prettier Similarity**: 100.00%


# js/comments/variable_declarator.js

**Prettier Similarity**: 100.00%


# js/comments/while.js

**Prettier Similarity**: 100.00%


# js/computed-props/classes.js

**Prettier Similarity**: 100.00%


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


# js/conditional/new-expression.js

**Prettier Similarity**: 100.00%


# js/conditional/new-ternary-examples.js

**Prettier Similarity**: 100.00%


# js/conditional/new-ternary-spec.js

**Prettier Similarity**: 100.00%


# js/conditional/no-confusing-arrow.js

**Prettier Similarity**: 100.00%


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


# js/cursor/comments-1.js

**Prettier Similarity**: 100.00%


# js/cursor/comments-2.js

**Prettier Similarity**: 100.00%


# js/cursor/comments-3.js

**Prettier Similarity**: 100.00%


# js/cursor/comments-4.js

**Prettier Similarity**: 100.00%


# js/cursor/cursor-0.js

**Prettier Similarity**: 100.00%


# js/cursor/cursor-1.js

**Prettier Similarity**: 100.00%


# js/cursor/cursor-10.js

**Prettier Similarity**: 100.00%


# js/cursor/cursor-2.js

**Prettier Similarity**: 100.00%


# js/cursor/cursor-3.js

**Prettier Similarity**: 100.00%


# js/cursor/cursor-4.js

**Prettier Similarity**: 100.00%


# js/cursor/cursor-5.js

**Prettier Similarity**: 100.00%


# js/cursor/cursor-6.js

**Prettier Similarity**: 100.00%


# js/cursor/cursor-7.js

**Prettier Similarity**: 100.00%


# js/cursor/cursor-8.js

**Prettier Similarity**: 100.00%


# js/cursor/cursor-9.js

**Prettier Similarity**: 100.00%


# js/cursor/cursor-emoji.js

**Prettier Similarity**: 100.00%


# js/cursor/file-start-with-comment-1.js

**Prettier Similarity**: 100.00%


# js/cursor/file-start-with-comment-2.js

**Prettier Similarity**: 100.00%


# js/cursor/file-start-with-comment-3.js

**Prettier Similarity**: 100.00%


# js/cursor/range-0.js

**Prettier Similarity**: 100.00%


# js/cursor/range-1.js

**Prettier Similarity**: 100.00%


# js/cursor/range-2.js

**Prettier Similarity**: 100.00%


# js/cursor/range-3.js

**Prettier Similarity**: 100.00%


# js/cursor/range-4.js

**Prettier Similarity**: 100.00%


# js/cursor/range-5.js

**Prettier Similarity**: 100.00%


# js/cursor/range-6.js

**Prettier Similarity**: 100.00%


# js/cursor/range-7.js

**Prettier Similarity**: 100.00%


# js/cursor/range-8.js

**Prettier Similarity**: 100.00%


# js/decorator-auto-accessors/basic.js

**Prettier Similarity**: 100.00%


# js/decorator-auto-accessors/comments.js

**Prettier Similarity**: 100.00%


# js/decorator-auto-accessors/computed.js

**Prettier Similarity**: 100.00%


# js/decorator-auto-accessors/not-accessor-method.js

**Prettier Similarity**: 100.00%


# js/decorator-auto-accessors/not-accessor-property.js

**Prettier Similarity**: 100.00%


# js/decorator-auto-accessors/private.js

**Prettier Similarity**: 100.00%


# js/decorator-auto-accessors/static-computed.js

**Prettier Similarity**: 100.00%


# js/decorator-auto-accessors/static-private.js

**Prettier Similarity**: 100.00%


# js/decorator-auto-accessors/static.js

**Prettier Similarity**: 100.00%


# js/decorator-auto-accessors/with-semicolon-1.js

**Prettier Similarity**: 100.00%


# js/decorator-auto-accessors/with-semicolon-2.js

**Prettier Similarity**: 100.00%


# js/decorators-export/after_export.js

**Prettier Similarity**: 100.00%


# js/decorators-export/before_export.js

**Prettier Similarity**: 100.00%


# js/decorators/class-expression/arguments.js

**Prettier Similarity**: 100.00%


# js/decorators/class-expression/class-expression.js

**Prettier Similarity**: 100.00%


# js/decorators/class-expression/member-expression.js

**Prettier Similarity**: 100.00%


# js/decorators/class-expression/super-class.js

**Prettier Similarity**: 100.00%


# js/decorators/classes.js

**Prettier Similarity**: 100.00%


# js/decorators/comments.js

**Prettier Similarity**: 100.00%


# js/decorators/member-expression.js

**Prettier Similarity**: 100.00%


# js/decorators/methods.js

**Prettier Similarity**: 100.00%


# js/decorators/mixed.js

**Prettier Similarity**: 100.00%


# js/decorators/mobx.js

**Prettier Similarity**: 100.00%


# js/decorators/multiline.js

**Prettier Similarity**: 100.00%


# js/decorators/multiple.js

**Prettier Similarity**: 100.00%


# js/decorators/parens.js

**Prettier Similarity**: 100.00%


# js/decorators/redux.js

**Prettier Similarity**: 100.00%


# js/destructuring-ignore/ignore.js

**Prettier Similarity**: 100.00%


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


# js/destructuring/issue-5988.js

**Prettier Similarity**: 100.00%


# js/directives/escaped.js

**Prettier Similarity**: 100.00%


# js/directives/issue-7346.js

**Prettier Similarity**: 100.00%


# js/directives/last-line-0.js

**Prettier Similarity**: 100.00%


# js/directives/last-line-1.js

**Prettier Similarity**: 100.00%


# js/directives/last-line-2.js

**Prettier Similarity**: 100.00%


# js/directives/newline.js

**Prettier Similarity**: 100.00%


# js/directives/no-newline.js

**Prettier Similarity**: 100.00%


# js/directives/test.js

**Prettier Similarity**: 100.00%


# js/dynamic-import/assertions.js

**Prettier Similarity**: 100.00%


# js/dynamic-import/test.js

**Prettier Similarity**: 100.00%


# js/empty-paren-comment/class-property.js

**Prettier Similarity**: 100.00%


# js/empty-paren-comment/class.js

**Prettier Similarity**: 100.00%


# js/empty-paren-comment/empty_paren_comment.js

**Prettier Similarity**: 100.00%


# js/empty-statement/body.js

**Prettier Similarity**: 100.00%


# js/empty-statement/no-newline.js

**Prettier Similarity**: 100.00%


# js/end-of-line/example.js

**Prettier Similarity**: 100.00%


# js/es6modules/export_default_arrow_expression.js

**Prettier Similarity**: 100.00%


# js/es6modules/export_default_call_expression.js

**Prettier Similarity**: 100.00%


# js/es6modules/export_default_class_declaration.js

**Prettier Similarity**: 100.00%


# js/es6modules/export_default_class_expression.js

**Prettier Similarity**: 100.00%


# js/es6modules/export_default_function_declaration.js

**Prettier Similarity**: 100.00%


# js/es6modules/export_default_function_declaration_async.js

**Prettier Similarity**: 100.00%


# js/es6modules/export_default_function_declaration_named.js

**Prettier Similarity**: 100.00%


# js/es6modules/export_default_function_expression.js

**Prettier Similarity**: 100.00%


# js/es6modules/export_default_function_expression_named.js

**Prettier Similarity**: 100.00%


# js/es6modules/export_default_new_expression.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/for-await-using-of-comments.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/invalid-duplicate-using-bindings.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/invalid-script-top-level-using-binding.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/using-declarations.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-await-expr-using-in.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-await-expr-using-instanceof.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-await-expr-using.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-await-using-asi-assignment.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-await-using-binding-basic.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-await-using-binding-escaped.js
```diff
 async function f() {
-  await using ab = c;
+  await using \u0061b = c;
 }

```

**Prettier Similarity**: 66.67%


# js/explicit-resource-management/valid-await-using-binding-non-bmp.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-await-using-binding-using.js

**Prettier Similarity**: 100.00%


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


# js/explicit-resource-management/valid-for-await-using-binding-escaped-of-of.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-for-using-binding-escaped-of-of.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-for-using-binding-of-of.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-for-using-declaration.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-module-block-top-level-await-using-binding.js
```diff
-const m = module {
+const m = module;
+{
   await using foo = bar();
-};
+}

```

**Prettier Similarity**: 25.00%


# js/explicit-resource-management/valid-module-block-top-level-using-binding.js
```diff
-module {
+module;
+{
   using foo = bar();
-};
+}

```

**Prettier Similarity**: 25.00%


# js/explicit-resource-management/valid-using-as-identifier-computed-member.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-using-as-identifier-expression-statement.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-using-as-identifier-for-await-of.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-using-as-identifier-for-in.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-using-as-identifier-for-init.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-using-as-identifier-for-of.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-using-as-identifier-in.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-using-binding-basic.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-using-binding-escaped.js
```diff
 {
-  using ab = c;
+  using \u0061b = c;
 }

```

**Prettier Similarity**: 66.67%


# js/explicit-resource-management/valid-using-binding-non-bmp.js

**Prettier Similarity**: 100.00%


# js/explicit-resource-management/valid-using-binding-using.js

**Prettier Similarity**: 100.00%


# js/export-default/binary_and_template.js

**Prettier Similarity**: 100.00%


# js/export-default/body.js

**Prettier Similarity**: 100.00%


# js/export-default/class_instance.js

**Prettier Similarity**: 100.00%


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


# js/export-default/function_in_template.js

**Prettier Similarity**: 100.00%


# js/export-default/function_tostring.js

**Prettier Similarity**: 100.00%


# js/export-default/iife.js

**Prettier Similarity**: 100.00%


# js/export-star/export-star-as-default.js

**Prettier Similarity**: 100.00%


# js/export-star/export-star-as-reserved-word.js

**Prettier Similarity**: 100.00%


# js/export-star/export-star-as-string.js

**Prettier Similarity**: 100.00%


# js/export-star/export-star-as-string2.js

**Prettier Similarity**: 100.00%


# js/export-star/export-star-as.js

**Prettier Similarity**: 100.00%


# js/export-star/export-star.js

**Prettier Similarity**: 100.00%


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


# js/export/bracket.js

**Prettier Similarity**: 100.00%


# js/export/empty.js

**Prettier Similarity**: 100.00%


# js/export/same-local-and-exported.js

**Prettier Similarity**: 100.00%


# js/export/test.js

**Prettier Similarity**: 100.00%


# js/export/undefined.js

**Prettier Similarity**: 100.00%


# js/expression_statement/no_regression.js

**Prettier Similarity**: 100.00%


# js/expression_statement/use_strict.js

**Prettier Similarity**: 100.00%


# js/for-await/for-await.js

**Prettier Similarity**: 100.00%


# js/for-of/async-identifier.js

**Prettier Similarity**: 100.00%


# js/for/comment.js

**Prettier Similarity**: 100.00%


# js/for/continue-and-break-comment-1.js
```diff
 for (;;) {
   continue; // comment
 }
 
 for (;;) {
   break; // comment
 }
 
 for (const f of []) {
   continue; // comment
 }
 
 for (const f of []) {
   break; // comment
 }
 
 for (const f in {}) {
   continue; // comment
 }
 
 for (const f in {}) {
   break; // comment
 }
 
 while (true) {
   continue; // comment
 }
 
 while (true) {
   break; // comment
 }
 
 do {
   continue; // comment
 } while (true);
 
 do {
   break; // comment
 } while (true);
 
 label1: for (;;) {
   continue label1; // comment
 }
 
 label2: {
   break label2; // comment
 }
 
 for (;;) {
   continue; /* comment */
 }
 
 for (;;) {
   break; /* comment */
 }
 
 for (const f of []) {
   continue; /* comment */
 }
 
 for (const f of []) {
   break; /* comment */
 }
 
 for (const f in {}) {
   continue; /* comment */
 }
 
 for (const f in {}) {
   break; /* comment */
 }
 
 while (true) {
   continue; /* comment */
 }
 
 while (true) {
   break; /* comment */
 }
 
 do {
   continue; /* comment */
 } while (true);
 
 do {
   break; /* comment */
 } while (true);
 
 label1: for (;;) {
-  continue label1 /* comment */;
+  continue label1; /* comment */
 }
 
 label2: {
-  break label2 /* comment */;
+  break label2; /* comment */
 }

```

**Prettier Similarity**: 97.89%


# js/for/continue-and-break-comment-2.js

**Prettier Similarity**: 100.00%


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
 
-label1: for (;;) continue label1 /* comment */;
+label1: for (;;) continue label1; /* comment */
 
 label1: for (;;) continue label1;
 /* comment */
 
 label1: for (;;) continue label1; // comment
 
 label1: for (;;) continue label1;
 // comment

```

**Prettier Similarity**: 98.55%


# js/for/for.js

**Prettier Similarity**: 100.00%


# js/for/in.js

**Prettier Similarity**: 100.00%


# js/for/var.js

**Prettier Similarity**: 100.00%


# js/function-comments/params-trail-comments.js

**Prettier Similarity**: 100.00%


# js/function-first-param/function_expression.js

**Prettier Similarity**: 100.00%


# js/function-single-destructuring/array.js

**Prettier Similarity**: 100.00%


# js/function-single-destructuring/object.js

**Prettier Similarity**: 100.00%


# js/function/function_expression.js

**Prettier Similarity**: 100.00%


# js/function/issue-10277.js

**Prettier Similarity**: 100.00%


# js/functional-composition/functional_compose.js

**Prettier Similarity**: 100.00%


# js/functional-composition/gobject_connect.js

**Prettier Similarity**: 100.00%


# js/functional-composition/lodash_flow.js

**Prettier Similarity**: 100.00%


# js/functional-composition/lodash_flow_right.js

**Prettier Similarity**: 100.00%


# js/functional-composition/mongo_connect.js

**Prettier Similarity**: 100.00%


# js/functional-composition/pipe-function-calls-with-comments.js

**Prettier Similarity**: 100.00%


# js/functional-composition/pipe-function-calls.js

**Prettier Similarity**: 100.00%


# js/functional-composition/ramda_compose.js

**Prettier Similarity**: 100.00%


# js/functional-composition/ramda_pipe.js

**Prettier Similarity**: 100.00%


# js/functional-composition/redux_compose.js

**Prettier Similarity**: 100.00%


# js/functional-composition/redux_connect.js

**Prettier Similarity**: 100.00%


# js/functional-composition/reselect_createselector.js

**Prettier Similarity**: 100.00%


# js/functional-composition/rxjs_pipe.js

**Prettier Similarity**: 100.00%


# js/generator/anonymous.js

**Prettier Similarity**: 100.00%


# js/generator/async.js

**Prettier Similarity**: 100.00%


# js/generator/function-name-starts-with-get.js

**Prettier Similarity**: 100.00%


# js/identifier/for-of/await.js

**Prettier Similarity**: 100.00%


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


# js/if/comment_before_else.js

**Prettier Similarity**: 100.00%


# js/if/else.js

**Prettier Similarity**: 100.00%


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


# js/if/if_comments.js

**Prettier Similarity**: 100.00%


# js/if/trailing_comment.js

**Prettier Similarity**: 100.00%


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


# js/ignore/decorator.js

**Prettier Similarity**: 100.00%


# js/ignore/ignore-2.js

**Prettier Similarity**: 100.00%


# js/ignore/ignore.js

**Prettier Similarity**: 100.00%


# js/ignore/issue-10661.js

**Prettier Similarity**: 100.00%


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


# js/ignore/issue-9335.js

**Prettier Similarity**: 100.00%


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


# js/ignore/semi/asi.js

**Prettier Similarity**: 100.00%


# js/ignore/semi/directive.js

**Prettier Similarity**: 100.00%


# js/import-assertions/bracket-spacing/dynamic-import.js

**Prettier Similarity**: 100.00%


# js/import-assertions/bracket-spacing/empty.js

**Prettier Similarity**: 100.00%


# js/import-assertions/bracket-spacing/re-export.js

**Prettier Similarity**: 100.00%


# js/import-assertions/bracket-spacing/static-import.js

**Prettier Similarity**: 100.00%


# js/import-assertions/dynamic-import.js

**Prettier Similarity**: 100.00%


# js/import-assertions/empty.js

**Prettier Similarity**: 100.00%


# js/import-assertions/multi-types.js

**Prettier Similarity**: 100.00%


# js/import-assertions/non-type.js

**Prettier Similarity**: 100.00%


# js/import-assertions/not-import-assertions.js

**Prettier Similarity**: 100.00%


# js/import-assertions/re-export.js

**Prettier Similarity**: 100.00%


# js/import-assertions/static-import.js

**Prettier Similarity**: 100.00%


# js/import-assertions/without-from.js

**Prettier Similarity**: 100.00%


# js/import-attributes/bracket-spacing/dynamic-import.js

**Prettier Similarity**: 100.00%


# js/import-attributes/bracket-spacing/empty.js

**Prettier Similarity**: 100.00%


# js/import-attributes/bracket-spacing/re-export.js

**Prettier Similarity**: 100.00%


# js/import-attributes/bracket-spacing/static-import.js

**Prettier Similarity**: 100.00%


# js/import-attributes/dynamic-import.js

**Prettier Similarity**: 100.00%


# js/import-attributes/empty.js

**Prettier Similarity**: 100.00%


# js/import-attributes/multi-types.js

**Prettier Similarity**: 100.00%


# js/import-attributes/non-type.js

**Prettier Similarity**: 100.00%


# js/import-attributes/re-export.js

**Prettier Similarity**: 100.00%


# js/import-attributes/static-import.js

**Prettier Similarity**: 100.00%


# js/import-attributes/without-from.js

**Prettier Similarity**: 100.00%


# js/import-meta/import_meta.js

**Prettier Similarity**: 100.00%


# js/import/brackets.js

**Prettier Similarity**: 100.00%


# js/import/comments.js

**Prettier Similarity**: 100.00%


# js/import/empty-import.js

**Prettier Similarity**: 100.00%


# js/import/inline.js

**Prettier Similarity**: 100.00%


# js/import/long-line.js

**Prettier Similarity**: 100.00%


# js/import/multiple_standalones.js

**Prettier Similarity**: 100.00%


# js/import/same-local-and-imported.js

**Prettier Similarity**: 100.00%


# js/in/arrow-function-invalid.js

**Prettier Similarity**: 100.00%


# js/in/arrow-function.js

**Prettier Similarity**: 100.00%


# js/invalid-code/duplicate_bindings.js

**Prettier Similarity**: 100.00%


# js/label/block-statement-and-regexp.js

**Prettier Similarity**: 100.00%


# js/label/comment.js

**Prettier Similarity**: 100.00%


# js/label/empty_label.js

**Prettier Similarity**: 100.00%


# js/last-argument-expansion/arrow.js

**Prettier Similarity**: 100.00%


# js/last-argument-expansion/assignment-pattern.js

**Prettier Similarity**: 100.00%


# js/last-argument-expansion/break-parent.js

**Prettier Similarity**: 100.00%


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


# js/last-argument-expansion/edge_case.js

**Prettier Similarity**: 100.00%


# js/last-argument-expansion/embed.js
```diff
-foo(/* HTML */ `<!-- bar1 -->
-    bar
-    <!-- bar2 -->`);
-foo(/* HTML */ `
-  <!-- bar1 -->
-  bar
-  <!-- bar2 -->
-`);
-foo(/* HTML */ `<div>
-    <p>bar</p>
-    foo
-  </div>`);
-foo(/* HTML */ `
-  <div>
-    <p>bar</p>
-    foo
-  </div>
-`);
-foo(/* GraphQL */ `
-  query {
-    foo {
-      bar
-    }
-  }
-`);
-foo(/* ... */ css`
-  color: magenta;
-`);
-const a = (b) => /* HTML */ `<!-- bar1 -->
-    bar
-    <!-- bar2 -->`;
-const c = (b) => /* HTML */ `
-  <!-- bar1 -->
-  bar
-  <!-- bar2 -->
-`;
+foo(/* HTML */ `<!-- bar1 --> bar <!-- bar2 -->`);
+foo(/* HTML */ ` <!-- bar1 --> bar <!-- bar2 --> `);
+foo(/* HTML */ `<div><p>bar</p>foo</div>`);
+foo(/* HTML */ ` <div><p>bar</p>foo</div> `);
+foo(/* GraphQL */ `query { foo { bar } }`);
+foo(/* ... */ css`color:magenta`);
+const a = (b) => /* HTML */ `<!-- bar1 --> bar <!-- bar2 -->`;
+const c = (b) => /* HTML */ ` <!-- bar1 --> bar <!-- bar2 --> `;

```

**Prettier Similarity**: 0.00%


# js/last-argument-expansion/empty-lines.js

**Prettier Similarity**: 100.00%


# js/last-argument-expansion/empty-object.js

**Prettier Similarity**: 100.00%


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


# js/last-argument-expansion/function-expression-issue-2239.js

**Prettier Similarity**: 100.00%


# js/last-argument-expansion/function-expression.js

**Prettier Similarity**: 100.00%


# js/last-argument-expansion/issue-10708.js

**Prettier Similarity**: 100.00%


# js/last-argument-expansion/issue-7518.js

**Prettier Similarity**: 100.00%


# js/last-argument-expansion/jsx.js

**Prettier Similarity**: 100.00%


# js/last-argument-expansion/number-only-array.js

**Prettier Similarity**: 100.00%


# js/last-argument-expansion/object.js

**Prettier Similarity**: 100.00%


# js/last-argument-expansion/overflow.js

**Prettier Similarity**: 100.00%


# js/line-suffix-boundary/boundary.js

**Prettier Similarity**: 100.00%


# js/line/windows.js

**Prettier Similarity**: 100.00%


# js/literal-numeric-separator/test.js

**Prettier Similarity**: 100.00%


# js/literal/number.js

**Prettier Similarity**: 100.00%


# js/logical-assignment/logical-assignment.js

**Prettier Similarity**: 100.00%


# js/logical_expressions/issue-7024.js

**Prettier Similarity**: 100.00%


# js/logical_expressions/logical_expression_operators.js

**Prettier Similarity**: 100.00%


# js/member/conditional.js

**Prettier Similarity**: 100.00%


# js/member/expand.js

**Prettier Similarity**: 100.00%


# js/member/logical.js

**Prettier Similarity**: 100.00%


# js/method-chain/13018.js

**Prettier Similarity**: 100.00%


# js/method-chain/bracket_0-1.js

**Prettier Similarity**: 100.00%


# js/method-chain/bracket_0.js

**Prettier Similarity**: 100.00%


# js/method-chain/break-last-call.js

**Prettier Similarity**: 100.00%


# js/method-chain/break-last-member.js

**Prettier Similarity**: 100.00%


# js/method-chain/break-multiple.js

**Prettier Similarity**: 100.00%


# js/method-chain/comment.js

**Prettier Similarity**: 100.00%


# js/method-chain/complex-args.js

**Prettier Similarity**: 100.00%


# js/method-chain/computed-merge.js

**Prettier Similarity**: 100.00%


# js/method-chain/computed.js

**Prettier Similarity**: 100.00%


# js/method-chain/conditional.js

**Prettier Similarity**: 100.00%


# js/method-chain/cypress.js

**Prettier Similarity**: 100.00%


# js/method-chain/d3.js

**Prettier Similarity**: 100.00%


# js/method-chain/first_long.js

**Prettier Similarity**: 100.00%


# js/method-chain/fluent-configuration.js

**Prettier Similarity**: 100.00%


# js/method-chain/inline_merge.js

**Prettier Similarity**: 100.00%


# js/method-chain/issue-11298.js

**Prettier Similarity**: 100.00%


# js/method-chain/issue-3594.js

**Prettier Similarity**: 100.00%


# js/method-chain/issue-3621.js

**Prettier Similarity**: 100.00%


# js/method-chain/issue-4125.js

**Prettier Similarity**: 100.00%


# js/method-chain/logical.js

**Prettier Similarity**: 100.00%


# js/method-chain/multiple-members.js

**Prettier Similarity**: 100.00%


# js/method-chain/object-literal.js

**Prettier Similarity**: 100.00%


# js/method-chain/pr-7889.js

**Prettier Similarity**: 100.00%


# js/method-chain/print-width-120/constructor.js

**Prettier Similarity**: 100.00%


# js/method-chain/print-width-120/issue-7884.js

**Prettier Similarity**: 100.00%


# js/method-chain/short-names.js

**Prettier Similarity**: 100.00%


# js/method-chain/simple-args.js

**Prettier Similarity**: 100.00%


# js/method-chain/square_0.js

**Prettier Similarity**: 100.00%


# js/method-chain/test.js

**Prettier Similarity**: 100.00%


# js/method-chain/this.js

**Prettier Similarity**: 100.00%


# js/module-string-names/module-string-names-export.js

**Prettier Similarity**: 100.00%


# js/module-string-names/module-string-names-import.js

**Prettier Similarity**: 100.00%


# js/new-expression/call.js

**Prettier Similarity**: 100.00%


# js/new-expression/new_expression.js

**Prettier Similarity**: 100.00%


# js/new-expression/with-member-expression.js

**Prettier Similarity**: 100.00%


# js/new-target/outside-functions.js

**Prettier Similarity**: 100.00%


# js/new-target/range.js

**Prettier Similarity**: 100.00%


# js/newline/backslash_2028.js

**Prettier Similarity**: 100.00%


# js/newline/backslash_2029.js

**Prettier Similarity**: 100.00%


# js/no-semi-babylon-extensions/no-semi.js
```diff
 a;
-::b.c;
+::b.c
 
 class A {
   a = b;
   in;
   c;
 
   a = b;
   instanceof() {}
 }

```

**Prettier Similarity**: 90.91%


# js/no-semi/class.js

**Prettier Similarity**: 100.00%


# js/no-semi/comments.js

**Prettier Similarity**: 100.00%


# js/no-semi/issue2006.js

**Prettier Similarity**: 100.00%


# js/no-semi/no-semi.js

**Prettier Similarity**: 100.00%


# js/no-semi/private-field.js

**Prettier Similarity**: 100.00%


# js/non-strict/argument-name-clash.js

**Prettier Similarity**: 100.00%


# js/non-strict/keywords.js

**Prettier Similarity**: 100.00%


# js/non-strict/octal-number.js

**Prettier Similarity**: 100.00%


# js/nullish-coalescing/nullish_coalesing_operator.js

**Prettier Similarity**: 100.00%


# js/numeric-separators/number.js

**Prettier Similarity**: 100.00%


# js/object-colon-bug/bug.js

**Prettier Similarity**: 100.00%


# js/object-prop-break-in/comment.js

**Prettier Similarity**: 100.00%


# js/object-prop-break-in/long-value.js

**Prettier Similarity**: 100.00%


# js/object-prop-break-in/short-keys.js

**Prettier Similarity**: 100.00%


# js/object-prop-break-in/test.js

**Prettier Similarity**: 100.00%


# js/object-property-comment/after-key.js

**Prettier Similarity**: 100.00%


# js/object-property-ignore/ignore.js

**Prettier Similarity**: 100.00%


# js/object-property-ignore/issue-5678.js

**Prettier Similarity**: 100.00%


# js/objects/assignment-expression/object-property.js
```diff
 a = {
-  [(this.resource = resource)]: 1,
+  [this.resource = resource]: 1,
 };

```

**Prettier Similarity**: 66.67%


# js/objects/assignment-expression/object-value.js

**Prettier Similarity**: 100.00%


# js/objects/bigint-key.js

**Prettier Similarity**: 100.00%


# js/objects/escape-sequence-key.js

**Prettier Similarity**: 100.00%


# js/objects/expand.js

**Prettier Similarity**: 100.00%


# js/objects/expression.js
```diff
 () => ({})``;
 ({})``;
 a = () => ({}).x;
 ({}) && a, b;
-({})::b, 0;
-({})::b()``[""].c++ && 0 ? 0 : 0, 0;
+({}
+::b, 0)
+({}
+::b()``[''].c++ && 0 ? 0 : 0, 0)
 ({})(), 0;
 ({} = 0);
 ({} = 0), 1;
 
 const a1 = {
   someKey: (shortName, shortName),
 };
 
 const a2 = {
   someKey:
     (longLongLongLongLongLongLongLongLongLongLongLongLongLongName, shortName),
 };
 
 const a3 = {
   someKey:
     (longLongLongLongLongLongLongLongLongLongLongLongLongLongName,
     longLongLongLongLongLongLongLongLongLongLongLongLongLongName,
     longLongLongLongLongLongLongLongLongLongLongLongLongLongName),
 };

```

**Prettier Similarity**: 85.19%


# js/objects/getter-setter.js

**Prettier Similarity**: 100.00%


# js/objects/method.js

**Prettier Similarity**: 100.00%


# js/objects/range.js

**Prettier Similarity**: 100.00%


# js/objects/right-break.js

**Prettier Similarity**: 100.00%


# js/optional-catch-binding/optional_catch_binding.js

**Prettier Similarity**: 100.00%


# js/optional-chaining-assignment/valid-complex-case.js

**Prettier Similarity**: 100.00%


# js/optional-chaining-assignment/valid-lhs-eq.js

**Prettier Similarity**: 100.00%


# js/optional-chaining-assignment/valid-lhs-plus-eq.js

**Prettier Similarity**: 100.00%


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


# js/optional-chaining/comments.js

**Prettier Similarity**: 100.00%


# js/optional-chaining/eval.js

**Prettier Similarity**: 100.00%


# js/performance/nested-real.js

**Prettier Similarity**: 100.00%


# js/performance/nested.js

**Prettier Similarity**: 100.00%


# js/preserve-line/argument-list.js

**Prettier Similarity**: 100.00%


# js/preserve-line/comments.js

**Prettier Similarity**: 100.00%


# js/preserve-line/member-chain.js

**Prettier Similarity**: 100.00%


# js/preserve-line/parameter-list.js

**Prettier Similarity**: 100.00%


# js/private-in/private-in.js

**Prettier Similarity**: 100.00%


# js/quote-props/classes.js

**Prettier Similarity**: 100.00%


# js/quote-props/numeric-separator.js

**Prettier Similarity**: 100.00%


# js/quote-props/objects.js

**Prettier Similarity**: 100.00%


# js/quote-props/with_member_expressions.js

**Prettier Similarity**: 100.00%


# js/quote-props/with_numbers.js

**Prettier Similarity**: 100.00%


# js/quotes/functions.js

**Prettier Similarity**: 100.00%


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


# js/quotes/strings.js

**Prettier Similarity**: 100.00%


# js/range/array.js
```diff
-a = [, , , , , , , a];
+a = [, , , , , , , a];

```

**Prettier Similarity**: 0.00%


# js/range/boundary-2.js
```diff
-function a() {
-  a();
-  b();
-  c();
-  d();
+function a(
+){
+a (
+);
+b();
+c(); d(
+);
+
 }

```

**Prettier Similarity**: 11.11%


# js/range/boundary-3.js
```diff
-a();
+a (
+);
 b();
-c();
-d();
+c(); d(
+);

```

**Prettier Similarity**: 20.00%


# js/range/boundary.js
```diff
-foo = 1.0;
-bar = 1.0;
-baz = 1.0;
+foo = 1.0000;bar = 1.0;baz=1.0000;
 // The range will be 13~26
 // `foo` ends at 13, should not format
 // `bar` ends at 26, should format

```

**Prettier Similarity**: 50.00%


# js/range/class-declaration.js
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


# js/range/different-levels.js
```diff
-call(1, 2, 3);
+call(1,2,3)
 call(1, 2, 3);
 function f() {
   call(1, 2, 3);
 }

```

**Prettier Similarity**: 80.00%


# js/range/directive.js
```diff
 "aaa";
-"bbb";
+'bbb';

```

**Prettier Similarity**: 50.00%


# js/range/function-body.js
```diff
-let fn = a((x) => {
+let fn =a((x ) => {
   quux(); //
 });

```

**Prettier Similarity**: 66.67%


# js/range/function-declaration.js

**Prettier Similarity**: 100.00%


# js/range/ignore-indentation.js
```diff
-function ugly({ a = 1, b = 2 }) {
-  function ugly({ a = 1, b = 2 }) {
-    function ugly({ a = 1, b = 2 }) {
-      `multiline template string
+function ugly ( {a=1,     b     =   2     }      ) {
+  function ugly ( {a=1,     b     =   2     }      ) {
+    function ugly ( {a=1,     b     =   2     }      ) {
+  	  	     `multiline template string
               with too much indentation`;
     }
   }
 }

```

**Prettier Similarity**: 50.00%


# js/range/issue-3789-1.js

**Prettier Similarity**: 100.00%


# js/range/issue-3789-2.js

**Prettier Similarity**: 100.00%


# js/range/issue-4206-1.js

**Prettier Similarity**: 100.00%


# js/range/issue-4206-2.js

**Prettier Similarity**: 100.00%


# js/range/issue-4206-3.js

**Prettier Similarity**: 100.00%


# js/range/issue-4206-4.js

**Prettier Similarity**: 100.00%


# js/range/large-dict.js

**Prettier Similarity**: 100.00%


# js/range/module-export1.js
```diff
-import def, { named } from "x";
+import  def , {named}  from    'x'
 
 export * from "d";
 
-export const x = 42;
+export    const  x
+  =  42
+
+export   default    42
 
-export default 42;

```

**Prettier Similarity**: 44.44%


# js/range/module-export2.js
```diff
-import def, { named } from "x";
+import  def , {named}  from    'x'
 
-export * from "d";
+export *  from   'd'
 
 export const x = 42;
 
-export default 42;
+export   default    42
+

```

**Prettier Similarity**: 50.00%


# js/range/module-export3.js
```diff
-import def, { named } from "x";
+import  def , {named}  from    'x'
 
-export * from "d";
+export *  from   'd'
 
-export const x = 42;
+export    const  x
+  =  42
 
 export default 42;
+

```

**Prettier Similarity**: 44.44%


# js/range/module-import.js
```diff
 import def, { named } from "x";
 
-export * from "d";
+export *  from   'd'
+
+export    const  x
+  =  42
 
-export const x = 42;
+export   default    42
 
-export default 42;

```

**Prettier Similarity**: 44.44%


# js/range/multiple-statements.js
```diff
-call(1, 2, 3);
+call(
+  1, 2,3
+);
 
 call(1, 2, 3);
 
 call(1, 2, 3);
 
-call(1, 2, 3);
+call(
+  1, 2,3
+);

```

**Prettier Similarity**: 45.45%


# js/range/multiple-statements2.js
```diff
-call(1, 2, 3);
+call(
+  1, 2,3
+);
 
 call(1, 2, 3);
 
 call(1, 2, 3);
 
-call(1, 2, 3);
+call(
+  1, 2,3
+);

```

**Prettier Similarity**: 45.45%


# js/range/nested-print-width.js

**Prettier Similarity**: 100.00%


# js/range/nested.js

**Prettier Similarity**: 100.00%


# js/range/nested2.js
```diff
 try {
   if (condition) {
     body;
   }
-} catch (err) {}
+}
+catch (err) {}

```

**Prettier Similarity**: 66.67%


# js/range/nested3.js
```diff
 try {
-  1;
-  if (condition) {
-    body;
-  }
-} catch (err) {}
+1;
+if (condition) {
+  body;
+}
+}
+catch (err) {}

```

**Prettier Similarity**: 14.29%


# js/range/object-expression.js

**Prettier Similarity**: 100.00%


# js/range/object-expression2.js
```diff
+
 const y = [
   {
     a: 1,
   },
   {
     a: 1,
     b: 2,
   },
 ];

```

**Prettier Similarity**: 90.00%


# js/range/range-end.js
```diff
 // Unchanged
-call(1, 2, 3);
+call(
+  1, 2,3
+);
+
 
-call(1, 2, 3);
+call(1, 2, 3);

```

**Prettier Similarity**: 28.57%


# js/range/range-start.js
```diff
 call(1, 2, 3);
 
+
 // Unchanged
-call(1, 2, 3);
+call(
+  1, 2,3
+);

```

**Prettier Similarity**: 42.86%


# js/range/range.js
```diff
-function ugly({ a = 1, b = 2 }) {
-  function ugly({ a = 1, b = 2 }) {
-    function ugly({ a = 1, b = 2 }) {
-      `multiline template string
+function ugly ( {a=1,     b     =   2     }      ) {
+  function ugly ( {a=1,     b     =   2     }      ) {
+    function ugly ( {a=1,     b     =   2     }      ) {
+             `multiline template string
               with too much indentation`;
     }
   }
 }

```

**Prettier Similarity**: 50.00%


# js/range/start-equals-end.js
```diff
-foo = 1.0;
-bar = 1.0;
-baz = 1.0;
+foo = 1.0000;bar = 1.0000;baz=1.0000;
 // The range will be 13~13, should not format anything

```

**Prettier Similarity**: 25.00%


# js/range/try-catch.js

**Prettier Similarity**: 100.00%


# js/range/whitespace.js

**Prettier Similarity**: 100.00%


# js/regex/d-flag.js

**Prettier Similarity**: 100.00%


# js/regex/multiple-flags.js

**Prettier Similarity**: 100.00%


# js/regex/regexp-modifiers.js

**Prettier Similarity**: 100.00%


# js/regex/test.js

**Prettier Similarity**: 100.00%


# js/regex/v-flag.js

**Prettier Similarity**: 100.00%


# js/require-amd/named-amd-module.js

**Prettier Similarity**: 100.00%


# js/require-amd/non-amd-define.js

**Prettier Similarity**: 100.00%


# js/require-amd/require.js

**Prettier Similarity**: 100.00%


# js/require/require.js

**Prettier Similarity**: 100.00%


# js/reserved-word/interfaces.js

**Prettier Similarity**: 100.00%


# js/rest/trailing-commas.js

**Prettier Similarity**: 100.00%


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


# js/return/binaryish.js

**Prettier Similarity**: 100.00%


# js/return/comment.js

**Prettier Similarity**: 100.00%


# js/sequence-break/break.js

**Prettier Similarity**: 100.00%


# js/sequence-expression/export-default.js

**Prettier Similarity**: 100.00%


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


# js/shebang/shebang-newline.js

**Prettier Similarity**: 100.00%


# js/shebang/shebang.js

**Prettier Similarity**: 100.00%


# js/sloppy-mode/delete-variable.js

**Prettier Similarity**: 100.00%


# js/sloppy-mode/eval-arguments-binding.js

**Prettier Similarity**: 100.00%


# js/sloppy-mode/eval-arguments.js

**Prettier Similarity**: 100.00%


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


# js/sloppy-mode/labeled-function-declaration.js

**Prettier Similarity**: 100.00%


# js/spread/spread.js

**Prettier Similarity**: 100.00%


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


# js/strings/multiline-literal.js

**Prettier Similarity**: 100.00%


# js/strings/non-octal-eight-and-nine.js

**Prettier Similarity**: 100.00%


# js/strings/strings.js

**Prettier Similarity**: 100.00%


# js/strings/template-literals.js

**Prettier Similarity**: 100.00%


# js/switch/comments.js

**Prettier Similarity**: 100.00%


# js/switch/comments2.js
```diff
 switch (1) {
-  default: // comment1
+  default:
+  // comment1
 }
 
 switch (2) {
-  default: // comment2
+  default:
+  // comment2
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

**Prettier Similarity**: 62.07%


# js/switch/empty_lines.js

**Prettier Similarity**: 100.00%


# js/switch/empty_statement.js

**Prettier Similarity**: 100.00%


# js/switch/empty_switch.js

**Prettier Similarity**: 100.00%


# js/switch/switch.js

**Prettier Similarity**: 100.00%


# js/tab-width/class.js

**Prettier Similarity**: 100.00%


# js/tab-width/nested-functions.spec.js

**Prettier Similarity**: 100.00%


# js/template-align/indent.js

**Prettier Similarity**: 100.00%


# js/template-literals/binary-exporessions.js

**Prettier Similarity**: 100.00%


# js/template-literals/conditional-expressions.js

**Prettier Similarity**: 100.00%


# js/template-literals/expressions.js

**Prettier Similarity**: 100.00%


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


# js/template-literals/logical-expressions.js

**Prettier Similarity**: 100.00%


# js/template-literals/sequence-expressions.js

**Prettier Similarity**: 100.00%


# js/template/arrow.js

**Prettier Similarity**: 100.00%


# js/template/call.js

**Prettier Similarity**: 100.00%


# js/template/comment.js

**Prettier Similarity**: 100.00%


# js/template/faulty-locations.js

**Prettier Similarity**: 100.00%


# js/template/graphql.js

**Prettier Similarity**: 100.00%


# js/template/indent.js

**Prettier Similarity**: 100.00%


# js/template/inline.js

**Prettier Similarity**: 100.00%


# js/template/parenthesis.js

**Prettier Similarity**: 100.00%


# js/ternaries/binary.js

**Prettier Similarity**: 100.00%


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


# js/ternaries/indent-after-paren.js

**Prettier Similarity**: 100.00%


# js/ternaries/indent.js

**Prettier Similarity**: 100.00%


# js/ternaries/nested-in-condition.js

**Prettier Similarity**: 100.00%


# js/ternaries/nested.js

**Prettier Similarity**: 100.00%


# js/ternaries/parenthesis.js

**Prettier Similarity**: 100.00%


# js/ternaries/test.js

**Prettier Similarity**: 100.00%


# js/test-declarations/angular_async.js

**Prettier Similarity**: 100.00%


# js/test-declarations/angular_fakeAsync.js

**Prettier Similarity**: 100.00%


# js/test-declarations/angular_waitForAsync.js

**Prettier Similarity**: 100.00%


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


# js/test-declarations/jest-each-template-string.js

**Prettier Similarity**: 100.00%


# js/test-declarations/jest-each.js

**Prettier Similarity**: 100.00%


# js/test-declarations/test_declarations.js
```diff
 // Shouldn't break
 
 it("does something really long and complicated so I have to write a very long name for the test", () => {
   console.log("hello!");
 });
 
 it("does something really long and complicated so I have to write a very long name for the test", function () {
   console.log("hello!");
 });
 
 it("does something really long and complicated so I have to write a very long name for the test", function (done) {
   console.log("hello!");
 });
 
 it("does something really long and complicated so I have to write a very long name for the test", function myAssertions(done) {
   console.log("hello!");
 });
 
 it(`does something really long and complicated so I have to write a very long name for the test`, function () {
   console.log("hello!");
 });
 
 it(`{foo + bar} does something really long and complicated so I have to write a very long name for the test`, function () {
   console.log("hello!");
 });
 
 it(`handles
   some
     newlines
   does something really long and complicated so I have to write a very long name for the test`, () => {
   console.log("hello!");
 });
 
 test("does something really long and complicated so I have to write a very long name for the test", (done) => {
   console.log("hello!");
 });
 
 test(`does something really long and complicated so I have to write a very long name for the test`, (done) => {
   console.log("hello!");
 });
 
 describe("does something really long and complicated so I have to write a very long name for the describe block", () => {
   it("an example test", (done) => {
     console.log("hello!");
   });
 });
 
 describe(`does something really long and complicated so I have to write a very long name for the describe block`, () => {
   it(`an example test`, (done) => {
     console.log("hello!");
   });
 });
 
 xdescribe("does something really long and complicated so I have to write a very long name for the describe block", () => {});
 
 fdescribe("does something really long and complicated so I have to write a very long name for the describe block", () => {});
 
 describe.only(`does something really long and complicated so I have to write a very long name for the test`, () => {});
 
 describe.skip(`does something really long and complicated so I have to write a very long name for the test`, () => {});
 
 fit("does something really long and complicated so I have to write a very long name for the describe block", () => {});
 
 xit("does something really long and complicated so I have to write a very long name for the describe block", () => {});
 
 it.only("does something really long and complicated so I have to write a very long name for the test", () => {
   console.log("hello!");
 });
 
 it.only(`does something really long and complicated so I have to write a very long name for the test`, () => {
   console.log("hello!");
 });
 
 it.skip(`does something really long and complicated so I have to write a very long name for the test`, () => {});
 
 test.only(`does something really long and complicated so I have to write a very long name for the test`, () => {});
 
 test.skip(`does something really long and complicated so I have to write a very long name for the test`, () => {});
 
 ftest("does something really long and complicated so I have to write a very long name for the describe block", () => {});
 
 xtest("does something really long and complicated so I have to write a very long name for the describe block", () => {});
 
 skip(`does something really long and complicated so I have to write a very long name for the test`, () => {});
 
 skip("does something really long and complicated so I have to write a very long name for the test", () => {});
 
 test.step("does something really long and complicated so I have to write a very long name for the test", () => {});
 
 test.step(`does something really long and complicated so I have to write a very long name for the test`, () => {});
 
 test.describe("does something really long and complicated so I have to write a very long name for the test", () => {});
 
 test.describe(`does something really long and complicated so I have to write a very long name for the test`, () => {});
 
-test.describe
-  .only("does something really long and complicated so I have to write a very long name for the test", () => {});
+test.describe.only("does something really long and complicated so I have to write a very long name for the test", () => {});
 
-test.describe
-  .only(`does something really long and complicated so I have to write a very long name for the test`, () => {});
+test.describe.only(`does something really long and complicated so I have to write a very long name for the test`, () => {});
 
-test.describe
-  .parallel("does something really long and complicated so I have to write a very long name for the test", () => {});
+test.describe.parallel("does something really long and complicated so I have to write a very long name for the test", () => {});
 
-test.describe
-  .parallel(`does something really long and complicated so I have to write a very long name for the test`, () => {});
+test.describe.parallel(`does something really long and complicated so I have to write a very long name for the test`, () => {});
 
-test.describe.parallel
-  .only("does something really long and complicated so I have to write a very long name for the testThis is a very", () => {});
+test.describe.parallel.only("does something really long and complicated so I have to write a very long name for the testThis is a very", () => {});
 
-test.describe.parallel
-  .only(`does something really long and complicated so I have to write a very long name for the testThis is a very`, () => {});
+test.describe.parallel.only(`does something really long and complicated so I have to write a very long name for the testThis is a very`, () => {});
 
-test.describe
-  .serial("does something really long and complicated so I have to write a very long name for the test", () => {});
+test.describe.serial("does something really long and complicated so I have to write a very long name for the test", () => {});
 
-test.describe
-  .serial(`does something really long and complicated so I have to write a very long name for the test`, () => {});
+test.describe.serial(`does something really long and complicated so I have to write a very long name for the test`, () => {});
 
-test.describe.serial
-  .only("does something really long and complicated so I have to write a very long name for the test", () => {});
+test.describe.serial.only("does something really long and complicated so I have to write a very long name for the test", () => {});
 
-test.describe.serial
-  .only(`does something really long and complicated so I have to write a very long name for the test`, () => {});
+test.describe.serial.only(`does something really long and complicated so I have to write a very long name for the test`, () => {});
 
 // Should break
 
 it.only(
   "does something really long and complicated so I have to write a very long name for the test",
   10,
   () => {
     console.log("hello!");
   },
 );
 
 it.only.only(
   "does something really long and complicated so I have to write a very long name for the test",
   () => {
     console.log("hello!");
   },
 );
 
 it.only.only(
   "does something really long and complicated so I have to write a very long name for the test",
   (a, b, c) => {
     console.log("hello!");
   },
 );
 
 xskip(
   "does something really long and complicated so I have to write a very long name for the test",
   () => {},
 );
 
-test.describe.only.parallel(
-  "does something really long and complicated so I have to write a very long name for the test",
-  () => {},
-);
+test.describe.only.parallel("does something really long and complicated so I have to write a very long name for the test", () => {});
 
 test.describe.parallel.serial(
   "does something really long and complicated so I have to write a very long name for the testThis is a very",
   () => {},
 );
 
 test.serial(
   "does something really long and complicated so I have to write a very long name for the test",
   () => {},
 );
 
 test.describe.dummy.serial(
   "does something really long and complicated so I have to write a very long name for the test",
   () => {},
 );
 
 // timeout
 
 it(`handles
   some
     newlines
   does something really long and complicated so I have to write a very long name for the test`, () => {
   console.log("hello!");
 }, 2500);
 
 it("does something quick", () => {
   console.log("hello!");
 }, 1000000000);
 
 it("succeeds if the test finishes in time", () =>
   new Promise((resolve) => setTimeout(resolve, 10)));
 
 it(
   "succeeds if the test finishes in time",
   () => new Promise((resolve) => setTimeout(resolve, 10)),
   250,
 );

```

**Prettier Similarity**: 87.69%


# js/throw_statement/binaryish.js

**Prettier Similarity**: 100.00%


# js/throw_statement/comment.js

**Prettier Similarity**: 100.00%


# js/throw_statement/jsx.js

**Prettier Similarity**: 100.00%


# js/top-level-await/example.js

**Prettier Similarity**: 100.00%


# js/top-level-await/in-expression.js

**Prettier Similarity**: 100.00%


# js/trailing-comma/dynamic-import.js

**Prettier Similarity**: 100.00%


# js/trailing-comma/es5.js

**Prettier Similarity**: 100.00%


# js/trailing-comma/function-calls.js

**Prettier Similarity**: 100.00%


# js/trailing-comma/jsx.js

**Prettier Similarity**: 100.00%


# js/trailing-comma/object.js

**Prettier Similarity**: 100.00%


# js/trailing-comma/trailing_whitespace.js

**Prettier Similarity**: 100.00%


# js/try/catch.js

**Prettier Similarity**: 100.00%


# js/try/empty.js

**Prettier Similarity**: 100.00%


# js/try/try.js

**Prettier Similarity**: 100.00%


# js/unary-expression/comments.js

**Prettier Similarity**: 100.00%


# js/unary-expression/urnary_expression.js

**Prettier Similarity**: 100.00%


# js/unary/object.js

**Prettier Similarity**: 100.00%


# js/unary/series.js

**Prettier Similarity**: 100.00%


# js/unicode/combining-characters.js

**Prettier Similarity**: 100.00%


# js/unicode/keys.js

**Prettier Similarity**: 100.00%


# js/unicode/nbsp-jsx.js

**Prettier Similarity**: 100.00%


# js/update-expression/update_expression.js

**Prettier Similarity**: 100.00%


# js/variable_declarator/multiple.js

**Prettier Similarity**: 100.00%


# js/variable_declarator/string.js

**Prettier Similarity**: 100.00%


# js/while/indent.js

**Prettier Similarity**: 100.00%


# js/with/indent.js
```diff
-with (0) {
-}
+with (0) {}
 
 with (0) 1;

```

**Prettier Similarity**: 50.00%


# js/yield/arrow.js

**Prettier Similarity**: 100.00%


# js/yield/conditional.js

**Prettier Similarity**: 100.00%


# js/yield/jsx-without-parenthesis.js

**Prettier Similarity**: 100.00%


# js/yield/jsx.js

**Prettier Similarity**: 100.00%


# jsx/attr-element/attr-element.js

**Prettier Similarity**: 100.00%


# jsx/binary-expressions/relational-operators.js

**Prettier Similarity**: 100.00%


# jsx/comments/eslint-disable.js

**Prettier Similarity**: 100.00%


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


# jsx/comments/in-tags.js

**Prettier Similarity**: 100.00%


# jsx/comments/jsx-tag-comment-after-prop.js

**Prettier Similarity**: 100.00%


# jsx/comments/like-a-comment-in-jsx-text.js

**Prettier Similarity**: 100.00%


# jsx/cursor/in-jsx-text.js
```diff
 <>
-  a<div>hi</div>
+  a
+  <div>hi</div>
 </>;

```

**Prettier Similarity**: 50.00%


# jsx/deprecated-jsx-bracket-same-line-option/jsx.js

**Prettier Similarity**: 100.00%


# jsx/escape/escape.js

**Prettier Similarity**: 100.00%


# jsx/escape/nbsp.js

**Prettier Similarity**: 100.00%


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


# jsx/fragment/fragment.js

**Prettier Similarity**: 100.00%


# jsx/ignore/jsx_ignore.js

**Prettier Similarity**: 100.00%


# jsx/jsx/array-iter.js

**Prettier Similarity**: 100.00%


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


# jsx/jsx/attr-comments.js

**Prettier Similarity**: 100.00%


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


# jsx/jsx/conditional-expression.js

**Prettier Similarity**: 100.00%


# jsx/jsx/expression.js

**Prettier Similarity**: 100.00%


# jsx/jsx/flow_fix_me.js

**Prettier Similarity**: 100.00%


# jsx/jsx/html_escape.js

**Prettier Similarity**: 100.00%


# jsx/jsx/hug.js

**Prettier Similarity**: 100.00%


# jsx/jsx/logical-expression.js

**Prettier Similarity**: 100.00%


# jsx/jsx/object-property.js

**Prettier Similarity**: 100.00%


# jsx/jsx/open-break.js

**Prettier Similarity**: 100.00%


# jsx/jsx/parens.js

**Prettier Similarity**: 100.00%


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


# jsx/jsx/regex.js

**Prettier Similarity**: 100.00%


# jsx/jsx/return-statement.js

**Prettier Similarity**: 100.00%


# jsx/jsx/self-closing.js

**Prettier Similarity**: 100.00%


# jsx/jsx/spacing.js

**Prettier Similarity**: 100.00%


# jsx/jsx/template-literal-in-attr.js

**Prettier Similarity**: 100.00%


# jsx/jsx/ternary.js

**Prettier Similarity**: 100.00%


# jsx/last-line/last_line.js

**Prettier Similarity**: 100.00%


# jsx/last-line/single_prop_multiline_string.js

**Prettier Similarity**: 100.00%


# jsx/multiline-assign/test.js

**Prettier Similarity**: 100.00%


# jsx/namespace/jsx_namespaced_name.js

**Prettier Similarity**: 100.00%


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


# jsx/newlines/windows.js

**Prettier Similarity**: 100.00%


# jsx/optional-chaining/optional-chaining.jsx

**Prettier Similarity**: 100.00%


# jsx/significant-space/comments.js

**Prettier Similarity**: 100.00%


# jsx/significant-space/test.js

**Prettier Similarity**: 100.00%


# jsx/single-attribute-per-line/single-attribute-per-line.js

**Prettier Similarity**: 100.00%


# jsx/split-attrs/test.js

**Prettier Similarity**: 100.00%


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


# jsx/stateless-arrow-fn/test.js

**Prettier Similarity**: 100.00%


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


# typescript/abstract-class/export-default.ts

**Prettier Similarity**: 100.00%


# typescript/abstract-construct-types/abstract-construct-types.ts

**Prettier Similarity**: 100.00%


# typescript/abstract-property/semicolon.ts

**Prettier Similarity**: 100.00%


# typescript/ambient/ambient.ts

**Prettier Similarity**: 100.00%


# typescript/angular-component-examples/test.component.ts

**Prettier Similarity**: 100.00%


# typescript/argument-expansion/argument_expansion.ts
```diff
-const bar1 = [1, 2, 3].reduce(
-  (carry, value) => {
-    return [...carry, value];
-  },
-  [] as unknown as number[],
-);
+const bar1 = [1, 2, 3].reduce((carry, value) => {
+  return [...carry, value];
+}, [] as unknown as number[]);
 
-const bar2 = [1, 2, 3].reduce(
-  (carry, value) => {
-    return [...carry, value];
-  },
-  <Array<number>>[],
-);
+const bar2 = [1, 2, 3].reduce((carry, value) => {
+  return [...carry, value];
+}, <Array<number>>[]);
 
 const bar3 = [1, 2, 3].reduce(
   (carry, value) => {
     return [...carry, value];
   },
   [1, 2, 3] as unknown as number[],
 );
 
 const bar4 = [1, 2, 3].reduce(
   (carry, value) => {
     return [...carry, value];
   },
   <Array<number>>[1, 2, 3],
 );
 
-const bar5 = [1, 2, 3].reduce(
-  (carry, value) => {
-    return { ...carry, [value]: true };
-  },
-  {} as unknown as { [key: number]: boolean },
-);
+const bar5 = [1, 2, 3].reduce((carry, value) => {
+  return { ...carry, [value]: true };
+}, {} as unknown as { [key: number]: boolean });
 
-const bar6 = [1, 2, 3].reduce(
-  (carry, value) => {
-    return { ...carry, [value]: true };
-  },
-  <{ [key: number]: boolean }>{},
-);
+const bar6 = [1, 2, 3].reduce((carry, value) => {
+  return { ...carry, [value]: true };
+}, <{ [key: number]: boolean }>{});
 
 const bar7 = [1, 2, 3].reduce(
   (carry, value) => {
     return { ...carry, [value]: true };
   },
   { 1: true } as unknown as { [key: number]: boolean },
 );
 
 const bar8 = [1, 2, 3].reduce(
   (carry, value) => {
     return { ...carry, [value]: true };
   },
   <{ [key: number]: boolean }>{ 1: true },
 );
 
 const bar9 = [1, 2, 3].reduce((carry, value) => {
   return [...carry, value];
 }, [] as foo);

```

**Prettier Similarity**: 59.32%


# typescript/argument-expansion/arrow-with-return-type.ts

**Prettier Similarity**: 100.00%


# typescript/array/comment.ts

**Prettier Similarity**: 100.00%


# typescript/array/key.ts

**Prettier Similarity**: 100.00%


# typescript/arrow/arrow_regression.ts

**Prettier Similarity**: 100.00%


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


# typescript/arrow/issue-6107-curry.ts

**Prettier Similarity**: 100.00%


# typescript/arrows/arrow_function_expression.ts

**Prettier Similarity**: 100.00%


# typescript/arrows/short_body.ts

**Prettier Similarity**: 100.00%


# typescript/arrows/type_params.ts
```diff
-<T,>(a) => {};
+<T>(a) => {};

```

**Prettier Similarity**: 0.00%


# typescript/as/array-pattern.ts

**Prettier Similarity**: 100.00%


# typescript/as/as-const-embedded.ts
```diff
 const GQL_QUERY_WITH_CONST = /* GraphQL */ `
-  query S {
-    shop
-  }
+  query S { shop }
 ` as const;
 
 const HTML_WITH_CONST = /* HTML */ `
-  <div>
-    <h1>foo</h1>
-    <p>foo</p>
-  </div>
+<div>
+<h1>foo</h1>
+  <p>foo</p>
+</div>
 ` as const;

```

**Prettier Similarity**: 41.67%


# typescript/as/as.ts

**Prettier Similarity**: 100.00%


# typescript/as/assignment.ts

**Prettier Similarity**: 100.00%


# typescript/as/assignment2.ts

**Prettier Similarity**: 100.00%


# typescript/as/export_default_as.ts

**Prettier Similarity**: 100.00%


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


# typescript/as/long-identifiers.ts

**Prettier Similarity**: 100.00%


# typescript/as/nested-await-and-as.ts

**Prettier Similarity**: 100.00%


# typescript/as/return.ts

**Prettier Similarity**: 100.00%


# typescript/as/ternary.ts

**Prettier Similarity**: 100.00%


# typescript/assert/comment.ts

**Prettier Similarity**: 100.00%


# typescript/assert/index.ts

**Prettier Similarity**: 100.00%


# typescript/assignment/issue-10846.ts

**Prettier Similarity**: 100.00%


# typescript/assignment/issue-10848.tsx

**Prettier Similarity**: 100.00%


# typescript/assignment/issue-10850.ts

**Prettier Similarity**: 100.00%


# typescript/assignment/issue-12413.ts

**Prettier Similarity**: 100.00%


# typescript/assignment/issue-2322.ts

**Prettier Similarity**: 100.00%


# typescript/assignment/issue-2482.ts

**Prettier Similarity**: 100.00%


# typescript/assignment/issue-2485.ts

**Prettier Similarity**: 100.00%


# typescript/assignment/issue-3122.ts

**Prettier Similarity**: 100.00%


# typescript/assignment/issue-5370.ts
```diff
-const durabilityMetricsSelectable: Immutable.OrderedSet<SomeReportingMetric> =
-  myExperienceSelectable.concat(otherDurabilityMetricsSelectable);
+const durabilityMetricsSelectable: Immutable.OrderedSet<
+  SomeReportingMetric,
+> = myExperienceSelectable.concat(otherDurabilityMetricsSelectable);

```

**Prettier Similarity**: 0.00%


# typescript/assignment/issue-6783.ts

**Prettier Similarity**: 100.00%


# typescript/assignment/issue-8619.ts

**Prettier Similarity**: 100.00%


# typescript/assignment/issue-9172.ts

**Prettier Similarity**: 100.00%


# typescript/assignment/lone-arg.ts

**Prettier Similarity**: 100.00%


# typescript/assignment/parenthesized.ts

**Prettier Similarity**: 100.00%


# typescript/bigint/bigint.ts

**Prettier Similarity**: 100.00%


# typescript/break-calls/type_args.ts

**Prettier Similarity**: 100.00%


# typescript/call-signature/call-signature.ts

**Prettier Similarity**: 100.00%


# typescript/cast/as-const.ts

**Prettier Similarity**: 100.00%


# typescript/cast/assert-and-assign.ts

**Prettier Similarity**: 100.00%


# typescript/cast/generic-cast.ts

**Prettier Similarity**: 100.00%


# typescript/cast/hug-args.ts

**Prettier Similarity**: 100.00%


# typescript/cast/parenthesis.ts

**Prettier Similarity**: 100.00%


# typescript/catch-clause/type-annotation.ts

**Prettier Similarity**: 100.00%


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


# typescript/class-comment/class-implements.ts

**Prettier Similarity**: 100.00%


# typescript/class-comment/declare.ts

**Prettier Similarity**: 100.00%


# typescript/class-comment/generic.ts

**Prettier Similarity**: 100.00%


# typescript/class-comment/misc.ts

**Prettier Similarity**: 100.00%


# typescript/class/abstract-method.ts

**Prettier Similarity**: 100.00%


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


# typescript/class/declare-readonly-field-initializer-w-annotation.ts

**Prettier Similarity**: 100.00%


# typescript/class/declare-readonly-field-initializer.ts

**Prettier Similarity**: 100.00%


# typescript/class/dunder.ts

**Prettier Similarity**: 100.00%


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


# typescript/class/extends_implements.ts

**Prettier Similarity**: 100.00%


# typescript/class/generics.ts

**Prettier Similarity**: 100.00%


# typescript/class/methods.ts

**Prettier Similarity**: 100.00%


# typescript/class/optional.ts

**Prettier Similarity**: 100.00%


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


# typescript/class/standard_private_fields.ts

**Prettier Similarity**: 100.00%


# typescript/classes/break-heritage.ts

**Prettier Similarity**: 100.00%


# typescript/classes/break.ts

**Prettier Similarity**: 100.00%


# typescript/comments-2/dangling.ts

**Prettier Similarity**: 100.00%


# typescript/comments-2/issues.ts

**Prettier Similarity**: 100.00%


# typescript/comments-2/last-arg.ts

**Prettier Similarity**: 100.00%


# typescript/comments/abstract_class.ts

**Prettier Similarity**: 100.00%


# typescript/comments/abstract_methods.ts

**Prettier Similarity**: 100.00%


# typescript/comments/after_jsx_generic.tsx

**Prettier Similarity**: 100.00%


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


# typescript/comments/interface.ts

**Prettier Similarity**: 100.00%


# typescript/comments/issues.ts

**Prettier Similarity**: 100.00%


# typescript/comments/jsx.tsx

**Prettier Similarity**: 100.00%


# typescript/comments/location.ts

**Prettier Similarity**: 100.00%


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


# typescript/comments/methods.ts

**Prettier Similarity**: 100.00%


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


# typescript/comments/type_literals.ts

**Prettier Similarity**: 100.00%


# typescript/comments/types.ts

**Prettier Similarity**: 100.00%


# typescript/comments/union.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/ClassDeclaration22.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/anyIsAssignableToObject.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/castOfAwait.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/castParentheses.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/castTest.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/checkInfiniteExpansionTermination.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/commentInNamespaceDeclarationWithIdentifierPathName.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/commentsInterface.ts

**Prettier Similarity**: 100.00%


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


# typescript/compiler/declareDottedModuleName.ts

**Prettier Similarity**: 100.00%


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


# typescript/compiler/es5ExportDefaultClassDeclaration4.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/functionOverloadsOnGenericArity1.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/globalIsContextualKeyword.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/indexSignatureWithInitializer.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/mappedTypeWithCombinedTypeMappers.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/modifiersOnInterfaceIndexSignature1.ts

**Prettier Similarity**: 100.00%


# typescript/compiler/privacyGloImport.ts

**Prettier Similarity**: 100.00%


# typescript/conditional-types/comments.ts

**Prettier Similarity**: 100.00%


# typescript/conditional-types/conditonal-types.ts

**Prettier Similarity**: 100.00%


# typescript/conditional-types/infer-type.ts

**Prettier Similarity**: 100.00%


# typescript/conditional-types/nested-in-condition.ts

**Prettier Similarity**: 100.00%


# typescript/conditional-types/new-ternary-spec.ts

**Prettier Similarity**: 100.00%


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


# typescript/conformance/ambient/ambientDeclarations.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/abstract.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractAccessor.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractAsIdentifier.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractAssignabilityConstructorFunction.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractClinterfaceAssignability.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractConstructorAssignability.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractCrashedOnce.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractExtends.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractFactoryFunction.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractGeneric.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractImportInstantiation.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractInAModule.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractInheritance.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractInstantiations1.ts

**Prettier Similarity**: 100.00%


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


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractMethodInNonAbstractClass.ts

**Prettier Similarity**: 100.00%


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


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractOverloads.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractOverrideWithAbstract.ts

**Prettier Similarity**: 100.00%


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


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractSingleLineDecl.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractSuperCalls.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractUsingAbstractMethod1.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractUsingAbstractMethods2.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractWithInterface.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classHeritageSpecification/classAppearsToHaveMembersOfObject.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classHeritageSpecification/classExtendingClass.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classHeritageSpecification/classExtendsItselfIndirectly.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classHeritageSpecification/classIsSubtypeOfBaseType.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classDeclarations/classInsideBlock.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/classExpression.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorDefaultValuesReferencingThis.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorImplementationWithDefaultValues.ts

**Prettier Similarity**: 100.00%


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


# typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorOverloadsWithDefaultValues.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorOverloadsWithOptionalParameters.ts

**Prettier Similarity**: 100.00%


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


# typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorParameterProperties2.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/constructorDeclarations/constructorParameters/declarationEmitReadonly.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/constructorDeclarations/constructorParameters/readonlyConstructorAssignment.ts

**Prettier Similarity**: 100.00%


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


# typescript/conformance/classes/mixinAccessModifiers.ts

**Prettier Similarity**: 100.00%


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


# typescript/conformance/classes/mixinClassesMembers.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/classes/nestedClassDeclaration.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/comments/comments.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/declarationEmit/typePredicates/declarationEmitThisPredicatesWithPrivateName01.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/es6/Symbols/symbolProperty15.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/es6/templates/templateStringWithEmbeddedTypeAssertionOnAdditionES6.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/expressions/asOperator/asOperatorContextualType.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/expressions/functionCalls/callWithSpreadES6.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/interfaces/interfaceDeclarations/interfaceWithMultipleBaseTypes2.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/internalModules/importDeclarations/circularImportAlias.ts

**Prettier Similarity**: 100.00%


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


# typescript/conformance/internalModules/importDeclarations/exportInterface.ts

**Prettier Similarity**: 100.00%


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


# typescript/conformance/internalModules/importDeclarations/invalidImportAliasIdentifiers.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/internalModules/importDeclarations/shadowedInternalModule.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/parser/ecmascript5/Statements/parserES5ForOfStatement2.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/parser/ecmascript5/Statements/parserES5ForOfStatement21.ts
```diff
 //@target: ES5
-for (var of of) {
-}
+for (var of of) { }

```

**Prettier Similarity**: 33.33%


# typescript/conformance/parser/ecmascript5/Statements/parserForInStatement2.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/ambient/ambientDeclarations.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/any/anyAsConstructor.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/any/anyAsFunctionCall.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/any/anyAsGenericFunctionCall.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/any/anyPropertyAccess.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/constKeyword/constKeyword.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/constructorType/cunstructorType.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/enumDeclaration/enumDeclaration.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/firstTypeNode/firstTypeNode.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/functions/TSFunctionTypeNoUnnecessaryParentheses.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/functions/functionImplementationErrors.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/functions/functionImplementations.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/functions/functionOverloadCompatibilityWithVoid01.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/functions/functionOverloadCompatibilityWithVoid02.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/functions/functionOverloadCompatibilityWithVoid03.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/functions/functionOverloadErrorsSyntax.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/functions/functionTypeTypeParameters.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/functions/parameterInitializersForwardReferencing.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/importEqualsDeclaration/importEqualsDeclaration.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/indexedAccesType/indexedAccesType.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/interfaceDeclaration/interfaceDeclaration.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/intersectionType/intersectionType.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/lastTypeNode/lastTypeNode.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/mappedType/mappedType.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/methodSignature/methodSignature.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/moduleDeclaration/kind-detection.ts
```diff
-declare namespace /* module */ A {}
+declare /* module */ namespace A {}

```

**Prettier Similarity**: 0.00%


# typescript/conformance/types/moduleDeclaration/moduleDeclaration.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/namespaceExportDeclaration/exportAsNamespace.d.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/never/never.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/nonNullExpression/nonNullExpression.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/parameterProperty/parameterProperty.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/symbol/symbol.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/thisType/thisType.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/contextualTypeWithTuple.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/emptyTuples/emptyTuplesTypeAssertion02.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/indexerWithTuple.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/tupleElementTypes1.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/tupleElementTypes2.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/tupleElementTypes3.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/tupleElementTypes4.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/typeInferenceWithTupleType.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/wideningTuples1.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/wideningTuples2.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/wideningTuples3.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/wideningTuples4.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/wideningTuples5.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/wideningTuples6.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/tuple/wideningTuples7.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/typeOperator/typeOperator.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/typeParameter/typeParameter.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/typeParameters/typeParameterLists/innerTypeParameterShadowingOuterOne.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/typeParameters/typeParameterLists/innerTypeParameterShadowingOuterOne2.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/typeParameters/typeParameterLists/staticMembersUsingClassTypeParameter.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/typeParameters/typeParameterLists/typeParametersAvailableInNestedScope2.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/typeReference/typeReference.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/undefined/undefined.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/union/unionTypeCallSignatures.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/union/unionTypeCallSignatures3.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/union/unionTypeCallSignatures4.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/union/unionTypeConstructSignatures.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/union/unionTypeEquivalence.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/union/unionTypeFromArrayLiteral.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/union/unionTypeIndexSignature.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/union/unionTypePropertyAccessibility.ts

**Prettier Similarity**: 100.00%


# typescript/conformance/types/variableDeclarator/variableDeclarator.ts

**Prettier Similarity**: 100.00%


# typescript/const/initializer-ambient-context.ts

**Prettier Similarity**: 100.00%


# typescript/cursor/array-pattern.ts

**Prettier Similarity**: 100.00%


# typescript/cursor/arrow-function-type.ts

**Prettier Similarity**: 100.00%


# typescript/cursor/class-property.ts

**Prettier Similarity**: 100.00%


# typescript/cursor/function-return-type.ts

**Prettier Similarity**: 100.00%


# typescript/cursor/identifier-1.ts

**Prettier Similarity**: 100.00%


# typescript/cursor/identifier-2.ts

**Prettier Similarity**: 100.00%


# typescript/cursor/identifier-3.ts

**Prettier Similarity**: 100.00%


# typescript/cursor/method-signature.ts

**Prettier Similarity**: 100.00%


# typescript/cursor/property-signature.ts

**Prettier Similarity**: 100.00%


# typescript/cursor/rest.ts

**Prettier Similarity**: 100.00%


# typescript/custom/abstract/abstractNewlineHandling.ts

**Prettier Similarity**: 100.00%


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


# typescript/custom/call/callSignature.ts

**Prettier Similarity**: 100.00%


# typescript/custom/computedProperties/string.ts

**Prettier Similarity**: 100.00%


# typescript/custom/computedProperties/symbol.ts

**Prettier Similarity**: 100.00%


# typescript/custom/declare/declareModifier.d.ts

**Prettier Similarity**: 100.00%


# typescript/custom/modifiers/minustoken.ts

**Prettier Similarity**: 100.00%


# typescript/custom/modifiers/question.ts

**Prettier Similarity**: 100.00%


# typescript/custom/modifiers/readonly.ts

**Prettier Similarity**: 100.00%


# typescript/custom/module/global.ts

**Prettier Similarity**: 100.00%


# typescript/custom/module/moduleNamespace.ts

**Prettier Similarity**: 100.00%


# typescript/custom/module/nestedNamespace.ts

**Prettier Similarity**: 100.00%


# typescript/custom/new/newKeyword.ts

**Prettier Similarity**: 100.00%


# typescript/custom/stability/moduleBlock.ts

**Prettier Similarity**: 100.00%


# typescript/custom/typeParameters/callAndConstructSignatureLong.ts

**Prettier Similarity**: 100.00%


# typescript/custom/typeParameters/functionTypeLong.ts

**Prettier Similarity**: 100.00%


# typescript/custom/typeParameters/interfaceParamsLong.ts

**Prettier Similarity**: 100.00%


# typescript/custom/typeParameters/typeParametersLong.ts

**Prettier Similarity**: 100.00%


# typescript/custom/typeParameters/variables.ts

**Prettier Similarity**: 100.00%


# typescript/declare/declare-get-set-field.ts

**Prettier Similarity**: 100.00%


# typescript/declare/declare_class_fields.ts

**Prettier Similarity**: 100.00%


# typescript/declare/declare_enum.ts

**Prettier Similarity**: 100.00%


# typescript/declare/declare_function.ts

**Prettier Similarity**: 100.00%


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


# typescript/declare/declare_interface.ts

**Prettier Similarity**: 100.00%


# typescript/declare/declare_module.ts

**Prettier Similarity**: 100.00%


# typescript/declare/declare_namespace.ts

**Prettier Similarity**: 100.00%


# typescript/declare/declare_var.ts

**Prettier Similarity**: 100.00%


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


# typescript/decorator-auto-accessors/decorator-auto-accessors-new-line.ts

**Prettier Similarity**: 100.00%


# typescript/decorator-auto-accessors/decorator-auto-accessors-type-annotations.ts

**Prettier Similarity**: 100.00%


# typescript/decorator-auto-accessors/no-semi/decorator-auto-accessor-like-property-name.ts

**Prettier Similarity**: 100.00%


# typescript/decorators-ts/accessor-decorator.ts

**Prettier Similarity**: 100.00%


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


# typescript/decorators-ts/class-decorator.ts

**Prettier Similarity**: 100.00%


# typescript/decorators-ts/method-decorator.ts

**Prettier Similarity**: 100.00%


# typescript/decorators-ts/mobx.ts

**Prettier Similarity**: 100.00%


# typescript/decorators-ts/multiple.ts

**Prettier Similarity**: 100.00%


# typescript/decorators-ts/parameter-decorator.ts

**Prettier Similarity**: 100.00%


# typescript/decorators-ts/property-decorator.ts

**Prettier Similarity**: 100.00%


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


# typescript/decorators/accessor.ts

**Prettier Similarity**: 100.00%


# typescript/decorators/argument-list-preserve-line.ts

**Prettier Similarity**: 100.00%


# typescript/decorators/comments.ts

**Prettier Similarity**: 100.00%


# typescript/decorators/decorator-type-assertion.ts

**Prettier Similarity**: 100.00%


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


# typescript/decorators/decorators.ts

**Prettier Similarity**: 100.00%


# typescript/decorators/inline-decorators.ts

**Prettier Similarity**: 100.00%


# typescript/decorators/legacy.ts

**Prettier Similarity**: 100.00%


# typescript/decorators/mobx.ts

**Prettier Similarity**: 100.00%


# typescript/definite/asi.ts

**Prettier Similarity**: 100.00%


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


# typescript/destructuring/destructuring.ts

**Prettier Similarity**: 100.00%


# typescript/end-of-line/multiline.ts

**Prettier Similarity**: 100.00%


# typescript/enum/computed-members.ts

**Prettier Similarity**: 100.00%


# typescript/enum/enum.ts

**Prettier Similarity**: 100.00%


# typescript/enum/multiline.ts

**Prettier Similarity**: 100.00%


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


# typescript/explicit-resource-management/await-using-with-type-declaration.ts

**Prettier Similarity**: 100.00%


# typescript/explicit-resource-management/using-with-type-declaration.ts

**Prettier Similarity**: 100.00%


# typescript/export-default/function_as.ts

**Prettier Similarity**: 100.00%


# typescript/export/comment.ts

**Prettier Similarity**: 100.00%


# typescript/export/default.ts

**Prettier Similarity**: 100.00%


# typescript/export/export-as-ns.ts

**Prettier Similarity**: 100.00%


# typescript/export/export-class.ts

**Prettier Similarity**: 100.00%


# typescript/export/export-type-star-from.ts

**Prettier Similarity**: 100.00%


# typescript/export/export.ts

**Prettier Similarity**: 100.00%


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


# typescript/function-type/single-parameter.ts

**Prettier Similarity**: 100.00%


# typescript/function-type/type-annotation.ts

**Prettier Similarity**: 100.00%


# typescript/function/single_expand.ts

**Prettier Similarity**: 100.00%


# typescript/functional-composition/pipe-function-calls-with-comments.ts

**Prettier Similarity**: 100.00%


# typescript/functional-composition/pipe-function-calls.ts

**Prettier Similarity**: 100.00%


# typescript/generic/arrow-return-type.ts

**Prettier Similarity**: 100.00%


# typescript/generic/issue-6899.ts

**Prettier Similarity**: 100.00%


# typescript/generic/object-method.ts

**Prettier Similarity**: 100.00%


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


# typescript/import-require/import_require.ts

**Prettier Similarity**: 100.00%


# typescript/import-require/type-imports.ts

**Prettier Similarity**: 100.00%


# typescript/import-type/import-type.ts

**Prettier Similarity**: 100.00%


# typescript/index-signature/index-signature.ts

**Prettier Similarity**: 100.00%


# typescript/index-signature/static.ts

**Prettier Similarity**: 100.00%


# typescript/infer-extends/basic.ts

**Prettier Similarity**: 100.00%


# typescript/instantiation-expression/basic.ts

**Prettier Similarity**: 100.00%


# typescript/instantiation-expression/binary-expr.ts

**Prettier Similarity**: 100.00%


# typescript/instantiation-expression/inferface-asi.ts

**Prettier Similarity**: 100.00%


# typescript/instantiation-expression/logical-expr.ts

**Prettier Similarity**: 100.00%


# typescript/instantiation-expression/new.ts

**Prettier Similarity**: 100.00%


# typescript/instantiation-expression/property-access.ts

**Prettier Similarity**: 100.00%


# typescript/instantiation-expression/typeof.ts

**Prettier Similarity**: 100.00%


# typescript/interface/comments-generic.ts

**Prettier Similarity**: 100.00%


# typescript/interface/comments.ts

**Prettier Similarity**: 100.00%


# typescript/interface/generic.ts

**Prettier Similarity**: 100.00%


# typescript/interface/ignore.ts

**Prettier Similarity**: 100.00%


# typescript/interface/long-extends.ts

**Prettier Similarity**: 100.00%


# typescript/interface/long-type-parameters/long-type-parameters.ts

**Prettier Similarity**: 100.00%


# typescript/interface/pattern-parameters.ts

**Prettier Similarity**: 100.00%


# typescript/interface/separator.ts

**Prettier Similarity**: 100.00%


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


# typescript/interface2/comments-declare.ts

**Prettier Similarity**: 100.00%


# typescript/interface2/comments.ts

**Prettier Similarity**: 100.00%


# typescript/interface2/module.ts

**Prettier Similarity**: 100.00%


# typescript/intersection/consistent-with-flow/comment.ts

**Prettier Similarity**: 100.00%


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


# typescript/intersection/type-arguments.ts

**Prettier Similarity**: 100.00%


# typescript/intrinsic/intrinsic.ts

**Prettier Similarity**: 100.00%


# typescript/key-remapping-in-mapped-types/key-remapping.ts

**Prettier Similarity**: 100.00%


# typescript/keyof/keyof.ts

**Prettier Similarity**: 100.00%


# typescript/keyword-types/conditional-types.ts

**Prettier Similarity**: 100.00%


# typescript/keyword-types/keyword-types-with-parens-comments.ts

**Prettier Similarity**: 100.00%


# typescript/keywords/keywords-2.ts

**Prettier Similarity**: 100.00%


# typescript/keywords/keywords.ts

**Prettier Similarity**: 100.00%


# typescript/keywords/module.ts

**Prettier Similarity**: 100.00%


# typescript/last-argument-expansion/break.ts

**Prettier Similarity**: 100.00%


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


# typescript/last-argument-expansion/edge_case.ts

**Prettier Similarity**: 100.00%


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


# typescript/literal/multiline.ts

**Prettier Similarity**: 100.00%


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


# typescript/mapped-type/intersection.ts

**Prettier Similarity**: 100.00%


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


# typescript/mapped-type/mapped-type.ts

**Prettier Similarity**: 100.00%


# typescript/method-chain/comment.ts

**Prettier Similarity**: 100.00%


# typescript/method/issue-10352-consistency.ts

**Prettier Similarity**: 100.00%


# typescript/method/method-signature-with-wrapped-return-type.ts

**Prettier Similarity**: 100.00%


# typescript/method/method-signature.ts

**Prettier Similarity**: 100.00%


# typescript/method/semi.ts

**Prettier Similarity**: 100.00%


# typescript/method/type_literal_optional_method.ts

**Prettier Similarity**: 100.00%


# typescript/module/empty.ts

**Prettier Similarity**: 100.00%


# typescript/module/global.ts

**Prettier Similarity**: 100.00%


# typescript/module/keyword.ts

**Prettier Similarity**: 100.00%


# typescript/module/module_nested.ts

**Prettier Similarity**: 100.00%


# typescript/module/namespace_function.ts

**Prettier Similarity**: 100.00%


# typescript/module/namespace_nested.ts

**Prettier Similarity**: 100.00%


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


# typescript/namespace/invalid-await.ts

**Prettier Similarity**: 100.00%


# typescript/never/type-argument.src.ts

**Prettier Similarity**: 100.00%


# typescript/new/new-signature.ts

**Prettier Similarity**: 100.00%


# typescript/no-semi/no-semi.ts

**Prettier Similarity**: 100.00%


# typescript/no-semi/non-null.ts

**Prettier Similarity**: 100.00%


# typescript/non-null/braces.ts

**Prettier Similarity**: 100.00%


# typescript/non-null/member-chain.ts

**Prettier Similarity**: 100.00%


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


# typescript/non-null/parens.ts

**Prettier Similarity**: 100.00%


# typescript/nosemi/index-signature.ts

**Prettier Similarity**: 100.00%


# typescript/nosemi/interface.ts

**Prettier Similarity**: 100.00%


# typescript/nosemi/type.ts

**Prettier Similarity**: 100.00%


# typescript/optional-call/type-parameters.ts

**Prettier Similarity**: 100.00%


# typescript/optional-method/optional-method.ts

**Prettier Similarity**: 100.00%


# typescript/optional-type/complex.ts

**Prettier Similarity**: 100.00%


# typescript/optional-type/simple.ts

**Prettier Similarity**: 100.00%


# typescript/optional-variance/basic.ts

**Prettier Similarity**: 100.00%


# typescript/optional-variance/with-jsx.tsx

**Prettier Similarity**: 100.00%


# typescript/override-modifiers/override-modifier.ts

**Prettier Similarity**: 100.00%


# typescript/override-modifiers/parameter-property.ts

**Prettier Similarity**: 100.00%


# typescript/predicate-types/predicate-types.ts

**Prettier Similarity**: 100.00%


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


# typescript/private-fields-in-in/basic.ts

**Prettier Similarity**: 100.00%


# typescript/quote-props/types.ts

**Prettier Similarity**: 100.00%


# typescript/range/export-assignment.ts
```diff
-f();
+f ( );
 export = f;
-g();
+g(  )

```

**Prettier Similarity**: 33.33%


# typescript/range/issue-4926.ts

**Prettier Similarity**: 100.00%


# typescript/range/issue-7148.ts

**Prettier Similarity**: 100.00%


# typescript/readonly/array.ts

**Prettier Similarity**: 100.00%


# typescript/readonly/readonly.ts

**Prettier Similarity**: 100.00%


# typescript/rest-type/complex.ts

**Prettier Similarity**: 100.00%


# typescript/rest-type/infer-type.ts

**Prettier Similarity**: 100.00%


# typescript/rest-type/simple.ts

**Prettier Similarity**: 100.00%


# typescript/rest/rest.ts

**Prettier Similarity**: 100.00%


# typescript/satisfies-operators/argument-expansion.ts
```diff
-const bar1 = [1, 2, 3].reduce(
-  (carry, value) => {
-    return [...carry, value];
-  },
-  [] satisfies unknown satisfies number[],
-);
+const bar1 = [1, 2, 3].reduce((carry, value) => {
+  return [...carry, value];
+}, [] satisfies unknown satisfies number[]);
 
 const bar2 = [1, 2, 3].reduce(
   (carry, value) => {
     return [...carry, value];
   },
   [1, 2, 3] satisfies unknown satisfies number[],
 );
 
-const bar3 = [1, 2, 3].reduce(
-  (carry, value) => {
-    return { ...carry, [value]: true };
-  },
-  {} satisfies unknown satisfies { [key: number]: boolean },
-);
+const bar3 = [1, 2, 3].reduce((carry, value) => {
+  return { ...carry, [value]: true };
+}, {} satisfies unknown satisfies { [key: number]: boolean });
 
 const bar4 = [1, 2, 3].reduce(
   (carry, value) => {
     return { ...carry, [value]: true };
   },
   { 1: true } satisfies unknown satisfies { [key: number]: boolean },
 );
 
 const bar5 = [1, 2, 3].reduce((carry, value) => {
   return [...carry, value];
 }, [] satisfies foo);

```

**Prettier Similarity**: 61.29%


# typescript/satisfies-operators/assignment.ts

**Prettier Similarity**: 100.00%


# typescript/satisfies-operators/basic.ts

**Prettier Similarity**: 100.00%


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


# typescript/satisfies-operators/comments.ts

**Prettier Similarity**: 100.00%


# typescript/satisfies-operators/export-default-as.ts

**Prettier Similarity**: 100.00%


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


# typescript/satisfies-operators/gt-lt.ts

**Prettier Similarity**: 100.00%


# typescript/satisfies-operators/hug-args.ts

**Prettier Similarity**: 100.00%


# typescript/satisfies-operators/lhs.ts

**Prettier Similarity**: 100.00%


# typescript/satisfies-operators/nested-await-and-satisfies.ts

**Prettier Similarity**: 100.00%


# typescript/satisfies-operators/non-null.ts

**Prettier Similarity**: 100.00%


# typescript/satisfies-operators/satisfies.ts

**Prettier Similarity**: 100.00%


# typescript/satisfies-operators/template-literal.ts

**Prettier Similarity**: 100.00%


# typescript/satisfies-operators/ternary.ts

**Prettier Similarity**: 100.00%


# typescript/satisfies-operators/types-comments.ts

**Prettier Similarity**: 100.00%


# typescript/semi/no-semi.ts

**Prettier Similarity**: 100.00%


# typescript/static-blocks/multiple.ts

**Prettier Similarity**: 100.00%


# typescript/static-blocks/nested.ts

**Prettier Similarity**: 100.00%


# typescript/static-blocks/static-blocks.ts

**Prettier Similarity**: 100.00%


# typescript/symbol/symbol.ts

**Prettier Similarity**: 100.00%


# typescript/template-literal-types/template-literal-types.ts

**Prettier Similarity**: 100.00%


# typescript/template-literals/as-expression.ts

**Prettier Similarity**: 100.00%


# typescript/template-literals/expressions.ts

**Prettier Similarity**: 100.00%


# typescript/ternaries/indent.ts

**Prettier Similarity**: 100.00%


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


# typescript/trailing-comma/arrow-functions.tsx

**Prettier Similarity**: 100.00%


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


# typescript/trailing-comma/type-arguments.ts

**Prettier Similarity**: 100.00%


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


# typescript/tsx/generic-component.tsx

**Prettier Similarity**: 100.00%


# typescript/tsx/keyword.tsx

**Prettier Similarity**: 100.00%


# typescript/tsx/member-expression.tsx

**Prettier Similarity**: 100.00%


# typescript/tsx/not-react.ts

**Prettier Similarity**: 100.00%


# typescript/tsx/react.tsx

**Prettier Similarity**: 100.00%


# typescript/tsx/this.tsx

**Prettier Similarity**: 100.00%


# typescript/tsx/type-parameters.tsx

**Prettier Similarity**: 100.00%


# typescript/tsx/url.tsx

**Prettier Similarity**: 100.00%


# typescript/tuple/dangling-comments.ts
```diff
-type Foo1 = [
-  /* comment */
-];
+type Foo1 = [/* comment */];
 
 type Foo2 = [
   // comment
 ];
 
 type Foo3 = [
   // comment1
   // comment2
 ];
 
 type Foo4 = [
   // comment1
   // comment2
 ];
 
-type Foo5 = [
-  /* comment1 */
-];
+type Foo5 = [/* comment1 */];
 
 type Foo6 = [
   /* comment1 */
   /* comment2 */
 ];

```

**Prettier Similarity**: 76.92%


# typescript/tuple/trailing-comma-for-empty-tuples.ts

**Prettier Similarity**: 100.00%


# typescript/tuple/trailing-comma-trailing-rest.ts

**Prettier Similarity**: 100.00%


# typescript/tuple/trailing-comma.ts

**Prettier Similarity**: 100.00%


# typescript/tuple/tuple-labeled.ts

**Prettier Similarity**: 100.00%


# typescript/tuple/tuple-rest-not-last.ts

**Prettier Similarity**: 100.00%


# typescript/tuple/tuple.ts

**Prettier Similarity**: 100.00%


# typescript/type-alias/issue-100857.ts

**Prettier Similarity**: 100.00%


# typescript/type-alias/issue-9874.ts

**Prettier Similarity**: 100.00%


# typescript/type-alias/pattern-parameter.ts

**Prettier Similarity**: 100.00%


# typescript/type-arguments-bit-shift-left-like/1.ts
```diff
-f << (<T>x);
+f << <T>x;

```

**Prettier Similarity**: 0.00%


# typescript/type-arguments-bit-shift-left-like/2.ts

**Prettier Similarity**: 100.00%


# typescript/type-arguments-bit-shift-left-like/4.ts

**Prettier Similarity**: 100.00%


# typescript/type-arguments-bit-shift-left-like/6.ts

**Prettier Similarity**: 100.00%


# typescript/type-member-get-set/type-member-get-set.ts

**Prettier Similarity**: 100.00%


# typescript/type-only-module-specifiers/basic.ts

**Prettier Similarity**: 100.00%


# typescript/typeof-this/decorators.ts

**Prettier Similarity**: 100.00%


# typescript/typeof-this/typeof-this.ts

**Prettier Similarity**: 100.00%


# typescript/typeof/typeof.ts

**Prettier Similarity**: 100.00%


# typescript/typeparams/class-method.ts

**Prettier Similarity**: 100.00%


# typescript/typeparams/consistent/flow-only.ts

**Prettier Similarity**: 100.00%


# typescript/typeparams/consistent/issue-9501.ts

**Prettier Similarity**: 100.00%


# typescript/typeparams/consistent/simple-types.ts

**Prettier Similarity**: 100.00%


# typescript/typeparams/consistent/template-literal-types.ts

**Prettier Similarity**: 100.00%


# typescript/typeparams/consistent/typescript-only.ts

**Prettier Similarity**: 100.00%


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


# typescript/typeparams/long-function-arg.ts

**Prettier Similarity**: 100.00%


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


# typescript/typeparams/tagged-template-expression.ts

**Prettier Similarity**: 100.00%


# typescript/typeparams/trailing-comma/type-paramters.ts

**Prettier Similarity**: 100.00%


# typescript/union/comments.ts

**Prettier Similarity**: 100.00%


# typescript/union/consistent-with-flow/comment.ts

**Prettier Similarity**: 100.00%


# typescript/union/consistent-with-flow/comments.ts

**Prettier Similarity**: 100.00%


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


# typescript/union/consistent-with-flow/within-tuple.ts

**Prettier Similarity**: 100.00%


# typescript/union/inlining.ts
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
-  articles:
-    | Array<{
-        __id: string;
-      } | null>
-    | null // articles type may be null
-    | void; // articles type may be void
+  articles: Array<{
+    __id: string;
+  } | null> | null | void; // articles type may be null // articles type may be void
 }
 
-type FooBar =
-  | null // null
-  | {
-      /** x **/ y: number;
-      z: string;
-    } // this documents the first option
-  | void; // this documents the second option
+// FIXME
+// TODO: reformat issue
+// type FooBar = null // null
+// | { /** x **/
+//   y: number;
+//   z: string;
+// } // this documents the first option
+//   | void // this documents the second option
+//   ;
 
 type FooBarWithoutComment = null | {
   y: number;
   z: string;
 } | void;
 
-type FooBar2 =
-  | Number // this documents the first option
-  | void; // this documents the second option
+type FooBar2 = Number | void; // this documents the first option // this documents the second option
 
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

**Prettier Similarity**: 76.12%


# typescript/union/single-type/single-type.ts
```diff
-type A1 /* 2 */ = /* 1 */ /* 3 */ /* 4 */ {
-  key: string;
-};
+// FIXME
+// TODO: reformat issue
+// type A1 =
+//   /* 1 */ | /* 2 */ (
+//     /* 3 */ | /* 4 */ {
+//         key: string;
+//       }
+//   );

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


# typescript/union/with-type-params.ts

**Prettier Similarity**: 100.00%


# typescript/unique-symbol/unique-symbol.ts

**Prettier Similarity**: 100.00%


# typescript/unknown/unknown.ts

**Prettier Similarity**: 100.00%


# typescript/webhost/webtsc.ts

**Prettier Similarity**: 100.00%


