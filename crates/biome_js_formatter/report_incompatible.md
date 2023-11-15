# Overall Metrics

**Average compatibility**: 91.65

    <details>
    	<summary>Definition</summary>

    	$$average = \frac\{\sum_{file}^\{files}compatibility_\{file}}\{files}$$
    </details>

    **Compatible lines**: 93.08
    <details>
        <summary>Definition</summary>

        $$average = \frac{\sum_{file}^{files}matching\_lines_{file}}{max(lines_{rome}, lines_{prettier})}$$
    </details>

    [Metric definition discussion](https://github.com/rome/tools/issues/2555#issuecomment-1124787893)
                

# js/arrays/holes-in-args.js
```diff
-new Test().test().test([, 0]).test();
+new Test()
+  .test()
+  .test([, 0])
+  .test();

```

**Prettier Similarity**: 0.00%


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


# js/arrows/arrow_function_expression.js
```diff
 ((a) => {}).length;
 typeof (() => {});
 export default (() => {})();
 (() => {})()``;
 (() => {})``;
 new (() => {})();
 if ((() => {}) ? 1 : 0) {
 }
 let f = () => ({})();
 let a = () => ({}) instanceof a;
 a = () => ({}) && a;
 a = () => ({})() && a;
 a = () => ({}) && a && b;
 a = () => ({}) + a;
 a = () => ({})()() && a;
 a = () => ({}).b && a;
 a = () => ({})[b] && a;
 a = () => ({})`` && a;
 a = () => ({} = 0);
-a = () => ({}, a);
+a = () => (({}), a);
 (a) => a instanceof {};
 (a) => ({})().b && 0;
-(a) => ({}().c = 0);
+(a) => (({})().c = 0);
 (x) => ({})()();
 (x) => ({})()``;
 (x) => ({})().b;
 a = (b) => c;
 (x) => (y = z);
 (x) => (y += z);
 f((a) => ({})) + 1;
 ((a) => ({})) || 0;
 a = (b) => c;
 a = (b) => {
   return c;
 };

```

**Prettier Similarity**: 94.29%


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


# js/babel-plugins/deferred-import-evaluation.js
```diff
-import defer * as ns from "x";
+import defer
+* as ns from "x"

```

**Prettier Similarity**: 0.00%


# js/babel-plugins/import-reflection.js
```diff
-import module foo from "./module.wasm";
+import module
+foo;
+from;
+("./module.wasm");

```

**Prettier Similarity**: 0.00%


# js/babel-plugins/source-phase-imports.js
```diff
-import source fooSource from "foo";
+import source
+fooSource;
+from;
+("foo");
 import.source("x");

```

**Prettier Similarity**: 20.00%


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


# js/comments/trailing_space.js
```diff
-#!/there/is-space-here->
+#!/there/is-space-here->         
 
 // Do not trim trailing whitespace from this source file!
 
 // There is some space here ->

```

**Prettier Similarity**: 80.00%


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


# js/decorators/class-expression/arguments.js
```diff
 console.log(
-  @deco
-  class Foo {},
+  (
+    @deco
+    class Foo {}
+  ),
 );
 console.log(
-  @deco
-  class {},
+  (
+    @deco
+    class {}
+  ),
 );

```

**Prettier Similarity**: 33.33%


# js/decorators/class-expression/class-expression.js
```diff
 const a1 =
-  @deco
-  class Foo {};
+  (
+    @deco
+    class Foo {}
+  );
 const a2 =
-  @deco
-  class {};
+  (
+    @deco
+    class {}
+  );
 
 (
   @deco
   class Foo {}
 );
 (
   @deco
   class {}
 );
 
 const b1 = [];
 (
   @deco
   class Foo {}
 );
 
 const b2 = [];
 (
   @deco
   class {}
 );
 
 // This is not a `ClassExpression` but `ClassDeclaration`
 @deco
 class Foo {}

```

**Prettier Similarity**: 77.14%


# js/decorators/classes.js
```diff
 @deco
 class Foo {}
 
 @deco
 export class Bar {}
 
 @deco
 export default class Baz {}
 
 const foo =
-  @deco
-  class {
-    //
-  };
+  (
+    @deco
+    class {
+      //
+    }
+  );
 
 const bar =
-  @deco
-  class {
-    //
-  };
+  (
+    @deco
+    class {
+      //
+    }
+  );

```

**Prettier Similarity**: 50.00%


# js/decorators/member-expression.js
```diff
 [
   class {
     @(decorators[0])
     method() {}
   },
   class {
     @decorators [0];
     method() {}
   },
   class {
     @(decorators?.[0])
     method() {}
   },
   class {
-    @decorators.at(0)
+    @(decorators.at(0))
     method() {}
   },
   class {
     @(decorators?.at(0))
     method() {}
   },
   class {
-    @decorators.first
+    @(decorators.first)
     method() {}
   },
   class {
     @(decorators?.first)
     method() {}
   },
   class {
     @(decorators[first])
     method() {}
   },
   class {
     @decorators [first];
     method() {}
   },
   class {
     @(decorators["first"])
     method() {}
   },
-  @(decorators[first])
-  class {
-    method() {}
-  },
-  @(decorators[0])
-  class {
-    method() {}
-  },
+  (
+    @(decorators[first])
+    class {
+      method() {}
+    }
+  ),
+  (
+    @(decorators[0])
+    class {
+      method() {}
+    }
+  ),
 ];

```

**Prettier Similarity**: 74.07%


# js/deferred-import-evaluation/import-defer-attributes-declaration.js
```diff
-import defer * as ns from "x" with { attr: "val" };
+import defer
+* as ns from "x"
+with { attr: "val" }

```

**Prettier Similarity**: 0.00%


# js/deferred-import-evaluation/import-defer.js
```diff
-import defer * as ns from "x";
+import defer
+* as ns from "x"

```

**Prettier Similarity**: 0.00%


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


# js/explicit-resource-management/valid-using-binding-escaped.js
```diff
 {
-  using ab = c;
+  using \u0061b = c;
 }

```

**Prettier Similarity**: 66.67%


# js/export-default/binary_and_template.js
```diff
-export default (function () {} + foo)``;
+export default ((function () {}) + foo)``;

```

**Prettier Similarity**: 0.00%


# js/export-default/body.js
```diff
-export default (class {}[1] = 1);
+export default ((class {})[1] = 1);

```

**Prettier Similarity**: 0.00%


# js/export-default/class_instance.js
```diff
-export default (class {}.getInstance());
+export default (class {}).getInstance();

```

**Prettier Similarity**: 0.00%


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


# js/export-default/function_tostring.js
```diff
-export default (function () {}.toString());
+export default (function () {}).toString();

```

**Prettier Similarity**: 0.00%


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


# js/import-reflection/comments.js
```diff
-/* 0 */ import module /* 1 */ /* 2 */ from /* 3 */ from /* 4 */ "./module.wasm" /* 5 */;
+/* 0 */ import /* 1 */module /* 2 */from /* 3 */
+from; /* 4 */
+("./module.wasm") /* 5 */;

```

**Prettier Similarity**: 0.00%


# js/import-reflection/import-reflection.js
```diff
-import module foo from "./module.wasm";
+import module
+foo;
+from;
+("./module.wasm");

```

**Prettier Similarity**: 0.00%


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


# js/method-chain/13018.js
```diff
 foo(_a).bar().leet();
 foo(-a).bar().leet();
-foo(+a).bar().leet();
-foo(~a).bar().leet();
-foo(++a).bar().leet();
-foo(--a).bar().leet();
-foo(a++).bar().leet();
-foo(a--).bar().leet();
+foo(+a)
+  .bar()
+  .leet();
+foo(~a)
+  .bar()
+  .leet();
+foo(++a)
+  .bar()
+  .leet();
+foo(--a)
+  .bar()
+  .leet();
+foo(a++)
+  .bar()
+  .leet();
+foo(a--)
+  .bar()
+  .leet();

```

**Prettier Similarity**: 10.00%


# js/multiparser-comments/comment-inside.js
```diff
 // #9274
 html`
   <div>
     ${
       this.set && this.set.artist
       /* avoid console errors if `this.set` is undefined */
     }
   </div>
 `;
 
 html`${
   foo
   /* comment */
 }`;
 html`
-  ${
-    foo
-    /* comment */
-  }
+${
+  foo
+  /* comment */
+}
 `;
 
+graphql`${
+  foo
+  /* comment */
+}`;
 graphql`
-  ${
-    foo
-    /* comment */
-  }
-`;
-graphql`
-  ${
-    foo
-    /* comment */
-  }
+${
+  foo
+  /* comment */
+}
 `;
 
-css`
-  ${
-    foo
-    /* comment */
-  }
-`;
+css`${
+  foo
+  /* comment */
+}`;
 css`
-  ${
-    foo
-    /* comment */
-  }
+${
+  foo
+  /* comment */
+}
 `;
 
 markdown`${
   foo
   /* comment */
 }`;
 markdown`
 ${
   foo
   /* comment */
 }
 `;
 
 // https://github.com/prettier/prettier/pull/9278#issuecomment-700589195
 expr1 = html`
   <div>
     ${x(
       foo, // fg
       bar,
-    )}
-  </div>
+    )}</div>
 `;

```

