## Overall Metrics

**Average compatibility**: 96.70

<details>
    <summary>Definition</summary>

    $$average = \frac\{\sum_{file}^\{files}compatibility_\{file}}\{files}$$
</details>

**Compatible lines**: 97.28

<details>
    <summary>Definition</summary>

    $$average = \frac{\sum_{file}^{files}matching\_lines_{file}}{max(lines_{rome}, lines_{prettier})}$$
</details>

[Metric definition discussion](https://github.com/rome/tools/issues/2555#issuecomment-1124787893)


## Test cases

### js/array-spread/multiple.js

**Prettier Similarity**: 100.00%


### js/arrays/empty.js

**Prettier Similarity**: 100.00%


### js/arrays/holes-in-args.js

**Prettier Similarity**: 100.00%


### js/arrays/issue-10159.js
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


### js/arrays/last.js

**Prettier Similarity**: 100.00%


### js/arrays/nested.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers-in-args.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers-in-assignment.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers-negative-comment-after-minus.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers-negative.js
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


### js/arrays/numbers-trailing-comma.js

**Prettier Similarity**: 100.00%


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


### js/arrays/numbers-with-trailing-comments.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers-with-tricky-comments.js
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


### js/arrays/numbers1.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers2.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers3.js

**Prettier Similarity**: 100.00%


### js/arrays/preserve_empty_lines.js

**Prettier Similarity**: 100.00%


### js/arrow-call/arrow_call.js

**Prettier Similarity**: 100.00%


### js/arrow-call/class-property.js

**Prettier Similarity**: 100.00%


### js/arrows/arrow_function_expression.js

**Prettier Similarity**: 100.00%


### js/arrows/assignment-chain-with-arrow-chain.js

**Prettier Similarity**: 100.00%


### js/arrows/block_like.js

**Prettier Similarity**: 100.00%


### js/arrows/call.js

**Prettier Similarity**: 100.00%


### js/arrows/chain-as-arg.js

**Prettier Similarity**: 100.00%


### js/arrows/comment.js

**Prettier Similarity**: 100.00%


### js/arrows/curried.js
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


### js/arrows/currying-2.js
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


### js/arrows/currying-3.js

**Prettier Similarity**: 100.00%


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


### js/arrows/currying.js

**Prettier Similarity**: 100.00%


### js/arrows/issue-1389-curry.js
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


### js/arrows/issue-4166-curry.js

**Prettier Similarity**: 100.00%


### js/arrows/long-call-no-args.js

**Prettier Similarity**: 100.00%


### js/arrows/long-contents.js

**Prettier Similarity**: 100.00%


### js/arrows/parens.js

**Prettier Similarity**: 100.00%


### js/arrows/semi/semi.js

**Prettier Similarity**: 100.00%


### js/assignment-comments/call.js

**Prettier Similarity**: 100.00%


### js/assignment-comments/call2.js

**Prettier Similarity**: 100.00%


### js/assignment-comments/function.js

**Prettier Similarity**: 100.00%


### js/assignment-comments/identifier.js

**Prettier Similarity**: 100.00%


### js/assignment-comments/number.js

**Prettier Similarity**: 100.00%


### js/assignment-comments/string.js

**Prettier Similarity**: 100.00%


### js/assignment-expression/assignment_expression.js

**Prettier Similarity**: 100.00%


### js/assignment/binaryish.js

**Prettier Similarity**: 100.00%


### js/assignment/call-with-template.js

**Prettier Similarity**: 100.00%


### js/assignment/chain-two-segments.js

**Prettier Similarity**: 100.00%


### js/assignment/chain.js

**Prettier Similarity**: 100.00%


### js/assignment/destructuring-array.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-1419.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-15534.js
```diff
-params["redirectTo"] =
-  `${window.location.pathname}${window.location.search}${window.location.hash}`;
+params[
+  "redirectTo"
+] = `${window.location.pathname}${window.location.search}${window.location.hash}`;
 
-params["redirectTo"]["codePointAt"]["name"] =
-  `${window.location.pathname}${window.location.search}${window.location.hash}`;
+params["redirectTo"]["codePointAt"][
+  "name"
+] = `${window.location.pathname}${window.location.search}${window.location.hash}`;
 
-params.redirectTo.bar.bar.ba.barab["foo"].abr =
-  `${window.location.pathname}${window.location.search}${window.location.hash}`;
+params.redirectTo.bar.bar.ba.barab[
+  "foo"
+].abr = `${window.location.pathname}${window.location.search}${window.location.hash}`;

```

**Prettier Similarity**: 18.18%


### js/assignment/issue-1966.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-2184.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-2482-1.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-2482-2.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-2540.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-3819.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-4094.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-6922.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-7572.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-7961.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-8218.js

**Prettier Similarity**: 100.00%


### js/assignment/sequence.js

**Prettier Similarity**: 100.00%


### js/assignment/unary.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/arrow.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/bitwise-flags.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/call.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/comment.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/equality.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/if.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/in_instanceof.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/inline-object-array.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/like-regexp.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/math.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/return.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/short-right.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/test.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/unary.js

**Prettier Similarity**: 100.00%


### js/binary_math/parens.js

**Prettier Similarity**: 100.00%


### js/bracket-spacing/array.js

**Prettier Similarity**: 100.00%


### js/bracket-spacing/object.js

**Prettier Similarity**: 100.00%


### js/break-calls/break.js

**Prettier Similarity**: 100.00%


### js/break-calls/parent.js

**Prettier Similarity**: 100.00%


### js/break-calls/react.js

**Prettier Similarity**: 100.00%


### js/break-calls/reduce.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/expression-2nd-arg.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/issue-12892.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/issue-13237.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/issue-14454.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/issue-2456.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/issue-4401.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/issue-5172.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/test.js

**Prettier Similarity**: 100.00%


### js/call/no-argument/special-cases.js

**Prettier Similarity**: 100.00%


### js/class-comment/class-property.js

**Prettier Similarity**: 100.00%


### js/class-comment/misc.js

**Prettier Similarity**: 100.00%


### js/class-comment/superclass.js

**Prettier Similarity**: 100.00%


### js/class-extends/complex.js

**Prettier Similarity**: 100.00%


### js/class-extends/extends.js

**Prettier Similarity**: 100.00%


### js/classes/asi.js

**Prettier Similarity**: 100.00%


### js/classes/assignment.js

**Prettier Similarity**: 100.00%


### js/classes/binary.js

**Prettier Similarity**: 100.00%


### js/classes/call.js

**Prettier Similarity**: 100.00%


### js/classes/empty.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/async.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/computed.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/get.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/set.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/static-async.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/static-get.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/static-set.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/static-static.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/static.js

**Prettier Similarity**: 100.00%


### js/classes/member.js

**Prettier Similarity**: 100.00%


### js/classes/method.js

**Prettier Similarity**: 100.00%


### js/classes/new.js

**Prettier Similarity**: 100.00%


### js/classes/property.js

**Prettier Similarity**: 100.00%


### js/classes/super.js

**Prettier Similarity**: 100.00%


### js/classes/ternary.js

**Prettier Similarity**: 100.00%


### js/classes/top-level-super/example.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/binary-expr.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/closure-compiler-type-cast.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/comment-in-the-middle.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/comment-placement.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/extra-spaces-and-asterisks.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/iife-issue-5850-isolated.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/iife.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/issue-4124.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/issue-8045.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/issue-9358.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/member.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/nested.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/non-casts.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/object-with-comment.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/satisfies.js
```diff
-module.exports = /** @satisfies {Record<string, string>} */ ({
+module.exports = /** @satisfies {Record<string, string>} */ {
   hello: 1337,
-});
+};

```

**Prettier Similarity**: 33.33%


### js/comments-closure-typecast/superclass.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/ways-to-specify-type.js

**Prettier Similarity**: 100.00%


### js/comments/arrow.js

**Prettier Similarity**: 100.00%


### js/comments/assignment-pattern.js

**Prettier Similarity**: 100.00%


### js/comments/before-comma.js

**Prettier Similarity**: 100.00%


### js/comments/binary-expressions-block-comments.js

**Prettier Similarity**: 100.00%


### js/comments/binary-expressions-parens.js

**Prettier Similarity**: 100.00%


### js/comments/binary-expressions-single-comments.js

**Prettier Similarity**: 100.00%


### js/comments/binary-expressions.js

**Prettier Similarity**: 100.00%


### js/comments/blank.js

**Prettier Similarity**: 100.00%


### js/comments/break-continue-statements.js

**Prettier Similarity**: 100.00%


### js/comments/call_comment.js

**Prettier Similarity**: 100.00%


### js/comments/class.js

**Prettier Similarity**: 100.00%


### js/comments/dangling.js

**Prettier Similarity**: 100.00%


### js/comments/dangling_array.js

**Prettier Similarity**: 100.00%


### js/comments/dangling_for.js

**Prettier Similarity**: 100.00%


### js/comments/dynamic_imports.js

**Prettier Similarity**: 100.00%


### js/comments/emoji.js

**Prettier Similarity**: 100.00%


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


### js/comments/export-and-import.js

**Prettier Similarity**: 100.00%


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


### js/comments/first-line.js

**Prettier Similarity**: 100.00%


### js/comments/flow-types/inline.js

**Prettier Similarity**: 100.00%


### js/comments/function-declaration.js

**Prettier Similarity**: 100.00%


### js/comments/function/between-parentheses-and-function-body.js
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


### js/comments/if.js

**Prettier Similarity**: 100.00%


### js/comments/issue-3532.js

**Prettier Similarity**: 100.00%


### js/comments/issues.js

**Prettier Similarity**: 100.00%


### js/comments/jsdoc-nestled-dangling.js
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


### js/comments/jsdoc-nestled.js
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


### js/comments/jsdoc.js

**Prettier Similarity**: 100.00%


### js/comments/last-arg.js

**Prettier Similarity**: 100.00%


### js/comments/multi-comments-2.js

**Prettier Similarity**: 100.00%


### js/comments/multi-comments-on-same-line-2.js

**Prettier Similarity**: 100.00%


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


### js/comments/multi-comments.js

**Prettier Similarity**: 100.00%


### js/comments/preserve-new-line-last.js

**Prettier Similarity**: 100.00%


### js/comments/single-star-jsdoc.js

**Prettier Similarity**: 100.00%


### js/comments/switch.js

**Prettier Similarity**: 100.00%


### js/comments/tagged-template-literal.js
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


### js/comments/template-literal.js

**Prettier Similarity**: 100.00%


### js/comments/trailing-jsdocs.js
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


### js/comments/trailing_space.js

**Prettier Similarity**: 100.00%


### js/comments/try.js

**Prettier Similarity**: 100.00%


### js/comments/variable_declarator.js

**Prettier Similarity**: 100.00%


### js/comments/while.js

**Prettier Similarity**: 100.00%


### js/computed-props/classes.js

**Prettier Similarity**: 100.00%


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


### js/conditional/new-expression.js

**Prettier Similarity**: 100.00%


### js/conditional/new-ternary-examples.js

**Prettier Similarity**: 100.00%


### js/conditional/new-ternary-spec.js

**Prettier Similarity**: 100.00%


### js/conditional/no-confusing-arrow.js

**Prettier Similarity**: 100.00%


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


### js/cursor/comments-1.js

**Prettier Similarity**: 100.00%


### js/cursor/comments-2.js

**Prettier Similarity**: 100.00%


### js/cursor/comments-3.js

**Prettier Similarity**: 100.00%


### js/cursor/comments-4.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-0.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-1.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-10.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-2.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-3.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-4.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-5.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-6.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-7.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-8.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-9.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-emoji.js

**Prettier Similarity**: 100.00%


### js/cursor/file-start-with-comment-1.js

**Prettier Similarity**: 100.00%


### js/cursor/file-start-with-comment-2.js

**Prettier Similarity**: 100.00%


### js/cursor/file-start-with-comment-3.js

**Prettier Similarity**: 100.00%


### js/cursor/range-0.js

**Prettier Similarity**: 100.00%


### js/cursor/range-1.js

**Prettier Similarity**: 100.00%


### js/cursor/range-2.js

**Prettier Similarity**: 100.00%


### js/cursor/range-3.js

**Prettier Similarity**: 100.00%


### js/cursor/range-4.js

**Prettier Similarity**: 100.00%


### js/cursor/range-5.js

**Prettier Similarity**: 100.00%


### js/cursor/range-6.js

**Prettier Similarity**: 100.00%


### js/cursor/range-7.js

**Prettier Similarity**: 100.00%


### js/cursor/range-8.js

**Prettier Similarity**: 100.00%


### js/directives/escaped.js

**Prettier Similarity**: 100.00%


### js/directives/issue-7346.js

**Prettier Similarity**: 100.00%


### js/directives/last-line-0.js

**Prettier Similarity**: 100.00%


### js/directives/last-line-1.js

**Prettier Similarity**: 100.00%


### js/directives/last-line-2.js

**Prettier Similarity**: 100.00%


### js/directives/newline.js

**Prettier Similarity**: 100.00%


### js/directives/no-newline.js

**Prettier Similarity**: 100.00%


### js/directives/test.js

**Prettier Similarity**: 100.00%


### js/dynamic-import/assertions.js

**Prettier Similarity**: 100.00%


### js/dynamic-import/test.js

**Prettier Similarity**: 100.00%


### js/empty-paren-comment/class-property.js

**Prettier Similarity**: 100.00%


### js/empty-paren-comment/class.js

**Prettier Similarity**: 100.00%


### js/empty-paren-comment/empty_paren_comment.js

**Prettier Similarity**: 100.00%


### js/empty-statement/body.js

**Prettier Similarity**: 100.00%


### js/empty-statement/no-newline.js

**Prettier Similarity**: 100.00%


### js/end-of-line/example.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_arrow_expression.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_call_expression.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_class_declaration.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_class_expression.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_function_declaration.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_function_declaration_async.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_function_declaration_named.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_function_expression.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_function_expression_named.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_new_expression.js

**Prettier Similarity**: 100.00%


### js/export-default/binary_and_template.js

**Prettier Similarity**: 100.00%


### js/export-default/body.js

**Prettier Similarity**: 100.00%


### js/export-default/class_instance.js

**Prettier Similarity**: 100.00%


### js/export-default/function_in_template.js

**Prettier Similarity**: 100.00%


### js/export-default/function_tostring.js

**Prettier Similarity**: 100.00%


### js/export-default/iife.js

**Prettier Similarity**: 100.00%


### js/export-star/export-star-as-default.js

**Prettier Similarity**: 100.00%


### js/export-star/export-star-as-reserved-word.js

**Prettier Similarity**: 100.00%


### js/export-star/export-star-as-string.js

**Prettier Similarity**: 100.00%


### js/export-star/export-star-as-string2.js

**Prettier Similarity**: 100.00%


### js/export-star/export-star-as.js

**Prettier Similarity**: 100.00%


### js/export-star/export-star.js

**Prettier Similarity**: 100.00%


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


### js/export/bracket.js

**Prettier Similarity**: 100.00%


### js/export/empty.js

**Prettier Similarity**: 100.00%


### js/export/same-local-and-exported.js

**Prettier Similarity**: 100.00%


### js/export/test.js

**Prettier Similarity**: 100.00%


### js/export/undefined.js

**Prettier Similarity**: 100.00%


### js/expression_statement/no_regression.js

**Prettier Similarity**: 100.00%


### js/expression_statement/use_strict.js

**Prettier Similarity**: 100.00%


### js/for-of/async-identifier.js

**Prettier Similarity**: 100.00%


### js/for/comment.js

**Prettier Similarity**: 100.00%


### js/for/continue-and-break-comment-1.js

**Prettier Similarity**: 100.00%


### js/for/continue-and-break-comment-2.js

**Prettier Similarity**: 100.00%


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


### js/for/for.js

**Prettier Similarity**: 100.00%


### js/for/in.js

**Prettier Similarity**: 100.00%


### js/for/var.js

**Prettier Similarity**: 100.00%


### js/function-comments/params-trail-comments.js

**Prettier Similarity**: 100.00%


### js/function-first-param/function_expression.js

**Prettier Similarity**: 100.00%


### js/function-single-destructuring/array.js

**Prettier Similarity**: 100.00%


### js/function/function_expression.js

**Prettier Similarity**: 100.00%


### js/function/issue-10277.js

**Prettier Similarity**: 100.00%


### js/functional-composition/functional_compose.js

**Prettier Similarity**: 100.00%


### js/functional-composition/gobject_connect.js

**Prettier Similarity**: 100.00%


### js/functional-composition/lodash_flow.js

**Prettier Similarity**: 100.00%


### js/functional-composition/lodash_flow_right.js

**Prettier Similarity**: 100.00%


### js/functional-composition/mongo_connect.js

**Prettier Similarity**: 100.00%


### js/functional-composition/pipe-function-calls-with-comments.js

**Prettier Similarity**: 100.00%


### js/functional-composition/pipe-function-calls.js

**Prettier Similarity**: 100.00%


### js/functional-composition/ramda_compose.js

**Prettier Similarity**: 100.00%


### js/functional-composition/ramda_pipe.js

**Prettier Similarity**: 100.00%


### js/functional-composition/redux_compose.js

**Prettier Similarity**: 100.00%


### js/functional-composition/redux_connect.js

**Prettier Similarity**: 100.00%


### js/functional-composition/reselect_createselector.js

**Prettier Similarity**: 100.00%


### js/functional-composition/rxjs_pipe.js

**Prettier Similarity**: 100.00%


### js/generator/anonymous.js

**Prettier Similarity**: 100.00%


### js/generator/async.js

**Prettier Similarity**: 100.00%


### js/generator/function-name-starts-with-get.js

**Prettier Similarity**: 100.00%


### js/identifier/for-of/await.js

**Prettier Similarity**: 100.00%


### js/identifier/for-of/let.js

**Prettier Similarity**: 100.00%


### js/identifier/parentheses/const.js

**Prettier Similarity**: 100.00%


### js/identifier/parentheses/let.js

**Prettier Similarity**: 100.00%


### js/if/comment_before_else.js

**Prettier Similarity**: 100.00%


### js/if/else.js

**Prettier Similarity**: 100.00%


### js/if/expr_and_same_line_comments.js
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


### js/if/if_comments.js

**Prettier Similarity**: 100.00%


### js/if/trailing_comment.js

**Prettier Similarity**: 100.00%


### js/ignore/ignore-2.js

**Prettier Similarity**: 100.00%


### js/ignore/ignore.js

**Prettier Similarity**: 100.00%


### js/ignore/issue-10661.js

**Prettier Similarity**: 100.00%


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


### js/ignore/issue-9335.js

**Prettier Similarity**: 100.00%


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


### js/ignore/semi/asi.js

**Prettier Similarity**: 100.00%


### js/ignore/semi/directive.js

**Prettier Similarity**: 100.00%


### js/import-meta/import_meta.js

**Prettier Similarity**: 100.00%


### js/import/brackets.js

**Prettier Similarity**: 100.00%


### js/import/comments.js

**Prettier Similarity**: 100.00%


### js/import/empty-import.js

**Prettier Similarity**: 100.00%


### js/import/inline.js

**Prettier Similarity**: 100.00%


### js/import/long-line.js

**Prettier Similarity**: 100.00%


### js/import/multiple_standalones.js

**Prettier Similarity**: 100.00%


### js/import/same-local-and-imported.js

**Prettier Similarity**: 100.00%


### js/in/arrow-function-invalid.js

**Prettier Similarity**: 100.00%


### js/in/arrow-function.js

**Prettier Similarity**: 100.00%


### js/invalid-code/duplicate_bindings.js

**Prettier Similarity**: 100.00%


### js/label/block-statement-and-regexp.js

**Prettier Similarity**: 100.00%


### js/label/comment.js

**Prettier Similarity**: 100.00%


### js/label/empty_label.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/arrow.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/break-parent.js

**Prettier Similarity**: 100.00%


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


### js/last-argument-expansion/edge_case.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/empty-lines.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/empty-object.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/function-body-in-mode-break.js
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


### js/last-argument-expansion/function-expression-issue-2239.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/function-expression.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/number-only-array.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/object.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/overflow.js

**Prettier Similarity**: 100.00%


### js/line-suffix-boundary/boundary.js

**Prettier Similarity**: 100.00%


### js/line/windows.js

**Prettier Similarity**: 100.00%


### js/literal/number.js

**Prettier Similarity**: 100.00%


### js/logical_expressions/issue-7024.js

**Prettier Similarity**: 100.00%


### js/logical_expressions/logical_expression_operators.js

**Prettier Similarity**: 100.00%


### js/member/conditional.js

**Prettier Similarity**: 100.00%


### js/member/expand.js

**Prettier Similarity**: 100.00%


### js/member/logical.js

**Prettier Similarity**: 100.00%


### js/method-chain/13018.js

**Prettier Similarity**: 100.00%


### js/method-chain/bracket_0-1.js

**Prettier Similarity**: 100.00%


### js/method-chain/bracket_0.js

**Prettier Similarity**: 100.00%


### js/method-chain/break-last-call.js

**Prettier Similarity**: 100.00%


### js/method-chain/break-last-member.js

**Prettier Similarity**: 100.00%


### js/method-chain/break-multiple.js

**Prettier Similarity**: 100.00%


### js/method-chain/comment.js

**Prettier Similarity**: 100.00%


### js/method-chain/complex-args.js

**Prettier Similarity**: 100.00%


### js/method-chain/computed-merge.js

**Prettier Similarity**: 100.00%


### js/method-chain/computed.js

**Prettier Similarity**: 100.00%


### js/method-chain/conditional.js

**Prettier Similarity**: 100.00%


### js/method-chain/cypress.js

**Prettier Similarity**: 100.00%


### js/method-chain/d3.js

**Prettier Similarity**: 100.00%


### js/method-chain/first_long.js

**Prettier Similarity**: 100.00%


### js/method-chain/fluent-configuration.js

**Prettier Similarity**: 100.00%


### js/method-chain/inline_merge.js

**Prettier Similarity**: 100.00%


### js/method-chain/issue-11298.js

**Prettier Similarity**: 100.00%


### js/method-chain/issue-3594.js

**Prettier Similarity**: 100.00%


### js/method-chain/issue-3621.js

**Prettier Similarity**: 100.00%


### js/method-chain/issue-4125.js

**Prettier Similarity**: 100.00%


### js/method-chain/logical.js

**Prettier Similarity**: 100.00%


### js/method-chain/multiple-members.js

**Prettier Similarity**: 100.00%


### js/method-chain/object-literal.js

**Prettier Similarity**: 100.00%


### js/method-chain/pr-7889.js

**Prettier Similarity**: 100.00%


### js/method-chain/print-width-120/constructor.js

**Prettier Similarity**: 100.00%


### js/method-chain/print-width-120/issue-7884.js

**Prettier Similarity**: 100.00%


### js/method-chain/short-names.js

**Prettier Similarity**: 100.00%


### js/method-chain/simple-args.js

**Prettier Similarity**: 100.00%


### js/method-chain/square_0.js

**Prettier Similarity**: 100.00%


### js/method-chain/test.js

**Prettier Similarity**: 100.00%


### js/method-chain/this.js

**Prettier Similarity**: 100.00%


### js/module-string-names/module-string-names-export.js

**Prettier Similarity**: 100.00%


### js/module-string-names/module-string-names-import.js

**Prettier Similarity**: 100.00%


### js/new-expression/call.js

**Prettier Similarity**: 100.00%


### js/new-expression/new_expression.js

**Prettier Similarity**: 100.00%


### js/new-expression/with-member-expression.js

**Prettier Similarity**: 100.00%


### js/new-target/outside-functions.js

**Prettier Similarity**: 100.00%


### js/new-target/range.js

**Prettier Similarity**: 100.00%


### js/newline/backslash_2028.js

**Prettier Similarity**: 100.00%


### js/newline/backslash_2029.js

**Prettier Similarity**: 100.00%


### js/no-semi/class.js

**Prettier Similarity**: 100.00%


### js/no-semi/comments.js

**Prettier Similarity**: 100.00%


### js/no-semi/issue2006.js

**Prettier Similarity**: 100.00%


### js/no-semi/no-semi.js

**Prettier Similarity**: 100.00%


### js/non-strict/argument-name-clash.js

**Prettier Similarity**: 100.00%


### js/non-strict/keywords.js

**Prettier Similarity**: 100.00%


### js/non-strict/octal-number.js

**Prettier Similarity**: 100.00%


### js/numeric-separators/number.js

**Prettier Similarity**: 100.00%


### js/object-colon-bug/bug.js

**Prettier Similarity**: 100.00%


### js/object-prop-break-in/comment.js

**Prettier Similarity**: 100.00%


### js/object-prop-break-in/long-value.js

**Prettier Similarity**: 100.00%


### js/object-prop-break-in/short-keys.js

**Prettier Similarity**: 100.00%


### js/object-prop-break-in/test.js

**Prettier Similarity**: 100.00%


### js/object-property-comment/after-key.js

**Prettier Similarity**: 100.00%


### js/object-property-ignore/ignore.js

**Prettier Similarity**: 100.00%


### js/object-property-ignore/issue-5678.js

**Prettier Similarity**: 100.00%


### js/objects/assignment-expression/object-property.js
```diff
 a = {
-  [(this.resource = resource)]: 1,
+  [this.resource = resource]: 1,
 };

```

**Prettier Similarity**: 66.67%


### js/objects/assignment-expression/object-value.js

**Prettier Similarity**: 100.00%


### js/objects/escape-sequence-key.js

**Prettier Similarity**: 100.00%


### js/objects/expand.js

**Prettier Similarity**: 100.00%


### js/objects/getter-setter.js

**Prettier Similarity**: 100.00%


### js/objects/method.js

**Prettier Similarity**: 100.00%


### js/objects/range.js

**Prettier Similarity**: 100.00%


### js/objects/right-break.js

**Prettier Similarity**: 100.00%


### js/performance/nested-real.js

**Prettier Similarity**: 100.00%


### js/performance/nested.js

**Prettier Similarity**: 100.00%


### js/preserve-line/argument-list.js

**Prettier Similarity**: 100.00%


### js/preserve-line/comments.js

**Prettier Similarity**: 100.00%


### js/preserve-line/member-chain.js

**Prettier Similarity**: 100.00%


### js/preserve-line/parameter-list.js

**Prettier Similarity**: 100.00%


### js/quote-props/classes.js

**Prettier Similarity**: 100.00%


### js/quote-props/objects.js

**Prettier Similarity**: 100.00%


### js/quote-props/with_member_expressions.js

**Prettier Similarity**: 100.00%


### js/quote-props/with_numbers.js

**Prettier Similarity**: 100.00%


### js/quotes/functions.js

**Prettier Similarity**: 100.00%


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


### js/quotes/strings.js

**Prettier Similarity**: 100.00%


### js/range/array.js

**Prettier Similarity**: 100.00%


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
 
 
 class a {
   b() {}
 }
 
-let x;
+let    x

```

**Prettier Similarity**: 85.71%


### js/range/different-levels.js

**Prettier Similarity**: 100.00%


### js/range/directive.js

**Prettier Similarity**: 100.00%


### js/range/function-body.js

**Prettier Similarity**: 100.00%


### js/range/function-declaration.js

**Prettier Similarity**: 100.00%


### js/range/ignore-indentation.js

**Prettier Similarity**: 100.00%


### js/range/issue-3789-1.js

**Prettier Similarity**: 100.00%


### js/range/issue-3789-2.js

**Prettier Similarity**: 100.00%


### js/range/issue-4206-1.js

**Prettier Similarity**: 100.00%


### js/range/issue-4206-2.js

**Prettier Similarity**: 100.00%


### js/range/issue-4206-3.js

**Prettier Similarity**: 100.00%


### js/range/issue-4206-4.js

**Prettier Similarity**: 100.00%


### js/range/large-dict.js

**Prettier Similarity**: 100.00%


### js/range/module-export1.js

**Prettier Similarity**: 100.00%


### js/range/module-export2.js

**Prettier Similarity**: 100.00%


### js/range/module-export3.js

**Prettier Similarity**: 100.00%


### js/range/module-import.js

**Prettier Similarity**: 100.00%


### js/range/multiple-statements.js

**Prettier Similarity**: 100.00%


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


### js/range/nested-print-width.js

**Prettier Similarity**: 100.00%


### js/range/nested.js

**Prettier Similarity**: 100.00%


### js/range/nested2.js

**Prettier Similarity**: 100.00%


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


### js/range/object-expression.js

**Prettier Similarity**: 100.00%


### js/range/object-expression2.js

**Prettier Similarity**: 100.00%


### js/range/range-end.js

**Prettier Similarity**: 100.00%


### js/range/range-start.js

**Prettier Similarity**: 100.00%


### js/range/range.js

**Prettier Similarity**: 100.00%


### js/range/start-equals-end.js

**Prettier Similarity**: 100.00%


### js/range/try-catch.js

**Prettier Similarity**: 100.00%


### js/range/whitespace.js
```diff
- 

```

**Prettier Similarity**: 0.00%


### js/regex/multiple-flags.js

**Prettier Similarity**: 100.00%


### js/regex/regexp-modifiers.js

**Prettier Similarity**: 100.00%


### js/regex/test.js

**Prettier Similarity**: 100.00%


### js/require-amd/named-amd-module.js

**Prettier Similarity**: 100.00%


### js/require-amd/non-amd-define.js

**Prettier Similarity**: 100.00%


### js/require-amd/require.js

**Prettier Similarity**: 100.00%


### js/require/require.js

**Prettier Similarity**: 100.00%


### js/reserved-word/interfaces.js

**Prettier Similarity**: 100.00%


### js/rest/trailing-commas.js

**Prettier Similarity**: 100.00%


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


### js/return/binaryish.js

**Prettier Similarity**: 100.00%


### js/return/comment.js

**Prettier Similarity**: 100.00%


### js/sequence-break/break.js

**Prettier Similarity**: 100.00%


### js/sequence-expression/export-default.js

**Prettier Similarity**: 100.00%


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


### js/sequence-expression/parenthesized.js
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


### js/sloppy-mode/delete-variable.js

**Prettier Similarity**: 100.00%


### js/sloppy-mode/eval-arguments-binding.js

**Prettier Similarity**: 100.00%


### js/sloppy-mode/eval-arguments.js

**Prettier Similarity**: 100.00%


### js/sloppy-mode/function-declaration-in-if.js

**Prettier Similarity**: 100.00%


### js/sloppy-mode/function-declaration-in-while.js
```diff
-while (false) function foo() {}
+while (false) function foo(){}

```

**Prettier Similarity**: 0.00%


### js/sloppy-mode/labeled-function-declaration.js

**Prettier Similarity**: 100.00%


### js/strings/escaped.js
```diff
+// FIXME
+// TODO: reformat issue
 export const MSG_GENERIC_OPERATION_FAILURE_BODY_1 =
   goog.getMsg("That's all we know");
 
-export const MSG_GENERIC_OPERATION_FAILURE_BODY_2 =
-  goog.getMsg("That's all we know");
+// FIXME
+// TODO: reformat issue
+// export const MSG_GENERIC_OPERATION_FAILURE_BODY_2 =
+//   goog.getMsg("That\'s all we know");

```

**Prettier Similarity**: 33.33%


### js/strings/multiline-literal.js

**Prettier Similarity**: 100.00%


### js/strings/non-octal-eight-and-nine.js

**Prettier Similarity**: 100.00%


### js/strings/strings.js

**Prettier Similarity**: 100.00%


### js/strings/template-literals.js

**Prettier Similarity**: 100.00%


### js/switch/comments.js

**Prettier Similarity**: 100.00%


### js/switch/comments2.js
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


### js/switch/empty_lines.js

**Prettier Similarity**: 100.00%


### js/switch/empty_statement.js

**Prettier Similarity**: 100.00%


### js/switch/empty_switch.js

**Prettier Similarity**: 100.00%


### js/switch/switch.js

**Prettier Similarity**: 100.00%


### js/tab-width/class.js

**Prettier Similarity**: 100.00%


### js/tab-width/nested-functions.spec.js

**Prettier Similarity**: 100.00%


### js/template-align/indent.js

**Prettier Similarity**: 100.00%


### js/template-literals/binary-exporessions.js

**Prettier Similarity**: 100.00%


### js/template-literals/conditional-expressions.js

**Prettier Similarity**: 100.00%


### js/template-literals/expressions.js

**Prettier Similarity**: 100.00%


### js/template-literals/indention.js

**Prettier Similarity**: 100.00%


### js/template-literals/logical-expressions.js

**Prettier Similarity**: 100.00%


### js/template-literals/sequence-expressions.js

**Prettier Similarity**: 100.00%


### js/template/arrow.js

**Prettier Similarity**: 100.00%


### js/template/call.js

**Prettier Similarity**: 100.00%


### js/template/comment.js

**Prettier Similarity**: 100.00%


### js/template/faulty-locations.js

**Prettier Similarity**: 100.00%


### js/template/graphql.js

**Prettier Similarity**: 100.00%


### js/template/indent.js

**Prettier Similarity**: 100.00%


### js/template/inline.js

**Prettier Similarity**: 100.00%


### js/template/parenthesis.js

**Prettier Similarity**: 100.00%


### js/ternaries/binary.js

**Prettier Similarity**: 100.00%


### js/ternaries/func-call.js
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


### js/ternaries/indent-after-paren.js

**Prettier Similarity**: 100.00%


### js/ternaries/indent.js

**Prettier Similarity**: 100.00%


### js/ternaries/nested-in-condition.js

**Prettier Similarity**: 100.00%


### js/ternaries/nested.js

**Prettier Similarity**: 100.00%


### js/ternaries/parenthesis.js

**Prettier Similarity**: 100.00%


### js/ternaries/test.js

**Prettier Similarity**: 100.00%


### js/test-declarations/angular_async.js

**Prettier Similarity**: 100.00%


### js/test-declarations/angular_fakeAsync.js

**Prettier Similarity**: 100.00%


### js/test-declarations/angular_waitForAsync.js

**Prettier Similarity**: 100.00%


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


### js/test-declarations/jest-each-template-string.js

**Prettier Similarity**: 100.00%


### js/test-declarations/jest-each.js

**Prettier Similarity**: 100.00%


### js/test-declarations/test_declarations.js

**Prettier Similarity**: 100.00%


### js/throw_expressions/throw_expression.js
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
-        ? new UTF16Encoder(false)
-        : encoding === "utf16be"
-          ? new UTF16Encoder(true)
-          : throw new Error("Unsupported encoding");
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


### js/throw_statement/binaryish.js

**Prettier Similarity**: 100.00%


### js/throw_statement/comment.js

**Prettier Similarity**: 100.00%


### js/trailing-comma/dynamic-import.js

**Prettier Similarity**: 100.00%


### js/trailing-comma/es5.js

**Prettier Similarity**: 100.00%


### js/trailing-comma/object.js

**Prettier Similarity**: 100.00%


### js/trailing-comma/trailing_whitespace.js

**Prettier Similarity**: 100.00%


### js/try/catch.js

**Prettier Similarity**: 100.00%


### js/try/empty.js

**Prettier Similarity**: 100.00%


### js/try/try.js

**Prettier Similarity**: 100.00%


### js/unary-expression/comments.js

**Prettier Similarity**: 100.00%


### js/unary-expression/urnary_expression.js

**Prettier Similarity**: 100.00%


### js/unary/object.js

**Prettier Similarity**: 100.00%


### js/unary/series.js

**Prettier Similarity**: 100.00%


### js/unicode/combining-characters.js

**Prettier Similarity**: 100.00%


### js/unicode/keys.js

**Prettier Similarity**: 100.00%


### js/update-expression/update_expression.js

**Prettier Similarity**: 100.00%


### js/variable_declarator/multiple.js

**Prettier Similarity**: 100.00%


### js/variable_declarator/string.js

**Prettier Similarity**: 100.00%


### js/while/indent.js

**Prettier Similarity**: 100.00%


### js/with/indent.js

**Prettier Similarity**: 100.00%


### js/yield/arrow.js

**Prettier Similarity**: 100.00%


### js/yield/conditional.js

**Prettier Similarity**: 100.00%