**Prettier Similarity**: 61.19%


# js/multiparser-comments/tagged.js
```diff
 foo(
   html // oops
-  `
-    <div>
-      <p>bar</p>
-      foo
-    </div>
-  `,
+  ` <div><p>bar</p>foo</div> `,
 );

```

**Prettier Similarity**: 33.33%


# js/multiparser-css/colons-after-substitutions.js
```diff
 const Icon = styled.div`
   flex: none;
-  transition: fill 0.25s;
+  transition:    fill 0.25s;
   width: 48px;
   height: 48px;
 
   ${Link}:hover {
-    fill: rebeccapurple;
+    fill:   rebeccapurple;
   }
 
   ${Link} :hover {
     fill: yellow;
   }
 
-  ${media.smallDown}::before {
-  }
+  ${media.smallDown}::before {}
 `;

```

**Prettier Similarity**: 76.47%


# js/multiparser-css/colons-after-substitutions2.js
```diff
 const Icon = styled.div`
   height: 48px;
 
   ${Link}:nth-child(2) {
     fill: rebeccapurple;
   }
 `;
 
 const Icon2 = styled.div`
   height: 48px;
 
-  ${Link}:empty:before {
+  ${Link}:empty:before{
     fill: rebeccapurple;
   }
 `;
 
 const Icon3 = styled.div`
   height: 48px;
 
   ${Link}:not(:first-child) {
     fill: rebeccapurple;
   }
 `;

```

**Prettier Similarity**: 95.65%


# js/multiparser-css/issue-11797.js
```diff
 const paragraph1 = css`
   font-size: 12px;
-  transform: ${vert ? "translateY" : "translateX"}
-    (${translation + handleOffset}px);
+  transform: ${vert ? "translateY" : "translateX"}(${translation + handleOffset}px);
 `;
 
 const paragraph2 = css`
   transform: ${expr}(30px);
 `;
 
 const paragraph3 = css`
   transform: ${expr} (30px);
 `;

```

**Prettier Similarity**: 84.62%


# js/multiparser-css/issue-2883.js
```diff
 export const foo = css`
-  &.foo .${bar}::before,&.foo[value="hello"] .${bar}::before {
-    position: absolute;
-  }
+&.foo .${bar}::before,&.foo[value="hello"] .${bar}::before {
+	position: absolute;
+}
 `;
 
 export const foo2 = css`
-  a.${bar}:focus,a.${bar}:hover {
-    color: red;
-  }
+a.${bar}:focus,a.${bar}:hover {
+  color: red;
+}
 `;
 
 export const global = css`
-  button.${foo}.${bar} {
-    color: #fff;
-  }
+button.${foo}.${bar} {
+  color: #fff;
+}
 `;

```

**Prettier Similarity**: 47.06%


# js/multiparser-css/issue-5961.js
```diff
 const Steps = styled.div`
   @media (min-width: 1px) {
-    ${Step}:nth-child(odd) {
-    }
+    ${Step}:nth-child(odd) {}
   }
 `;
 
 const Steps2 = styled.div`
   @media (min-width: ${breakpoints.lg}) {
     ${Step} {
       margin-bottom: 90px;
     }
 
     ${Step}:nth-child(odd) {
       ${StepItemDescription} {
         grid-row: 1;
         grid-column: 3 / span 3;
       }
       ${Image} {
         grid-row: 1;
         grid-column: 7 / span 6;
       }
     }
 
     ${Step}:nth-child(even) {
       ${Image} {
         grid-row: 1;
         grid-column: 3 / span 6;
       }
       ${StepItemDescription} {
         grid-row: 1;
         grid-column: 10 / span 3;
       }
     }
   }
 `;

```

**Prettier Similarity**: 94.44%


# js/multiparser-css/issue-9072.js
```diff
 const style1 = css`
-  width: ${size + 10}${sizeUnit};
-  border: ${size / 10} ${sizeUnit} solid ${color};
+  width:${size + 10}${sizeUnit};
+  border:${size / 10} ${sizeUnit} solid ${color};
 `;
 
 const style2 = css`
   width: ${size + 10}${sizeUnit};
   border: ${size / 10} ${sizeUnit} solid ${color};
 `;
 
 const style3 = css`
-  foo: ${foo}${bar} ${baz};
+  foo: ${foo}${bar}       ${baz};
 `;

```

**Prettier Similarity**: 76.92%


# js/multiparser-css/styled-components-multiple-expressions.js
```diff
 const Header = styled.div`
   ${something()}
   & > ${Child}:not(:first-child) {
-    margin-left: 5px;
-  }
+margin-left:5px;
+}
 `;
 
 const Header2 = styled.div`
   ${something()}
   & > ${Child}${Child2}:not(:first-child) {
-    margin-left: 5px;
-  }
+margin-left:5px;
+}
 `;
 
-styled.div`
-  ${foo}-idle {
-  }
-`;
+styled.div`${foo}-idle { }`;
 
-styled.div`
-  ${foo}-0-idle {
-  }
-`;
+styled.div`${foo}-0-idle { }`;
 
 styled.div`
-  font-family: "${a}", "${b}";
+font-family: "${a}", "${b}";
 `;

```

**Prettier Similarity**: 51.85%


# js/multiparser-css/styled-components.js
```diff
 const ListItem1 = styled.li``;
 
-const ListItem2 = styled.li``;
+const ListItem2 = styled.li` `;
 
-const Dropdown = styled.div`
-  position: relative;
-`;
+const Dropdown = styled.div`position: relative;`;
 
 const Button = styled.button`
-  color: palevioletred;
+	  color:   palevioletred ;
 
-  font-size: 1em;
+	font-size : 1em   ;
 `;
 
 const TomatoButton = Button.extend`
-  color: tomato;
+	color  : tomato  ;
 
-  border-color: tomato;
+border-color : tomato
+    ;
+
 `;
 
 Button.extend.attr({})`
-  border-color: black;
+border-color : black;
 `;
 
 styled(ExistingComponent)`
-  color: papayawhip;
-  background-color: firebrick;
-`;
+       color : papayawhip ; background-color: firebrick`;
 
 styled.button.attr({})`
-  border: rebeccapurple;
-`;
+border : rebeccapurple`;
 
 styled(ExistingComponent).attr({})`
-  border: rebeccapurple;
-`;
+border : rebeccapurple`;
 
 styled.div`
   color: ${(props) => props.theme.colors.paragraph};
   /* prettier-ignore */
   ${(props) => (props.small ? "font-size: 0.8em;" : "")};
 `;
 
 styled.div`
   color: ${(props) => props.theme.colors.paragraph};
   /* prettier-ignore */
   ${(props) => (props.small ? "font-size: 0.8em;" : "")}
 `;
 
 styled.div`
-  /* prettier-ignore */
+   /* prettier-ignore */
   color: ${(props) => props.theme.colors.paragraph};
   ${(props) => (props.small ? "font-size: 0.8em;" : "")};
 `;
 
 styled.div`
   color: ${(props) => props.theme.colors.paragraph};
   /* prettier-ignore */
   ${(props) => (props.small ? "font-size: 0.8em;" : "")};
   /* prettier-ignore */
   ${(props) => (props.red ? "color: red;" : "")};
 `;
 
 styled.div`
   /* prettier-ignore */
   color: ${(props) => props.theme.colors.paragraph};
   /* prettier-ignore */
   ${(props) => (props.small ? "font-size: 0.8em;" : "")};
   /* prettier-ignore */
   ${(props) => (props.red ? "color: red;" : "")};
   /* prettier-ignore */
 `;
 
 styled.div`
-  ${sanitize} ${fonts}
+ ${sanitize} ${fonts}
   html {
     margin: 0;
   }
 `;
 
 styled.div`
   ${bar}
   baz
 `;
 
 styled.span`
   foo
   ${bar}
   baz
 `;
 
 styled.div`
   foo
   ${bar}
   ${baz}
 `;
 
 styled.span`
   ${foo}
   ${bar}
 `;
 
 styled.div`
   ${foo} bar
 `;
 
 styled.span`
   ${foo} ${bar}
   baz: ${foo}
 `;
 
 styled.span`
-  ${foo};
-  ${bar};
+${foo};
+${bar};
 `;
 
 styled.span`
-  ${foo}: ${bar};
+${foo}: ${bar};
 `;
 
 styled.span`
-  ${foo}: ${bar}
+${foo}: ${bar}
 `;
 
 styled.span`
-  ${foo}: ${bar}
+${foo}:
+${bar}
 `;
 
 styled.span`
-  ${foo}: ${bar};
+${foo}:
+${bar};
 `;
 
 styled.a`
   ${feedbackCountBlockCss}
   text-decoration: none;
 
   ${FeedbackCount} {
     margin: 0;
   }
 `;
 
 const StyledComponent1 = styled.div`
   ${anInterpolation}
   /* a comment */
 
   .aRule {
-    color: red;
+    color: red
   }
 `;
 
 const StyledComponent2 = styled.div`
   ${anInterpolation}
 
   /* a comment */
 
   .aRule {
-    color: red;
+    color: red
   }
 `;
 
 const Direction = styled.span`
   ${({ up }) => up && `color: ${color.positive};`}
   ${({ down }) => down && `color: ${color.negative};`}
 `;
 
 const Direction2 = styled.span`
   ${({ up }) => up && `color: ${color.positive}`};
   ${({ down }) => down && `color: ${color.negative}`};
 `;
 
 const mixin = css`
   color: ${(props) => props.color};
   ${(props) => props.otherProperty}: ${(props) => props.otherValue};
 `;
 
 const foo = styled.div`
   display: flex;
   ${(props) => props.useMixin && mixin}
 `;
 
 const Single1 = styled.div`
-  color: red;
+  color: red
 `;
 
 const Single2 = styled.div`
   color: red;
 `;
 
 const Dropdown2 = styled.div`
   /* A comment to avoid the prettier issue: https://github.com/prettier/prettier/issues/2291 */
   position: relative;
 `;
 
 const bar = styled.div`
   border-radius: 50%;
   border: 5px solid rgba(var(--green-rgb), 0);
   display: inline-block;
   height: 40px;
   width: 40px;
 
   ${(props) =>
     (props.complete || props.inProgress) &&
     css`
       border-color: rgba(var(--green-rgb), 0.15);
     `}
 
   div {
     background-color: var(--purpleTT);
     border-radius: 50%;
     border: 4px solid rgba(var(--purple-rgb), 0.2);
     color: var(--purpleTT);
     display: inline-flex;
 
     ${(props) =>
       props.complete &&
       css`
         background-color: var(--green);
         border-width: 7px;
       `}
 
     ${(props) =>
       (props.complete || props.inProgress) &&
       css`
         border-color: var(--green);
       `}
   }
 `;
 
 const A = styled.a`
   display: inline-block;
   color: #fff;
   ${(props) =>
     props.a &&
     css`
-      display: none;
-    `}
-  height: 30px;
+    display: none;
+  `}
+   height: 30px;
 `;
 
 const Foo = styled.p`
   max-width: 980px;
   ${mediaBreakpointOnlyXs`
     && {
       font-size: 0.8rem;
     }
   `}
 
   &.bottom {
     margin-top: 3rem;
   }
 `;
 
 styled(A)`
   // prettier-ignore
   @media (aaaaaaaaaaaaa) {
 	z-index: ${(props) => (props.isComplete ? "1" : "0")};
   }
 `;
 
 const StyledDiv = styled.div`
   ${(props) => getSize(props.$size.xs)}
   ${(props) => getSize(props.$size.sm, "sm")}
   ${(props) => getSize(props.$size.md, "md")}
 `;

```

**Prettier Similarity**: 88.64%


# js/multiparser-css/url.js
```diff
-styled.div`
-  color: red;
-  background: url(http://example.com?q=${foo});
-`;
+styled.div`color:red;background: url(http://example.com?q=${foo})`;

```

**Prettier Similarity**: 0.00%


# js/multiparser-css/var.js
```diff
 const Something = styled.div`
   background: var(--${one}); /* ... */
   border: 1px solid var(--${two}); /* ... */
 `;
 
 const StyledPurchaseCard = styled(Card)`
   min-width: 200px;
   background-color: var(--${(props) => props.color});
   color: #fff;
 `;
 
 const v1 = css`
-  prop: var(--global--color--${props.variant});
+prop: var(--global--color--${props.variant});
 `;
 
 const v2 = css`
-  background-color: var(--global--color--${props.variant});
+        background-color: var(--global--color--${props.variant});
 
-  &:hover {
-    background-color: var(--global--color--${props.variant}__one);
-  }
-`;
+        &:hover {
+          background-color: var(--global--color--${props.variant}__one);
+        }
+      `;
 
 export const StyledComponent = styled.div`
-  grid-area: area-${(props) => props.propName};
+  grid-area:  area-${(props) => props.propName};
 `;

```

**Prettier Similarity**: 73.08%


# js/multiparser-graphql/comment-tag.js
```diff
 const query = /* GraphQL */ `
-  {
-    user(id: 5) {
+      {
+    user(   id :   5  )  {
       firstName
 
       lastName
     }
   }
 `;

```

**Prettier Similarity**: 77.78%


# js/multiparser-graphql/escape.js
```diff
 gql`
   "\`foo\` mutation payload."
-  type FooPayload {
-    bar: String
+  type      FooPayload       {
+    	bar: String
   }
 `;
 
 gql`
-  type Project {
+type Project {
     "Pattern: \`\${project}\`"
     pattern: String
     """
     Pattern: \`\${project}\`
     """
     pattern: String
 
-    # Also: Escaping the first parentheses...
-    "Pattern: \`\${project}\`"
+	# Also: Escaping the first parentheses...
+	"Pattern: \`$\{project}\`"
     pattern: String
     # Or escaping the first and second parentheses...
-    "Pattern: \`\${project}\`"
+	"Pattern: \`$\{project\}\`"
     pattern: String
-  }
+}
 `;
 
 gql`
   """
   - \`
   - \\\`
   - \\ a
   - \\\\
   - $
   - \$
   - \${
   - \\\${
   - \u1234
   """
   type A {
     a
   }
 `;

```

**Prettier Similarity**: 82.93%


# js/multiparser-graphql/expressions.js
```diff
 graphql(
   schema,
   `
-    query allPartsByManufacturerName($name: String!) {
-      allParts(filter: { manufacturer: { name: $name } }) {
-        ...PartAll
-      }
-    }
-    ${fragments.all}
-  `,
+query allPartsByManufacturerName($name: String!) {
+  allParts(filter:{manufacturer: {name: $name}}) {
+...    PartAll
+}}
+${fragments.all}
+`,
 );
 
 const veryLongVariableNameToMakeTheLineBreak = graphql(
   schema,
   `
-    query allPartsByManufacturerName($name: String!) {
-      allParts(filter: { manufacturer: { name: $name } }) {
-        ...PartAll
-      }
-    }
-    ${fragments.all}
-  `,
+query allPartsByManufacturerName($name: String!) {
+  allParts(filter:{manufacturer: {name: $name}}) {
+...    PartAll
+}}
+${fragments.all}
+`,
 );

```

**Prettier Similarity**: 39.13%


# js/multiparser-graphql/graphql-tag.js
```diff
 import gql from "graphql-tag";
 
 const query = gql`
-  {
-    user(id: 5) {
+      {
+    user(   id :   5  )  {
       firstName
 
       lastName
     }
   }
 `;
 
 // With interpolations:
 
 gql`
-  query User {
-    user(id: 5) {
-      ...UserDetails
-      ...Friends
-    }
+query User {
+  user(id:5){
+    ...UserDetails
+    ...Friends
   }
+}
 
-  ${USER_DETAILS_FRAGMENT}
-  ${FRIENDS_FRAGMENT}
+${USER_DETAILS_FRAGMENT}${FRIENDS_FRAGMENT}
 `;
 
 // Skip if non-toplevel interpolation:
 
 gql`
 query User {
   user(id:${id}){ name }
 }
 `;
 
 // Skip if top-level interpolation within comment:
 
 gql`
 query User {
   user(id:5){ name }
 }
 #${test}
 `;
 
 // Comment on last line:
 
 gql`
-  query User {
-    user(id: 5) {
-      name
-    }
-  }
-  # comment
-`;
+query User {
+  user(id:5){ name }
+}
+# comment`;
 // ` <-- editor syntax highlighting workaround
 
 // Preserve up to one blank line between things and enforce linebreak between
 // interpolations:
 
 gql`
-  # comment
-  ${one}
-  ${two}
-  ${three}
-  ${four}
+# comment
+${one}${two}  ${three}
+${four}
 
-  ${five}
-  # comment
-  ${six}
+${five}
+# comment
+${six}
 
-  # comment
-  ${seven}
-  # comment
+# comment
+${seven}
+# comment
+
+${eight}
+
+  # comment with trailing whitespace      
 
-  ${eight}
 
-  # comment with trailing whitespace
+# blank line above this comment
+
 
-  # blank line above this comment
 `;
 
 // Interpolation directly before and after query:
 
-gql`
-  ${one}
-  query Test {
-    test
-  }
-  ${two}
-`;
+gql`${one} query Test { test }${two}`;
 
 // Only interpolation:
 
-gql`
-  ${test}
-`;
+gql`${test}`;
 
 // Only comment:
 
-gql`
-  # comment
-`;
+gql`# comment`;
 // ` <-- editor syntax highlighting workaround
 
 // Only whitespace:
 
-gql``;
+gql`   `;
 
 // Empty:
 
 gql``;
 
 // Comments after other things:
 // Currently, comments after interpolations are moved to the next line.
 // We might want to keep them on the next line in the future.
 
 gql`
-  ${test}
-  # comment
+  ${test} # comment
 
-  query Test {
-    # comment
+  query Test { # comment
     test # comment
   } # comment
-  ${test}
-  # comment
-  ${test}
-  # comment
+  ${test} # comment
+  ${test} # comment
 
-  ${test}
-  # comment
+  ${test} # comment
 
   # comment
-  ${test}
-  # comment
+  ${test} # comment
 `;
 
 // Larger mixed test:
 
 gql`
-  query User {
-    test
-  }
 
-  ${USER_DETAILS_FRAGMENT}
 
-  # Comment
-  # that continues on a new line
 
-  # and has a blank line in the middle
+query User {
+  test
+}
+
+    
+	
+${USER_DETAILS_FRAGMENT}
 
-  ${FRIENDS_FRAGMENT}
+   # Comment    
+   # that continues on a new line
+
+    
+   # and has a blank line in the middle
+
+    ${FRIENDS_FRAGMENT}
   ${generateFragment({
     totally: "a good idea",
   })}
 
-  ${fragment}
-  #comment
+${fragment}#comment
 
-  fragment another on User {
-    name
-  }
-  ${fragment}
-`;
+fragment another on User { name
+}${fragment}`;

```

**Prettier Similarity**: 56.44%


# js/multiparser-graphql/graphql.js
```diff
 graphql(
   schema,
   `
-    mutation MarkReadNotificationMutation($input: MarkReadNotificationData!) {
-      markReadNotification(data: $input) {
-        notification {
-          seenState
-        }
-      }
-    }
-  `,
+mutation     MarkReadNotificationMutation(
+    $input
+    : MarkReadNotificationData!
+  )
+{ markReadNotification(data: $input ) { notification {seenState} } }`,
 );

```

**Prettier Similarity**: 33.33%


# js/multiparser-graphql/react-relay.js
```diff
 const { graphql } = require("react-relay");
 
 graphql`
-  mutation MarkReadNotificationMutation($input: MarkReadNotificationData!) {
-    markReadNotification(data: $input) {
-      notification {
-        seenState
-      }
-    }
-  }
+ mutation     MarkReadNotificationMutation(
+    $input
+    : MarkReadNotificationData!
+  )
+{ markReadNotification(data: $input ) { notification {seenState} } }
 `;
 
 graphql.experimental`
-  mutation MarkReadNotificationMutation($input: MarkReadNotificationData!) {
-    markReadNotification(data: $input) {
-      notification {
-        seenState
-      }
-    }
-  }
+ mutation     MarkReadNotificationMutation(
+    $input
+    : MarkReadNotificationData!
+  )
+{ markReadNotification(data: $input ) { notification {seenState} } }
 `;

```

**Prettier Similarity**: 33.33%


# js/multiparser-html/html-template-literals.js
```diff
 const nestedFun = /* HTML */ `${outerExpr(1)}
   <script>
     const tpl = html\`<div>\${innerExpr(1)} ${outerExpr(2)}</div>\`;
   </script>`;
 
 const nestedFun2 = /* HTML */ `${outerExpr(1)}
   <script>
-    const tpl = html\` <div>\${innerExpr(1)} ${outerExpr(2)}</div> \`;
+    const tpl = html\`\\n<div>\${innerExpr(1)} ${outerExpr(2)}</div>\\n\`;
   </script>`;
 
 setFoo(
   html`<div>one</div>
     <div>two</div>
     <div>three</div>`,
   secondArgument,
 );
 
 setFoo(
   html`<div>
       <div>nested</div>
     </div>
     <div>two</div>
     <div>three</div>`,
   secondArgument,
 );
 
 setFoo(
   html`<div>
     <div>nested</div>
   </div>`,
   secondArgument,
 );

```

**Prettier Similarity**: 96.88%


# js/multiparser-html/issue-10691.js
```diff
 export default function include_photoswipe(gallery_selector = ".my-gallery") {
-  return /* HTML */ ` <script>
-    window.addEventListener("load", () =>
-      initPhotoSwipeFromDOM("${gallery_selector}"),
-    );
-  </script>`;
+  return /* HTML */ `
+		<script>
+			window.addEventListener("load", () =>
+				initPhotoSwipeFromDOM("${gallery_selector}")
+			);
+		</script>`;
 }

```

**Prettier Similarity**: 25.00%


# js/multiparser-html/lit-html.js
```diff
 import { LitElement, html } from "@polymer/lit-element";
 
 class MyElement extends LitElement {
   static get properties() {
     return {
       mood: { type: String },
     };
   }
 
   constructor() {
     super();
     this.mood = "happy";
   }
 
   render() {
     return html`
-      <style>
-        .mood {
-          color: green;
-        }
-      </style>
+      <style
+      
+      
+      >
+                  .mood { color: green; }
+      </style
+      
+      
+      
+      >
 
-      Web Components are <span class="mood">${this.mood}</span>!
+         Web            Components         are     <span 
+      
+      
+      class="mood"      >${this.mood}</span
+      
+           >!
     `;
   }
 }
 
 customElements.define("my-element", MyElement);
 
-const someHtml1 = html`<div>hello ${world}</div>`;
-const someHtml2 = /* HTML */ `<div>hello ${world}</div>`;
+const someHtml1 = html`<div       > hello ${world} </div     >`;
+const someHtml2 = /* HTML */ `<div      > hello ${world} </div     >`;
 
 html``;
 
 html`<my-element obj=${obj}></my-element>`;
 
-html` <${Footer}>footer content<//> `;
+html`  <${Footer}  >footer      content<//     >  `;
 
-html` <div /> `;
+html`  <div />  `;
 
-html` <div /> `;
+html`
+  <div />
+`;
 
 html`<span>one</span><span>two</span><span>three</span>`;
 
 function HelloWorld() {
   return html`
     <h3>Bar List</h3>
-    ${bars.map((bar) => html` <p>${bar}</p> `)}
+    ${bars.map(
+      (bar) => html`
+       <p>${bar}</p>
+    `,
+    )}
   `;
 }
 
-const trickyParens = html`<script>
-  f((${expr}) / 2);
-</script>`;
-const nestedFun = /* HTML */ `${outerExpr(1)}
-  <script>
-    const tpl = html\`<div>\${innerExpr(1)} ${outerExpr(2)}</div>\`;
-  </script>`;
+const trickyParens = html`<script> f((${expr}) / 2); </script>`;
+const nestedFun = /* HTML */ `${outerExpr(
+  1,
+)} <script>const tpl = html\`<div>\${innerExpr( 1 )} ${outerExpr(
+  2,
+)}</div>\`</script>`;
 
 const closingScriptTagShouldBeEscapedProperly = /* HTML */ `
   <script>
     const html = /* HTML */ \`<script><\\/script>\`;
   </script>
 `;
 
-const closingScriptTag2 = /* HTML */ `<script>
-  const scriptTag = "<\\/script>";
-</script>`;
+const closingScriptTag2 = /* HTML */ `<script>const  scriptTag='<\\/script>'; <\/script>`;
 
 html`
-  <div
-    style="
+ <div style="
  ${foo}
-"
-  ></div>
+"></div>
+`;
+html`
+ <div style=${foo}></div>
 `;
-html` <div style=${foo}></div> `;
 
-html`<div
-  style="   color : red;
-            display    :inline "
-></div>`;
+html`<div style="   color : red;
+            display    :inline ">
+  </div>`;
 
-html`<div
-  style="   color : red;
+html`<div style="   color : red;
 ${foo}
-            display    :inline "
-></div>`;
-html`<div
-  style="   color : red;
+            display    :inline ">
+  </div>`;
+html`<div style="   color : red;
 ${foo}:${bar};
-            display    :inline "
-></div>`;
+            display    :inline ">
+  </div>`;

```

**Prettier Similarity**: 52.43%


# js/multiparser-text/text.js
```diff
 a = {
   viewer: graphql`
     fragment x on Viewer {
-      y(
-        named: [
-          "projects_feedback_ids" # PROJECTS_FEEDBACK_IDS
-        ]
-      ) {
+      y(named: [
+        "projects_feedback_ids" # PROJECTS_FEEDBACK_IDS
+      ]) {
         name
       }
     }
   `,
 };

```

**Prettier Similarity**: 61.54%


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


# js/objects/assignment-expression/object-property.js
```diff
 a = {
-  [(this.resource = resource)]: 1,
+  [this.resource = resource]: 1,
 };

```

**Prettier Similarity**: 66.67%


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


# js/quote-props/objects.js
```diff
 const a = {
   a: "a",
 };
 
 const b = {
   b: "b",
 };
 
 const b2 = {
   // Escapes should stay as escapes and not be unquoted.
   "\u0062": "b",
   "\u0031": "1",
 };
 
 const c = {
   c1: "c1",
   c2: "c2",
 };
 
 const d = {
   d1: "d1",
   "d-2": "d2",
 };
 
 // None of these should become quoted, regardless of the quoteProps value.
 const e = {
   NaN: null,
   1: null,
   1.5: null,
-  0.1: null,
-  1: null,
+  .1: null,
+  1.: null,
   1.0: null,
   999999999999999999999: null,
   0.99999999999999999: null,
-  1e2: null,
-  1e3: null,
-  1e100: null,
+  1E2: null,
+  1e+3: null,
+  1e+100: null,
   0b10: null,
   0o10: null,
   0xf: null,
   2n: null,
 };
 
 const f = {
   // These should be unquoted for quoteProps=as-needed.
   NaN: null,
   1: null,
-  1.5: null,
+  "1.5": null,
   // These should never be unquoted. `1e+100` technically could (it’s the only
   // one where `String(Number(key)) === key`), but we came to the conclusion
   // that it is unexpected.
   ".1": null,
   "1.": null,
   "1.0": null,
-  "999999999999999999999": null,
+  999999999999999999999: null,
   "0.99999999999999999": null,
   "1E2": null,
   "1e+3": null,
   "1e+100": null,
   "0b10": null,
   "0o10": null,
   "0xf": null,
   "2n": null,
 };
 
 Object.entries({
   // To force quotes for quoteProps=consistent.
   "a-": "a-",
   // These can be quoted:
   NaN: "NaN",
   1: "1",
   1.5: "1.5",
   // Prettier will normalize these to `0.1` and `1` – then they can be quoted.
-  0.1: ".1",
-  1: "1.",
+  .1: ".1",
+  1.: "1.",
   // These should never be quoted. The _actual_ keys are shown as comments.
   // Copy-paste this into the console to verify. If we were to convert these
   // numbers into decimal (which completely valid), “information/intent” is
   // lost. Either way, writing code like this is super confusing.
   1.0: "1.0", // 1
   999999999999999999999: "999999999999999999999", // 1e+21
   0.99999999999999999: "0.99999999999999999", // 1
-  1e2: "1E2", // 100
-  1e3: "1e+3", // 1000
-  1e100: "1e+100", // 1e+100 – this one is identical, but would be inconsistent to quote.
+  1E2: "1E2", // 100
+  1e+3: "1e+3", // 1000
+  1e+100: "1e+100", // 1e+100 – this one is identical, but would be inconsistent to quote.
   0b10: "0b10", // 2
   0o10: "0o10", // 8
   0xf: "0xf", // 15
   2n: "2n", // 2
   0xan: "0xan", // 10
 });
 
 // Negative numbers cannot be unquoted.
 !{
   "-1": null,
   "-1.5": null,
 };

```

**Prettier Similarity**: 87.63%


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


# js/range/issue-7082.js
```diff
 export const Button = styled.button`
-  color: blue;
+color: blue;
 `;

```

**Prettier Similarity**: 66.67%


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


# js/require/require.js
```diff
 const {
   one,
   two,
   three,
   four,
   five,
   six,
   seven,
   eight,
   nine,
   ten,
 } = require("./my-utils");
 const {
   one1,
   two1,
   three1,
   four1,
   five1,
   six1,
   seven1,
   eight1,
   nine1,
   ten1,
   eleven1,
 } = require("./my-utils");
 
 const MyReallyExtrememlyLongModuleName = require("MyReallyExtrememlyLongModuleName");
 
-const plugin = require(
-  global.STANDALONE
-    ? path.join(__dirname, "../standalone.js")
-    : path.join(__dirname, ".."),
-);
+const plugin = require(global.STANDALONE
+  ? path.join(__dirname, "../standalone.js")
+  : path.join(__dirname, ".."));
 
-const plugin2 = require(
-  path.join(__dirname, global.STANDALONE ? "../standalone.js" : ".."),
-);
+const plugin2 = require(path.join(
+  __dirname,
+  global.STANDALONE ? "../standalone.js" : "..",
+));

```

**Prettier Similarity**: 78.38%


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


# js/source-phase-imports/import-source-attributes-declaration.js
```diff
-import source s from "x" with { attr: "val" };
+import source
+s;
+from;
+("x");
+with { attr: "val" }

```

**Prettier Similarity**: 0.00%


# js/source-phase-imports/import-source-binding-from.js
```diff
-import source from from "x";
+import source from
+from;
+("x");

```

**Prettier Similarity**: 0.00%


# js/source-phase-imports/import-source-binding-source.js
```diff
-import source source from "x";
+import source
+source;
+from;
+("x");

```

**Prettier Similarity**: 0.00%


# js/source-phase-imports/import-source.js
```diff
-import source x from "x";
+import source
+x;
+from;
+("x");

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


# js/template-literals/css-prop.js
```diff
 function SomeComponent(props) {
   // Create styles as if you're calling css and the class will be applied to the component
   return (
     <div
       css={`
-        color: blue;
-        font-size: 17 px;
+    color: blue;
+    font-size: 17 px;
 
-        &:hover {
-          color: green;
-        }
+    &:hover {
+      color: green;
+    }
 
-        & .some-class {
-          font-size: 20px;
-        }
-      `}
+    & .some-class {
+      font-size: 20px;
+    }
+  `}
     >
       This will be blue until hovered.
       <div className="some-class">This font size will be 20px</div>
     </div>
   );
 }
 
 const TestComponent = ({ children, ...props }) => (
-  <div
-    css={`
-      color: white;
-      background: black;
-    `}
-  >
-    {children}
-  </div>
+  <div css={`color: white; background: black`}>{children}</div>
 );

```

**Prettier Similarity**: 48.48%


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


# js/template-literals/styled-components-with-expressions.js
```diff
 const Button = styled.a`
-  /* Comment */
-  display: ${(props) => props.display};
+/* Comment */
+	display: ${(props) => props.display};
 `;
 
 styled.div`
-  display: ${(props) => props.display};
-  border: ${(props) => props.border}px;
-  margin: 10px ${(props) => props.border}px;
+	display: ${(props) => props.display};
+	border: ${(props) => props.border}px;
+	margin: 10px ${(props) => props.border}px ;
 `;
 
 const EqualDivider = styled.div`
-  margin: 0.5rem;
-  padding: 1rem;
-  background: papayawhip;
+margin: 0.5rem;
+		padding: 1rem;
+	background: papayawhip    ;
 
-  > * {
-    flex: 1;
+	> * {
+	flex: 1;
 
-    &:not(:first-child) {
-      ${(props) => (props.vertical ? "margin-top" : "margin-left")}: 1rem;
-    }
-  }
+	&:not(:first-child) {
+			${(props) => (props.vertical ? "margin-top" : "margin-left")}: 1rem;
+		}
+	}
 `;
 
 const header = css`
-  .top-bar {
-    background: black;
-    margin: 0;
+.top-bar {background:black;
+margin: 0;
     position: fixed;
-    top: 0;
-    left: 0;
-    width: 100%;
-    text-align: center;
-    padding: 15px 0 0 1em;
-    z-index: 9999;
-  }
+	top: 0;left:0;
+	width: 100%;
+    text-align: center     ;
+	padding: 15px  0  0  1em;
+		z-index: 9999;
+}
 
-  .top-bar .logo {
-    height: 30px;
-    margin: auto;
+.top-bar .logo {
+  height: 30px;
+  margin: auto; 
     position: absolute;
-    left: 0;
-    right: 0;
-  }
+	left: 0;right: 0;
+}
 `;

```

**Prettier Similarity**: 34.78%


# js/template-literals/styled-jsx-with-expressions.js
```diff
 <style jsx>{`
   div {
-    display: ${expr};
+  display: ${expr};
     color: ${expr};
     ${expr};
     ${expr};
     background: red;
-    animation: ${expr} 10s ease-out;
+  animation: ${expr} 10s ease-out;
   }
   @media (${expr}) {
-    div.${expr} {
-      color: red;
-    }
-    ${expr} {
-      color: red;
-    }
+   div.${expr} {
+    color: red;
+   }
+  ${expr} {
+    color: red;
+  }
   }
   @media (min-width: ${expr}) {
-    div.${expr} {
-      color: red;
-    }
-    all${expr} {
-      color: red;
-    }
+   div.${expr} {
+    color: red;
+   }
+  all${expr} {
+    color: red;
+  }
   }
   @font-face {
     ${expr}
   }
 `}</style>;
 
 <style jsx>{`
   div {
-    animation: linear ${seconds}s ease-out;
+  animation: linear ${seconds}s ease-out;
   }
 `}</style>;
 
 <style jsx>{`
   div {
-    animation: 3s ease-in 1s ${(foo) => foo.getIterations()} reverse both paused
-      slidein;
+  animation: 3s ease-in 1s ${(foo) => foo.getIterations()} reverse both paused slidein;
   }
 `}</style>;

```

**Prettier Similarity**: 59.52%


# js/template-literals/styled-jsx.js
```diff
 <style jsx>{`
-  /* a comment */
-  div :global(.react-select) {
-    color: red;
-    display: none;
-  }
+	/* a comment */
+	div :global(.react-select) {
+		color: red; display: none
+	}
 `}</style>;
 
 <div>
   <style jsx>{`
-    /* a comment */
-    div :global(.react-select) {
-      color: red;
-      display: none;
-    }
-  `}</style>
+	/* a comment */
+div :global(.react-select) {
+color: red; display: none
+}`}</style>
 </div>;
 
 <div>
-  <style jsx>{`
-    div {
-      color: red;
-    }
-  `}</style>
+  <style jsx>{`div{color:red}`}</style>
 </div>;
 
 <div>
   <style jsx>{`This is invalid css. 
       Shouldn't fail.
             Shouldn't be formatted.`}</style>
 </div>;
 
 const header = css`
-  .top-bar {
-    background: black;
-    margin: 0;
+.top-bar {background:black;
+margin: 0;
     position: fixed;
-    top: 0;
-    left: 0;
-    width: 100%;
-    text-align: center;
-    padding: 15px 0 0 1em;
-    z-index: 9999;
-  }
+	top: 0;left:0;
+	width: 100%;
+    text-align: center     ;
+	padding: 15px  0  0  1em;
+		z-index: 9999;
+}
 
-  .top-bar .logo {
-    height: 30px;
-    margin: auto;
+.top-bar .logo {
+  height: 30px;
+  margin: auto; 
     position: absolute;
-    left: 0;
-    right: 0;
-  }
+	left: 0;right: 0;
+}
 `;
 
 const headerResolve = css.resolve`
-  .top-bar {
-    background: black;
-    margin: 0;
+.top-bar {background:black;
+margin: 0;
     position: fixed;
-    top: 0;
-    left: 0;
-    width: 100%;
-    text-align: center;
-    padding: 15px 0 0 1em;
-    z-index: 9999;
-  }
+	top: 0;left:0;
+	width: 100%;
+    text-align: center     ;
+	padding: 15px  0  0  1em;
+		z-index: 9999;
+}
 
-  .top-bar .logo {
-    height: 30px;
-    margin: auto;
+.top-bar .logo {
+  height: 30px;
+  margin: auto; 
     position: absolute;
-    left: 0;
-    right: 0;
-  }
+	left: 0;right: 0;
+}
 `;
 
 const headerGlobal = css.global`
-  .top-bar {
-    background: black;
-    margin: 0;
+.top-bar {background:black;
+margin: 0;
     position: fixed;
-    top: 0;
-    left: 0;
-    width: 100%;
-    text-align: center;
-    padding: 15px 0 0 1em;
-    z-index: 9999;
-  }
+	top: 0;left:0;
+	width: 100%;
+    text-align: center     ;
+	padding: 15px  0  0  1em;
+		z-index: 9999;
+}
 
-  .top-bar .logo {
-    height: 30px;
-    margin: auto;
+.top-bar .logo {
+  height: 30px;
+  margin: auto; 
     position: absolute;
-    left: 0;
-    right: 0;
-  }
+	left: 0;right: 0;
+}
 `;

```

**Prettier Similarity**: 34.02%


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


# js/throw_expressions/throw_expression.js
```diff
-function save(filename = throw new TypeError("Argument required")) {}
+function save(filename = throw new TypeError("Argument required")
+)
+{
+}
 
 lint(ast, {
-  with: () => throw new Error("avoid using 'with' statements."),
+  with: () => throw new Error("avoid using 'with' statements.")
 });
 
 function getEncoder(encoding) {
-  const encoder =
-    encoding === "utf8"
-      ? new UTF8Encoder()
-      : encoding === "utf16le"
-      ? new UTF16Encoder(false)
-      : encoding === "utf16be"
-      ? new UTF16Encoder(true)
-      : throw new Error("Unsupported encoding");
+  const encoder = encoding === "utf8" ? new UTF8Encoder()
+                : encoding === "utf16le" ? new UTF16Encoder(false)
+                : encoding === "utf16be" ? new UTF16Encoder(true)
+                :
+  throw new Error("Unsupported encoding");
 }
 
 class Product {
   get id() {
     return this._id;
   }
   set id(value) {
-    this._id = value || throw new Error("Invalid value");
+    this._id = value ||
+    throw new Error("Invalid value");
   }
 }

```

**Prettier Similarity**: 53.85%


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


# typescript/as/export_default_as.ts
```diff
-export default (function log() {} as typeof console.log);
+export default (function log() {}) as typeof console.log;

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


# typescript/assert/index.ts
```diff
 const assertString = (x: any): asserts x => {
   console.assert(typeof x === "string");
 };
 
 function assertsString(x: any): asserts x {
   console.assert(typeof x === "string");
 }
 
 const assertStringWithGuard = (x: any): asserts x is string => {
   console.assert(typeof x === "string");
 };
 
 function assertsStringWithGuard(x: any): asserts x is string {
   console.assert(typeof x === "string");
 }
 
 interface AssertFoo {
-  isString(node: any): asserts node;
+  isString(node: any): asserts node ;
 }
 
 class AssertsFoo {
   isBar(): asserts this {
     return;
   }
   isBaz = (): asserts this => {
     return;
   };
 }

```

**Prettier Similarity**: 96.43%


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


# typescript/compiler/castParentheses.ts
```diff
-﻿class a {
+class a {
   static b: any;
 }
 
 var b = <any>a;
 var b = (<any>a).b;
 var b = (<any>a.b).c;
 var b = (<any>a.b()).c;
 var b = <any>new a();
 var b = <any>new a.b();
 var b = (<any>new a()).b;

```

**Prettier Similarity**: 90.91%


# typescript/compiler/commentInNamespaceDeclarationWithIdentifierPathName.ts
```diff
-﻿namespace hello.hi.world {
+namespace hello.hi.world {
   function foo() {}
 }

```

**Prettier Similarity**: 66.67%


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


# typescript/conformance/es6/templates/templateStringWithEmbeddedTypeAssertionOnAdditionES6.ts
```diff
-﻿// @target: ES6
+// @target: ES6
 var x = `abc${<any>(10 + 10)}def`;

```

**Prettier Similarity**: 50.00%


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


# typescript/conformance/types/tuple/contextualTypeWithTuple.ts
```diff
-﻿// no error
+// no error
 var numStrTuple: [number, string] = [5, "hello"];
 var numStrTuple2: [number, string] = [5, "foo", true];
 var numStrBoolTuple: [number, string, boolean] = [5, "foo", true];
 var objNumTuple: [{ a: string }, number] = [{ a: "world" }, 5];
 var strTupleTuple: [string, [number, {}]] = ["bar", [5, { x: 1, y: 1 }]];
 class C {}
 class D {}
 var unionTuple: [C, string | number] = [new C(), "foo"];
 var unionTuple1: [C, string | number] = [new C(), "foo"];
 var unionTuple2: [C, string | number, D] = [new C(), "foo", new D()];
 var unionTuple3: [number, string | number] = [10, "foo"];
 
 numStrTuple = numStrTuple2;
 numStrTuple = numStrBoolTuple;
 
 // error
 objNumTuple = [{}, 5];
 numStrBoolTuple = numStrTuple;
 var strStrTuple: [string, string] = ["foo", "bar", 5];
 
 unionTuple = unionTuple1;
 unionTuple = unionTuple2;
 unionTuple2 = unionTuple;
 numStrTuple = unionTuple3;

```

**Prettier Similarity**: 96.00%


# typescript/conformance/types/tuple/indexerWithTuple.ts
```diff
-﻿var strNumTuple: [string, number] = ["foo", 10];
+var strNumTuple: [string, number] = ["foo", 10];
 var numTupleTuple: [number, [string, number]] = [10, ["bar", 20]];
 var unionTuple1: [number, string | number] = [10, "foo"];
 var unionTuple2: [boolean, string | number] = [true, "foo"];
 
 // no error
 var idx0 = 0;
 var idx1 = 1;
 var ele10 = strNumTuple[0]; // string
 var ele11 = strNumTuple[1]; // number
 var ele12 = strNumTuple[2]; // string | number
 var ele13 = strNumTuple[idx0]; // string | number
 var ele14 = strNumTuple[idx1]; // string | number
 var ele15 = strNumTuple["0"]; // string
 var ele16 = strNumTuple["1"]; // number
 var strNumTuple1 = numTupleTuple[1]; //[string, number];
 var ele17 = numTupleTuple[2]; // number | [string, number]
 var eleUnion10 = unionTuple1[0]; // number
 var eleUnion11 = unionTuple1[1]; // string | number
 var eleUnion12 = unionTuple1[2]; // string | number
 var eleUnion13 = unionTuple1[idx0]; // string | number
 var eleUnion14 = unionTuple1[idx1]; // string | number
 var eleUnion15 = unionTuple1["0"]; // number
 var eleUnion16 = unionTuple1["1"]; // string | number
 
 var eleUnion20 = unionTuple2[0]; // boolean
 var eleUnion21 = unionTuple2[1]; // string | number
 var eleUnion22 = unionTuple2[2]; // string | number | boolean
 var eleUnion23 = unionTuple2[idx0]; // string | number | boolean
 var eleUnion24 = unionTuple2[idx1]; // string | number | boolean
 var eleUnion25 = unionTuple2["0"]; // boolean
 var eleUnion26 = unionTuple2["1"]; // string | number

```

**Prettier Similarity**: 96.88%


# typescript/conformance/types/tuple/typeInferenceWithTupleType.ts
```diff
-﻿function combine<T, U>(x: T, y: U): [T, U] {
+function combine<T, U>(x: T, y: U): [T, U] {
   return [x, y];
 }
 
 var combineResult = combine("string", 10);
 var combineEle1 = combineResult[0]; // string
 var combineEle2 = combineResult[1]; // number
 
 function zip<T, U>(array1: T[], array2: U[]): [[T, U]] {
   if (array1.length != array2.length) {
     return [[undefined, undefined]];
   }
   var length = array1.length;
   var zipResult: [[T, U]];
   for (var i = 0; i < length; ++i) {
     zipResult.push([array1[i], array2[i]]);
   }
   return zipResult;
 }
 
 var zipResult = zip(["foo", "bar"], [5, 6]);
 var zipResultEle = zipResult[0]; // [string, number]
 var zipResultEleEle = zipResult[0][0]; // string

```

**Prettier Similarity**: 95.65%


# typescript/conformance/types/union/unionTypeCallSignatures.ts
```diff
-﻿var numOrDate: number | Date;
+var numOrDate: number | Date;
 var strOrBoolean: string | boolean;
 var strOrNum: string | number;
 
 // If each type in U has call signatures and the sets of call signatures are identical ignoring return types,
 // U has the same set of call signatures, but with return types that are unions of the return types of the respective call signatures from each type in U.
 var unionOfDifferentReturnType: { (a: number): number } | { (a: number): Date };
 numOrDate = unionOfDifferentReturnType(10);
 strOrBoolean = unionOfDifferentReturnType("hello"); // error
 unionOfDifferentReturnType1(true); // error in type of parameter
 
 var unionOfDifferentReturnType1:
   | { (a: number): number; (a: string): string }
   | { (a: number): Date; (a: string): boolean };
 numOrDate = unionOfDifferentReturnType1(10);
 strOrBoolean = unionOfDifferentReturnType1("hello");
 unionOfDifferentReturnType1(true); // error in type of parameter
 unionOfDifferentReturnType1(); // error missing parameter
 
 var unionOfDifferentParameterTypes:
   | { (a: number): number }
   | { (a: string): Date };
 unionOfDifferentParameterTypes(10); // error - no call signatures
 unionOfDifferentParameterTypes("hello"); // error - no call signatures
 unionOfDifferentParameterTypes(); // error - no call signatures
 
 var unionOfDifferentNumberOfSignatures:
   | { (a: number): number }
   | { (a: number): Date; (a: string): boolean };
 unionOfDifferentNumberOfSignatures(); // error - no call signatures
 unionOfDifferentNumberOfSignatures(10); // error - no call signatures
 unionOfDifferentNumberOfSignatures("hello"); // error - no call signatures
 
 var unionWithDifferentParameterCount:
   | { (a: string): string }
   | { (a: string, b: number): number };
 unionWithDifferentParameterCount(); // no  call signature
 unionWithDifferentParameterCount("hello"); // no  call signature
 unionWithDifferentParameterCount("hello", 10); // no  call signature
 
 var unionWithOptionalParameter1:
   | { (a: string, b?: number): string }
   | { (a: string, b?: number): number };
 strOrNum = unionWithOptionalParameter1("hello");
 strOrNum = unionWithOptionalParameter1("hello", 10);
 strOrNum = unionWithOptionalParameter1("hello", "hello"); // error in parameter type
 strOrNum = unionWithOptionalParameter1(); // error
 
 var unionWithOptionalParameter2:
   | { (a: string, b?: number): string }
   | { (a: string, b: number): number };
 strOrNum = unionWithOptionalParameter2("hello"); // error no call signature
 strOrNum = unionWithOptionalParameter2("hello", 10); // error no call signature
 strOrNum = unionWithOptionalParameter2("hello", "hello"); // error no call signature
 strOrNum = unionWithOptionalParameter2(); // error no call signature
 
 var unionWithOptionalParameter3:
   | { (a: string, b?: number): string }
   | { (a: string): number };
 strOrNum = unionWithOptionalParameter3("hello");
 strOrNum = unionWithOptionalParameter3("hello", 10); // error no call signature
 strOrNum = unionWithOptionalParameter3("hello", "hello"); // error no call signature
 strOrNum = unionWithOptionalParameter3(); // error no call signature
 
 var unionWithRestParameter1:
   | { (a: string, ...b: number[]): string }
   | { (a: string, ...b: number[]): number };
 strOrNum = unionWithRestParameter1("hello");
 strOrNum = unionWithRestParameter1("hello", 10);
 strOrNum = unionWithRestParameter1("hello", 10, 11);
 strOrNum = unionWithRestParameter1("hello", "hello"); // error in parameter type
 strOrNum = unionWithRestParameter1(); // error
 
 var unionWithRestParameter2:
   | { (a: string, ...b: number[]): string }
   | { (a: string, b: number): number };
 strOrNum = unionWithRestParameter2("hello"); // error no call signature
 strOrNum = unionWithRestParameter2("hello", 10); // error no call signature
 strOrNum = unionWithRestParameter2("hello", 10, 11); // error no call signature
 strOrNum = unionWithRestParameter2("hello", "hello"); // error no call signature
 strOrNum = unionWithRestParameter2(); // error no call signature
 
 var unionWithRestParameter3:
   | { (a: string, ...b: number[]): string }
   | { (a: string): number };
 strOrNum = unionWithRestParameter3("hello");
 strOrNum = unionWithRestParameter3("hello", 10); // error no call signature
 strOrNum = unionWithRestParameter3("hello", 10, 11); // error no call signature
 strOrNum = unionWithRestParameter3("hello", "hello"); // error no call signature
 strOrNum = unionWithRestParameter3(); // error no call signature
 
 var unionWithRestParameter4:
   | { (...a: string[]): string }
   | { (a: string, b: string): number };
 strOrNum = unionWithRestParameter4("hello"); // error supplied parameters do not match any call signature
 strOrNum = unionWithRestParameter4("hello", "world");

```

**Prettier Similarity**: 98.96%


# typescript/conformance/types/union/unionTypeCallSignatures3.ts
```diff
-﻿function f1(s: string) {}
+function f1(s: string) {}
 function f2(s?: string) {}
 function f3(...s: string[]) {}
 function f4(s: string, s2?: string) {}
 function f5(s?: string, n?: number) {}
 function f6(s?: string, ...n: number[]) {}
 function f7(s: string, ...sRest: string[]) {}
 
 var fUnion:
   | typeof f1
   | typeof f2
   | typeof f3
   | typeof f4
   | typeof f5
   | typeof f6
   | typeof f7;
 
 fUnion(""); // All constituents can be called by passing a single string.

```

**Prettier Similarity**: 94.44%


# typescript/conformance/types/union/unionTypeCallSignatures4.ts
```diff
-﻿type F1 = (a: string, b?: string) => void;
+type F1 = (a: string, b?: string) => void;
 type F2 = (a: string, b?: string, c?: string) => void;
 type F3 = (a: string, ...rest: string[]) => void;
 type F4 = (a: string, b?: string, ...rest: string[]) => void;
 type F5 = (a: string, b: string) => void;
 
 var f12: F1 | F2;
 f12("a");
 f12("a", "b");
 f12("a", "b", "c"); // error
 
 var f34: F3 | F4;
 f34("a");
 f34("a", "b");
 f34("a", "b", "c");
 
 var f1234: F1 | F2 | F3 | F4;
 f1234("a");
 f1234("a", "b");
 f1234("a", "b", "c"); // error
 
 var f12345: F1 | F2 | F3 | F4 | F5;
 f12345("a"); // error
 f12345("a", "b");
 f12345("a", "b", "c"); // error

```

**Prettier Similarity**: 96.00%


# typescript/conformance/types/union/unionTypeConstructSignatures.ts
```diff
-﻿var numOrDate: number | Date;
+var numOrDate: number | Date;
 var strOrBoolean: string | boolean;
 var strOrNum: string | number;
 
 // If each type in U has construct signatures and the sets of construct signatures are identical ignoring return types,
 // U has the same set of construct signatures, but with return types that are unions of the return types of the respective construct signatures from each type in U.
 var unionOfDifferentReturnType:
   | { new (a: number): number }
   | { new (a: number): Date };
 numOrDate = new unionOfDifferentReturnType(10);
 strOrBoolean = new unionOfDifferentReturnType("hello"); // error
 new unionOfDifferentReturnType1(true); // error in type of parameter
 
 var unionOfDifferentReturnType1:
   | { new (a: number): number; new (a: string): string }
   | { new (a: number): Date; new (a: string): boolean };
 numOrDate = new unionOfDifferentReturnType1(10);
 strOrBoolean = new unionOfDifferentReturnType1("hello");
 new unionOfDifferentReturnType1(true); // error in type of parameter
 new unionOfDifferentReturnType1(); // error missing parameter
 
 var unionOfDifferentParameterTypes:
   | { new (a: number): number }
   | { new (a: string): Date };
 new unionOfDifferentParameterTypes(10); // error - no call signatures
 new unionOfDifferentParameterTypes("hello"); // error - no call signatures
 new unionOfDifferentParameterTypes(); // error - no call signatures
 
 var unionOfDifferentNumberOfSignatures:
   | { new (a: number): number }
   | { new (a: number): Date; new (a: string): boolean };
 new unionOfDifferentNumberOfSignatures(); // error - no call signatures
 new unionOfDifferentNumberOfSignatures(10); // error - no call signatures
 new unionOfDifferentNumberOfSignatures("hello"); // error - no call signatures
 
 var unionWithDifferentParameterCount:
   | { new (a: string): string }
   | { new (a: string, b: number): number };
 new unionWithDifferentParameterCount(); // no  call signature
 new unionWithDifferentParameterCount("hello"); // no  call signature
 new unionWithDifferentParameterCount("hello", 10); // no  call signature
 
 var unionWithOptionalParameter1:
   | { new (a: string, b?: number): string }
   | { new (a: string, b?: number): number };
 strOrNum = new unionWithOptionalParameter1("hello");
 strOrNum = new unionWithOptionalParameter1("hello", 10);
 strOrNum = new unionWithOptionalParameter1("hello", "hello"); // error in parameter type
 strOrNum = new unionWithOptionalParameter1(); // error
 
 var unionWithOptionalParameter2:
   | { new (a: string, b?: number): string }
   | { new (a: string, b: number): number };
 strOrNum = new unionWithOptionalParameter2("hello"); // error no call signature
 strOrNum = new unionWithOptionalParameter2("hello", 10); // error no call signature
 strOrNum = new unionWithOptionalParameter2("hello", "hello"); // error no call signature
 strOrNum = new unionWithOptionalParameter2(); // error no call signature
 
 var unionWithOptionalParameter3:
   | { new (a: string, b?: number): string }
   | { new (a: string): number };
 strOrNum = new unionWithOptionalParameter3("hello"); // error no call signature
 strOrNum = new unionWithOptionalParameter3("hello", 10); // error no call signature
 strOrNum = new unionWithOptionalParameter3("hello", "hello"); // error no call signature
 strOrNum = new unionWithOptionalParameter3(); // error no call signature
 
 var unionWithRestParameter1:
   | { new (a: string, ...b: number[]): string }
   | { new (a: string, ...b: number[]): number };
 strOrNum = new unionWithRestParameter1("hello");
 strOrNum = new unionWithRestParameter1("hello", 10);
 strOrNum = new unionWithRestParameter1("hello", 10, 11);
 strOrNum = new unionWithRestParameter1("hello", "hello"); // error in parameter type
 strOrNum = new unionWithRestParameter1(); // error
 
 var unionWithRestParameter2:
   | { new (a: string, ...b: number[]): string }
   | { new (a: string, b: number): number };
 strOrNum = new unionWithRestParameter2("hello"); // error no call signature
 strOrNum = new unionWithRestParameter2("hello", 10); // error no call signature
 strOrNum = new unionWithRestParameter2("hello", 10, 11); // error no call signature
 strOrNum = new unionWithRestParameter2("hello", "hello"); // error no call signature
 strOrNum = new unionWithRestParameter2(); // error no call signature
 
 var unionWithRestParameter3:
   | { new (a: string, ...b: number[]): string }
   | { new (a: string): number };
 strOrNum = new unionWithRestParameter3("hello"); // error no call signature
 strOrNum = new unionWithRestParameter3("hello", 10); // error no call signature
 strOrNum = new unionWithRestParameter3("hello", 10, 11); // error no call signature
 strOrNum = new unionWithRestParameter3("hello", "hello"); // error no call signature
 strOrNum = new unionWithRestParameter3(); // error no call signature

```

**Prettier Similarity**: 98.91%


# typescript/conformance/types/union/unionTypeEquivalence.ts
```diff
-﻿// A | B is equivalent to A if B is a subtype of A
+// A | B is equivalent to A if B is a subtype of A
 class C {}
 class D extends C {
   foo() {}
 }
 var x: C;
 var x: C | D;
 
 // A | B is equivalent to B | A.
 var y: string | number;
 var y: number | string;
 
 // AB | C is equivalent to A | BC, where AB is A | B and BC is B | C.
 var z: string | number | boolean;
 var z: (string | number) | boolean;
 var z: string | (number | boolean);
 var AB: string | number;
 var BC: number | boolean;
 var z1: typeof AB | boolean;
 var z1: string | typeof BC;

```

**Prettier Similarity**: 95.00%


# typescript/conformance/types/union/unionTypeFromArrayLiteral.ts
```diff
-﻿// The resulting type an array literal expression is determined as follows:
+// The resulting type an array literal expression is determined as follows:
 // If the array literal is empty, the resulting type is an array type with the element type Undefined.
 // Otherwise, if the array literal is contextually typed by a type that has a property with the numeric name ‘0’, the resulting type is a tuple type constructed from the types of the element expressions.
 // Otherwise, the resulting type is an array type with an element type that is the union of the types of the element expressions.
 
 var arr1 = [1, 2]; // number[]
 var arr2 = ["hello", true]; // (string | number)[]
 var arr3Tuple: [number, string] = [3, "three"]; // [number, string]
 var arr4Tuple: [number, string] = [3, "three", "hello"]; // [number, string, string]
 var arrEmpty = [];
 var arr5Tuple: {
   0: string;
   5: number;
 } = ["hello", true, false, " hello", true, 10, "any"]; // Tuple
 class C {
   foo() {}
 }
 class D {
   foo2() {}
 }
 class E extends C {
   foo3() {}
 }
 class F extends C {
   foo4() {}
 }
 var c: C, d: D, e: E, f: F;
 var arr6 = [c, d]; // (C | D)[]
 var arr7 = [c, d, e]; // (C | D)[]
 var arr8 = [c, e]; // C[]
 var arr9 = [e, f]; // (E|F)[]

```

**Prettier Similarity**: 96.77%


# typescript/conformance/types/union/unionTypeIndexSignature.ts
```diff
-﻿var numOrDate: number | Date;
+var numOrDate: number | Date;
 var anyVar: number;
 
 // If each type in U has a string index signature,
 // U has a string index signature of a union type of the types of the string index signatures from each type in U.
 
 var unionOfDifferentReturnType: { [a: string]: number } | { [a: string]: Date };
 numOrDate = unionOfDifferentReturnType["hello"]; // number | Date
 numOrDate = unionOfDifferentReturnType[10]; // number | Date
 
 var unionOfTypesWithAndWithoutStringSignature:
   | { [a: string]: number }
   | boolean;
 anyVar = unionOfTypesWithAndWithoutStringSignature["hello"]; // any
 anyVar = unionOfTypesWithAndWithoutStringSignature[10]; // any
 
 // If each type in U has a numeric index signature,
 // U has a numeric index signature of a union type of the types of the numeric index signatures from each type in U.
 var unionOfDifferentReturnType1:
   | { [a: number]: number }
   | { [a: number]: Date };
 numOrDate = unionOfDifferentReturnType1["hello"]; // any
 numOrDate = unionOfDifferentReturnType1[10]; // number | Date
 
 var unionOfTypesWithAndWithoutStringSignature1:
   | { [a: number]: number }
   | boolean;
 anyVar = unionOfTypesWithAndWithoutStringSignature1["hello"]; // any
 anyVar = unionOfTypesWithAndWithoutStringSignature1[10]; // any

```

**Prettier Similarity**: 96.55%


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


# typescript/decorators/legacy.ts
```diff
 [
-  @decorator()
-  class {},
-  @decorator()
-  class A {},
+  (
+    @decorator()
+    class {}
+  ),
+  (
+    @decorator()
+    class A {}
+  ),
 ];
 
 class A {
   @decorator() accessor #field;
 }
 
 class B {
   @decorator() #field() {}
 }

```

**Prettier Similarity**: 55.56%


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


# typescript/export-default/function_as.ts
```diff
-export default (function log() {} as typeof console.log);
+export default (function log() {}) as typeof console.log;

```

**Prettier Similarity**: 0.00%


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


# typescript/instantiation-expression/property-access.ts
```diff
-(Array<string>).name;
-(fn1<string>).bind(obj);
-(fn2<string, number>).bind(obj);
-a[Array<string>];
-a[(Array<string>).name];
-(Array<string>).a;
-(Array<string>)?.a;
-(Array<string>)[a];
-(Array<string>)?.[a];
-(Array<string>)["a"];
-(Array<string>)?.["a"];
-(Array<string>)[`a`];
-(Array<string>)?.[`a`];
-(Array<string>)[Array<string>];
+// FIXME
+// TODO: parse issue
+// (Array<string>).name;
+// (fn1<string>).bind(obj);
+// (fn2<string, number>).bind(obj);
+// a[(Array<string>)];
+// a[(Array<string>).name];
+// (Array<string>).a;
+// (Array<string>)?.a;
+// (Array<string>)[a];
+// (Array<string>)?.[a];
+// (Array<string>)["a"];
+// (Array<string>)?.["a"];
+// (Array<string>)[`a`];
+// (Array<string>)?.[`a`];
+// (Array<string>)[(Array<string>)];

```

**Prettier Similarity**: 0.00%


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


# typescript/range/export-assignment.ts
```diff
-f();
+f ( );
 export = f;
-g();
+g(  )

```

**Prettier Similarity**: 33.33%


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


# typescript/satisfies-operators/export-default-as.ts
```diff
-export default (function log() {} satisfies typeof console.log);
+export default (function log() {}) satisfies typeof console.log;

```

**Prettier Similarity**: 0.00%


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


